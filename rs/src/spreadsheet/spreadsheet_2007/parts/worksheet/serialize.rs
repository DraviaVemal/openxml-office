use std::{cell::RefCell, collections::HashMap, rc::Rc};

use draviavemal_xml_rs::{XmlAttribute, XmlDocument};

use crate::{
    converters::ConverterUtil,
    element_dictionary::COMMON_TYPE_COLLECTION,
    global_2007::{parts::RelationsPart, traits::Enum},
    namespaces::RELATIONSHIP_OFFICE_DOC_NS,
    spreadsheet_2007::{models::CellDataType, WorkSheet},
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};

impl WorkSheet {
    pub(crate) fn serialize_dimension(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        fn set_default(xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
            let dimension_attribute = vec![XmlAttribute::new("ref".to_string(), "A1".to_string())];
            let root_id = xml_doc_mut.get_root_id();
            xml_doc_mut
                .append_child_element_mut(root_id, "dimension", Some(dimension_attribute))
                .context("draviavemal-openxml_office::Failed to Add Dimension node to worksheet")?;
            Ok(())
        }
        if let Some(sheet_data) = self.sheet_data.as_ref() {
            if let Some(first_item) = sheet_data.first_key_value() {
                let dimension_attribute = vec![XmlAttribute::new(
                    "ref".to_string(),
                    format!(
                        "{}{}:{}{}",
                        ConverterUtil::get_column_ref(self.dimension.start_col).context(
                            "draviavemal-openxml_office::Failed to convert dim col start"
                        )?,
                        first_item.0,
                        ConverterUtil::get_column_ref(self.dimension.end_col)
                            .context("draviavemal-openxml_office::Failed to convert dim col end")?,
                        if let Some(row_end) = sheet_data.last_key_value() {
                            row_end.0
                        } else {
                            first_item.0
                        }
                    ),
                )];
                let root_id = xml_doc_mut.get_root_id();
                xml_doc_mut
                    .append_child_element_mut(root_id, "dimension", Some(dimension_attribute))
                    .context(
                        "draviavemal-openxml_office::Failed to Add Dimension node to worksheet",
                    )?;
            } else {
                set_default(xml_doc_mut)?;
            }
        } else {
            set_default(xml_doc_mut)?;
        }
        Ok(())
    }

