use std::{cell::RefCell, collections::{BTreeMap, VecDeque}, rc::{Rc, Weak}};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::XmlDocument;

use crate::{
    converters::ConverterUtil, element_dictionary::EXCEL_TYPE_COLLECTION, files::OfficeDocument, global_2007::{parts::RelationsPart, traits::XmlDocumentPartInitializing}, log_elapsed, spreadsheet_2007::{DrawingPart, WorkSheet, models::{CellDataType, CellPackage, CellProperty, ColumnProperties, HyperLinks, ReferenceRange, RowIndex}, services::CommonServices, worksheet::model::{Dimension, RowRecords, WorkSheetViews}},
};

impl WorkSheet {
    /// Create New object for the group
    pub(crate) fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        sheet_collection: Weak<RefCell<Vec<(String, String, bool)>>>,
        workbook_relationship_part: Weak<RefCell<RelationsPart>>,
        common_service: Weak<RefCell<CommonServices>>,
        display_sheet_name: Option<String>,
    ) -> AnyResult<WorkSheet, AnyError> {
        let (file_path, display_sheet_name) = WorkSheet::get_sheet_file_name(
            display_sheet_name,
            &office_document,
            &sheet_collection,
            &workbook_relationship_part,
        )
        .context("draviavemal-openxml_office::Failed to pull worksheet file name")?;
        let xml_document = WorkSheet::get_xml_document(&office_document, &file_path)?;
        let sheet_relationship_part = Rc::new(RefCell::new(
            RelationsPart::new(
                office_document.clone(),
                &format!(
                    "{}/_rels/{}.rels",
                    &file_path[..file_path.rfind('/').unwrap()],
                    file_path.rsplit('/').next().unwrap()
                ),
            )
            .context(
                "draviavemal-openxml_office::Creating Relation ship part for workbook failed.",
            )?,
        ));
        let (column_collection, sheet_data, merge_cells, hyperlinks, sheet_views, dimension) = log_elapsed!(
            || {
                WorkSheet::initialize_worksheet(&xml_document, Rc::clone(&sheet_relationship_part))
                    .context("draviavemal-openxml_office::Failed to open Worksheet")
            },
            "Worksheet Initialize Time"
        )?;
        let drawing_part = Rc::new(RefCell::new(
            DrawingPart::new(
                office_document.clone(),
                Rc::downgrade(&sheet_relationship_part),
                common_service.clone(),
                &EXCEL_TYPE_COLLECTION,
            )
            .context(
                "draviavemal-openxml_office::Failed to create/load drawing part of the sheet",
            )?,
        ));
        Ok(WorkSheet {
            office_document,
            xml_document,
            common_service,
            workbook_relationship_part,
            sheet_relationship_part,
            dimension,
            sheet_views,
            sheet_collection,
            column_collection,
            sheet_data,
            merge_cells,
            hyperlinks,
            file_path: file_path.to_string(),
            display_sheet_name,
            drawing_part,
        })
    }

    fn initialize_worksheet(
        xml_document: &Weak<RefCell<XmlDocument>>,
        relationship_part: Rc<RefCell<RelationsPart>>,
    ) -> AnyResult<
        (
            Option<VecDeque<ColumnProperties>>,
            Option<BTreeMap<u32, RowRecords>>,
            // Merge Range
            Option<Vec<ReferenceRange>>,
            // Hyperlinks
            Option<Vec<HyperLinks>>,
            WorkSheetViews,
            Dimension,
        ),
        AnyError,
    > {
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to get XML doc handle")?;
            // unwrap dimension
            let dimension_root_id = xml_doc_mut.get_root_id();
            if let Some(dimension_ids) = xml_doc_mut
                .find_all_child(dimension_root_id, "dimension")
                .context("draviavemal-openxml_office::Failed to find dimension elements")?
            {
                for dimension_id in dimension_ids {
                    xml_doc_mut.remove_element_mut(dimension_id).context(
                        "draviavemal-openxml_office::Failed to remove dimension element",
                    )?;
                }
            }
            let worksheet_views = log_elapsed!(
                || {
                    WorkSheet::deserialize_worksheet_views(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed to deserialize Worksheet View")
                },
                "Worksheet View Deserialization"
            )?;
            // unwrap columns to local collection
            let column_collection = log_elapsed!(
                || {
                    WorkSheet::deserialize_cols(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed To Deserialize Cols")
                },
                "Column deserialize"
            )?;
            // unwrap sheet data into object
            let (sheet_data, dimension) = log_elapsed!(
                || {
                    WorkSheet::deserialize_sheet_data(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed To Deserialize Sheet Data")
                },
                "Sheet Data Deserialize"
            )?;
            let merge_cells = log_elapsed!(
                || {
                    WorkSheet::deserialize_merge_cells(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed To Deserialize Merge Cells")
                },
                "Merge Cell Deserialize"
            )?;
            let hyperlinks = log_elapsed!(
                || {
                    WorkSheet::deserialize_hyperlinks(&mut xml_doc_mut, &relationship_part)
                        .context("draviavemal-openxml_office::Failed To Deserialize hyperlinks")
                },
                "Hyperlink Deserialize"
            )?;
            Ok((
                column_collection,
                sheet_data,
                merge_cells,
                hyperlinks,
                worksheet_views,
                dimension,
            ))
        } else {
            Ok((
                None,
                None,
                None,
                None,
                WorkSheetViews::default(),
                Dimension::default(),
            ))
        }
    }

    fn get_sheet_file_name(
        display_sheet_name: Option<String>,
        office_document: &Weak<RefCell<OfficeDocument>>,
        sheet_collection: &Weak<RefCell<Vec<(String, String, bool)>>>,
        workbook_relationship_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<(String, String), AnyError> {
        let worksheet_content = EXCEL_TYPE_COLLECTION
            .get("worksheet")
            .context("Failed to read Excel type collection")?;
        if let Some(sheet_collection) = sheet_collection.upgrade() {
            if let Some(workbook_relationship_part) = workbook_relationship_part.upgrade() {
                if let Some(sheet_name) = display_sheet_name.clone() {
                    // If the Sheet name already exist get the path of sheet name
                    if let Some((_, rel_id, _)) = sheet_collection
                        .try_borrow()
                        .context("draviavemal-openxml_office::Failed to Get Sheet Collection")?
                        .iter()
                        .find(|item| item.0 == sheet_name)
                    {
                        return Ok((
                            workbook_relationship_part
                                .try_borrow()
                                .context("draviavemal-openxml_office::Failed to Get Workbook relationship")?
                                .get_target_path_by_id(&rel_id)
                                .context("draviavemal-openxml_office::Failed to Get Target Path")?
                                .context("draviavemal-openxml_office::Failed to Get Relationship path")?,
                            sheet_name,
                        ));
                    }
                }
                let relative_path = workbook_relationship_part
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                    .get_relative_path()
                    .context("draviavemal-openxml_office::Get Relative Path for Part File")?;
                let file_path = format!("{}{}", relative_path, worksheet_content.default_path);
                let file_number = if let Some(office_doc) = office_document.upgrade() {
                    office_doc
                        .try_borrow()
                        .context("draviavemal-openxml_office::Failed to Borrow Document")?
                        .get_next_part_number(
                            &file_path,
                            worksheet_content.default_name,
                            worksheet_content.extension,
                        )
                } else {
                    sheet_collection
                        .try_borrow()
                        .context(
                            "draviavemal-openxml_office::Failed to pull Sheet Name Collection",
                        )?
                        .len()
                        + 1
                };
                let sheet_name = format!("{}{}", worksheet_content.default_name, &file_number);
                let relationship_id = workbook_relationship_part
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to Get Relationship Handle")?
                    .set_new_relationship_path_mut(
                        worksheet_content,
                        Some(file_path.clone()),
                        Some(sheet_name.clone()),
                    )
                    .context(
                        "draviavemal-openxml_office::Setting New Worksheet Relationship Failed.",
                    )?;
                sheet_collection
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed To pull Sheet Collection Handle")?
                    .push((
                        display_sheet_name.clone().unwrap_or(sheet_name.clone()),
                        relationship_id,
                        false,
                    ));
                return Ok((
                    format!(
                        "{}/{}.{}",
                        file_path, &sheet_name, worksheet_content.extension
                    ),
                    display_sheet_name.clone().unwrap_or(sheet_name.clone()),
                ));
            }
        }
        Err(AnyError::msg(
            "draviavemal-openxml_office::Failed to upgrade relation part",
        ))
    }

    pub(crate) fn update_share_string(
        &mut self,
        cell_value: &String,
    ) -> AnyResult<String, AnyError> {
        if let Some(common_service) = self.common_service.upgrade() {
            common_service
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Get Share String Handle")?
                .get_string_id_mut(cell_value.to_owned())
                .context("draviavemal-openxml_office::Failed to get share string id")
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to update Share String Record",
            ))
        }
    }

    pub(crate) fn extract_cell_records(
        &self,
        range: &ReferenceRange,
        data_range: &mut Vec<CellPackage>,
        row_index: &RowIndex,
        row_records: &RowRecords,
    ) -> Result<(), AnyError> {
        if let Some(cols) = row_records.cell_records.as_ref() {
            if range.column_end == 0 {
                for (column_index, cell_property) in cols.range(range.column_start..) {
                    data_range.push(CellPackage {
                        cell_ref: ConverterUtil::get_cell_ref(
                            row_index.clone(),
                            column_index.clone(),
                        )
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?,
                        row_index: row_index.clone(),
                        column_index: column_index.clone(),
                        cell_property: self.normalize_cell_property(cell_property)?,
                    });
                }
            } else {
                for (column_index, cell_property) in
                    cols.range(range.column_start..=range.column_end)
                {
                    data_range.push(CellPackage {
                        cell_ref: ConverterUtil::get_cell_ref(
                            row_index.clone(),
                            column_index.clone(),
                        )
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?,
                        row_index: row_index.clone(),
                        column_index: column_index.clone(),
                        cell_property: self.normalize_cell_property(cell_property)?,
                    });
                }
            }
        }
        Ok(())
    }

    fn normalize_cell_property(&self, cell_prop: &CellProperty) -> Result<CellProperty, AnyError> {
        let mut parsed_property = cell_prop.clone();
        if parsed_property.data_type == CellDataType::ShareString {
            if let Some(cmn_service) = self.common_service.upgrade() {
                let actual_value = cmn_service
                    .borrow()
                    .get_string_id_value(parsed_property.value.context("Failed to get cell value")?)
                    .context("draviavemal-openxml_office::Failed to normalize share string")?;
                parsed_property.value = Some(actual_value);
            }
        }
        Ok(parsed_property)
    }
}