    pub(crate) fn serialize_cols(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        if let Some(mut column_collection) = self.column_collection.take() {
            if column_collection.len() > 0 {
                let root_id = xml_doc_mut.get_root_id();
                let cols_id = xml_doc_mut
                    .insert_child_element_after_last_tag_mut(root_id, "cols", "sheetFormatPr", None)
                    .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
                loop {
                    if let Some(item) = column_collection.pop_front() {
                        let mut attribute: Vec<XmlAttribute> = Vec::new();
                        attribute.push(XmlAttribute::new("min".to_string(), item.min.to_string()));
                        attribute.push(XmlAttribute::new("max".to_string(), item.max.to_string()));
                        if let Some(width) = item.width {
                            attribute.push(XmlAttribute::new(
                                "customWidth".to_string(),
                                "1".to_string(),
                            ));
                            attribute
                                .push(XmlAttribute::new("width".to_string(), width.to_string()));
                        }
                        if let Some(style_id) = item.style_id {
                            attribute.push(XmlAttribute::new(
                                "style".to_string(),
                                style_id.id.to_string(),
                            ));
                        }
                        if item.hidden.is_some() {
                            attribute
                                .push(XmlAttribute::new("hidden".to_string(), "1".to_string()));
                        }
                        if item.best_fit.is_some() {
                            attribute
                                .push(XmlAttribute::new("bestFit".to_string(), "1".to_string()));
                        }
                        xml_doc_mut
                            .append_child_element_mut(cols_id, "col", Some(attribute))
                            .context("draviavemal-openxml_office::Failed to insert col record")?;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn serialize_sheet_views(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        let root_id = xml_doc_mut.get_root_id();
        let sheet_views_id = xml_doc_mut
            .insert_child_element_after_last_tag_mut(root_id, "sheetViews", "dimension", None)
            .context("draviavemal-openxml_office::Failed to Insert Sheet Views Element")?;
        loop {
            if let Some(sheet_view) = self.sheet_views.view_collection.pop() {
                let mut attribute: HashMap<String, String> = HashMap::new();
                attribute.insert("workbookViewId".to_string(), sheet_view.workbook_view_id);
                if let Some(window_protection) = sheet_view.window_protection {
                    attribute.insert(
                        "windowProtection".to_string(),
                        ConverterUtil::bool_xml_flag(&window_protection),
                    );
                }
                if let Some(show_formula_bar) = sheet_view.show_formula_bar {
                    attribute.insert(
                        "showFormulas".to_string(),
                        ConverterUtil::bool_xml_flag(&show_formula_bar),
                    );
                }
                if let Some(show_grid_line) = sheet_view.show_grid_line {
                    attribute.insert(
                        "showGridLines".to_string(),
                        ConverterUtil::bool_xml_flag(&show_grid_line),
                    );
                }
                if let Some(show_row_col_header) = sheet_view.show_row_col_header {
                    attribute.insert(
                        "showRowColHeaders".to_string(),
                        ConverterUtil::bool_xml_flag(&show_row_col_header),
                    );
                }
                if let Some(show_zero) = sheet_view.show_zero {
                    attribute.insert(
                        "showZeros".to_string(),
                        ConverterUtil::bool_xml_flag(&show_zero),
                    );
                }
                if let Some(view_right_to_left) = sheet_view.view_right_to_left {
                    attribute.insert(
                        "rightToLeft".to_string(),
                        ConverterUtil::bool_xml_flag(&view_right_to_left),
                    );
                }
                if let Some(tab_selected) = sheet_view.tab_selected {
                    attribute.insert(
                        "tabSelected".to_string(),
                        ConverterUtil::bool_xml_flag(&tab_selected),
                    );
                }
                if let Some(show_ruler) = sheet_view.show_ruler {
                    attribute.insert(
                        "showRuler".to_string(),
                        ConverterUtil::bool_xml_flag(&show_ruler),
                    );
                }
                if let Some(show_white_space) = sheet_view.show_white_space {
                    attribute.insert(
                        "showWhiteSpace".to_string(),
                        ConverterUtil::bool_xml_flag(&show_white_space),
                    );
                }
                if let Some(show_outline_symbol) = sheet_view.show_outline_symbol {
                    attribute.insert(
                        "showOutlineSymbols".to_string(),
                        ConverterUtil::bool_xml_flag(&show_outline_symbol),
                    );
                }
                if let Some(default_grid_color) = sheet_view.default_grid_color {
                    attribute.insert(
                        "defaultGridColor".to_string(),
                        ConverterUtil::bool_xml_flag(&default_grid_color),
                    );
                }
                if let Some(top_left_cell) = sheet_view.top_left_cell {
                    attribute.insert("topLeftCell".to_string(), top_left_cell);
                }
                if let Some(view) = sheet_view.view {
                    attribute.insert("view".to_string(), view);
                }
                if let Some(zoom_scale) = sheet_view.zoom_scale {
                    attribute.insert("zoomScale".to_string(), zoom_scale.to_string());
                }
                if let Some(zoom_scale_normal) = sheet_view.zoom_scale_normal {
                    attribute.insert("zoomScaleNormal".to_string(), zoom_scale_normal.to_string());
                }
                if let Some(zoom_scale_sheet_layout) = sheet_view.zoom_scale_sheet_layout {
                    attribute.insert(
                        "zoomScaleSheetLayoutView".to_string(),
                        zoom_scale_sheet_layout.to_string(),
                    );
                }
                if let Some(zoom_scale_page_layout) = sheet_view.zoom_scale_page_layout {
                    attribute.insert(
                        "zoomScalePageLayoutView".to_string(),
                        zoom_scale_page_layout.to_string(),
                    );
                }
                xml_doc_mut
                    .append_child_element_mut(
                        sheet_views_id,
                        "sheetView",
                        Some(
                            attribute
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed to insert sheetView record")?;
            } else {
                break;
            }
        }
        Ok(())
    }

    pub(crate) fn serialize_sheet_data(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        if let Some(sheet_data) = self.sheet_data.take() {
            let root_id = xml_doc_mut.get_root_id();
            let sheet_data_id = xml_doc_mut
                .insert_child_element_after_last_tag_mut(root_id, "sheetData", "cols", None)
                .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
            for (row_index, db_row) in sheet_data {
                let mut row_attribute = HashMap::new();
                row_attribute.insert("r".to_string(), row_index.to_string());
                if let Some(row_span) = db_row.row_property.span {
                    row_attribute.insert("spans".to_string(), row_span);
                }
                if let Some(row_style_id) = db_row.row_property.style_id {
                    row_attribute.insert("customFormat".to_string(), "1".to_string());
                    row_attribute.insert("s".to_string(), row_style_id.id.to_string());
                }
                if let Some(row_height) = db_row.row_property.height {
                    row_attribute.insert("customHeight".to_string(), "1".to_string());
                    row_attribute.insert("ht".to_string(), row_height.to_string());
                }
                if db_row.row_property.hidden.is_some() {
                    row_attribute.insert("hidden".to_string(), "1".to_string());
                }
                if let Some(row_group_level) = db_row.row_property.group_level {
                    row_attribute.insert("outlineLevel".to_string(), row_group_level.to_string());
                }
                if db_row.row_property.collapsed.is_some() {
                    row_attribute.insert("collapsed".to_string(), "1".to_string());
                }
                if db_row.row_property.thick_top.is_some() {
                    row_attribute.insert("thickTop".to_string(), "1".to_string());
                }
                if db_row.row_property.thick_bottom.is_some() {
                    row_attribute.insert("thickBot".to_string(), "1".to_string());
                }
                if db_row.row_property.place_holder.is_some() {
                    row_attribute.insert("ph".to_string(), "1".to_string());
                }
                let row_element_id = xml_doc_mut
                    .append_child_element_mut(
                        sheet_data_id,
                        "row",
                        Some(
                            row_attribute
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed to insert row element")?;
                if let Some(cols) = db_row.cell_records {
                    for (col_index, cell_record) in cols {
                        // Create cell element
                        let mut cell_attribute = HashMap::new();
                        cell_attribute.insert(
                            "r".to_string(),
                            format!(
                                "{}{}",
                                ConverterUtil::get_column_ref(col_index).context(
                                    "draviavemal-openxml_office::Failed to get Char Id from Int"
                                )?,
                                row_index
                            ),
                        );
                        if let Some(cell_style_id) = cell_record.style_id {
                            cell_attribute.insert("s".to_string(), cell_style_id.id.to_string());
                        }
                        if cell_record.data_type != CellDataType::Number {
                            cell_attribute.insert(
                                "t".to_string(),
                                CellDataType::get_string(cell_record.data_type),
                            );
                        }
                        if let Some(cell_comment_id) = cell_record.comment_id {
                            cell_attribute.insert("cm".to_string(), cell_comment_id.to_string());
                        }
                        if let Some(cell_metadata) = cell_record.metadata {
                            cell_attribute.insert("vm".to_string(), cell_metadata.to_string());
                        }
                        if cell_record.place_holder.is_some() {
                            cell_attribute.insert("ph".to_string(), "1".to_string());
                        }
                        let cell_id = xml_doc_mut
                            .append_child_element_mut(
                                row_element_id,
                                "c",
                                Some(
                                    cell_attribute
                                        .into_iter()
                                        .map(|(k, v)| XmlAttribute::new(k, v))
                                        .collect(),
                                ),
                            )
                            .context("draviavemal-openxml_office::Failed to insert row element")?;
                        // Create cell's child element
                        match cell_record.data_type {
                            CellDataType::InlineString => {
                                let inline_string_id = xml_doc_mut
                                    .append_child_element_mut(cell_id, "is", None)
                                    .context("draviavemal-openxml_office::Failed to insert Inline string element")?;
                                let text_id = xml_doc_mut
                                    .append_child_element_mut(inline_string_id, "t", None)
                                    .context("draviavemal-openxml_office::Failed To insert Text Value to inline string")?;
                                xml_doc_mut
                                    .get_element_mut(text_id)
                                    .context("draviavemal-openxml_office::Failed to get text element")?
                                    .add_text_mut(&if let Some(value) = cell_record.value {
                                        value
                                    } else {
                                        "".to_string()
                                    })
                                    .context("draviavemal-openxml_office::Failed to add text to inline string")?;
                            }
                            _ => {
                                if let Some(formula) = cell_record.formula {
                                    let formula_id = xml_doc_mut
                                        .append_child_element_mut(cell_id, "f", None)
                                        .context("draviavemal-openxml_office::Failed to insert Inline string element")?;
                                    xml_doc_mut
                                        .get_element_mut(formula_id)
                                        .context("draviavemal-openxml_office::Failed to get formula element")?
                                        .add_text_mut(&formula)
                                        .context("draviavemal-openxml_office::Failed to add formula text")?;
                                }
                                if let Some(value) = cell_record.value {
                                    let value_id = xml_doc_mut
                                        .append_child_element_mut(cell_id, "v", None)
                                        .context("draviavemal-openxml_office::Failed to insert Inline string element")?;
                                    xml_doc_mut
                                        .get_element_mut(value_id)
                                        .context("draviavemal-openxml_office::Failed to get value element")?
                                        .add_text_mut(&value)
                                        .context("draviavemal-openxml_office::Failed to add value text")?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn serialize_merge_cells(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        if let Some(merge_cells) = self.merge_cells.take() {
            let root_id = xml_doc_mut.get_root_id();
            let merge_cells_id = xml_doc_mut
                .insert_child_element_after_last_tag_mut(root_id, "mergeCells", "sheetData", None)
                .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
            {
                let merge_cells_element = xml_doc_mut
                    .get_element_mut(merge_cells_id)
                    .context("draviavemal-openxml_office::Failed to get element")?;
                merge_cells_element
                    .add_attribute_mut("count", &merge_cells.len().to_string())
                    .context("draviavemal-openxml_office::Failed to set Merge Cells Attribute")?;
            }
            for merge_cell in merge_cells {
                let mut attribute = HashMap::new();
                if merge_cell.row_start == merge_cell.row_end
                    && merge_cell.column_start == merge_cell.column_end
                {
                    attribute.insert(
                        "ref".to_string(),
                        ConverterUtil::get_cell_ref(merge_cell.row_start, merge_cell.column_start)?,
                    );
                } else {
                    attribute.insert(
                        "ref".to_string(),
                        format!(
                            "{}:{}",
                            ConverterUtil::get_cell_ref(
                                merge_cell.row_start,
                                merge_cell.column_start
                            )?,
                            ConverterUtil::get_cell_ref(merge_cell.row_end, merge_cell.column_end)?,
                        ),
                    );
                }
                xml_doc_mut
                    .append_child_element_mut(
                        merge_cells_id,
                        "mergeCell",
                        Some(
                            attribute
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed to add MergeCell Node")?;
            }
        }
        Ok(())
    }

    pub(crate) fn serialize_hyperlinks(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
        relationship_part: Rc<RefCell<RelationsPart>>,
    ) -> AnyResult<(), AnyError> {
        if let Some(hyperlinks) = self.hyperlinks.take() {
            let root_id = xml_doc_mut.get_root_id();
            let hyperlinks_id = xml_doc_mut
                .insert_child_element_after_last_tag_mut(root_id, "hyperlinks", "mergeCells", None)
                .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
            for hyperlink in hyperlinks {
                let mut attributes = HashMap::new();
                if hyperlink.range.row_start == hyperlink.range.row_end
                    && hyperlink.range.column_start == hyperlink.range.column_end
                {
                    attributes.insert(
                        "ref".to_string(),
                        ConverterUtil::get_cell_ref(
                            hyperlink.range.row_start,
                            hyperlink.range.column_start,
                        )?,
                    );
                } else {
                    attributes.insert(
                        "ref".to_string(),
                        format!(
                            "{}:{}",
                            ConverterUtil::get_cell_ref(
                                hyperlink.range.row_start,
                                hyperlink.range.column_start
                            )?,
                            ConverterUtil::get_cell_ref(
                                hyperlink.range.row_end,
                                hyperlink.range.column_end
                            )?,
                        ),
                    );
                }
                if let Some(display_value) = hyperlink.display {
                    attributes.insert("display".to_string(), display_value);
                }
                // Insert Relationship link
                let mut relationship_id = None;
                if hyperlink.id.is_some() {
                    let content = COMMON_TYPE_COLLECTION
                        .get("hyperlink")
                        .context("Failed to read Common type collection")?;
                    relationship_id = Some(
                        relationship_part
                            .borrow_mut()
                            .set_new_relationship_mut(&content, hyperlink.link)
                            .context(
                                "draviavemal-openxml_office::Failed to Create Hyperlink Relationship",
                            )?,
                    );
                } else {
                    attributes.insert("location".to_string(), hyperlink.link);
                }
                let hyperlink_id = xml_doc_mut
                    .append_child_element_mut(
                        hyperlinks_id,
                        "hyperlink",
                        Some(
                            attributes
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed tp Add element")?;
                if let Some(r_id) = relationship_id {
                    xml_doc_mut
                        .get_element_mut(hyperlink_id)
                        .context("draviavemal-openxml_office::Failed to fetch hyperlink element")?
                        .add_attribute_ns_mut("id", &RELATIONSHIP_OFFICE_DOC_NS, &r_id)
                        .context(
                            "draviavemal-openxml_office::Failed to set hyperlink relationship id",
                        )?;
                }
            }
        }
        Ok(())
    }
}
