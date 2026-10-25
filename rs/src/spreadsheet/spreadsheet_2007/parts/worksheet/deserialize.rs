use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlDocument, XmlElementContentType};

use crate::{
    converters::ConverterUtil,
    global_2007::{parts::RelationsPart, traits::Enum},
    namespaces::RELATIONSHIP_OFFICE_DOC_NS,
    spreadsheet_2007::{
        models::{
            CellDataType, CellProperty, ColumnIndex, ColumnProperties, HyperLinks, ReferenceRange,
            RowProperties, StyleId,
        },
        worksheet::model::{Dimension, RowRecords, WorkSheetView, WorkSheetViews},
        WorkSheet,
    },
};

impl WorkSheet {
    pub(crate) fn deserialize_cols(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<Option<VecDeque<ColumnProperties>>, AnyError> {
        let root_id = xml_doc_mut.get_root_id();
        if let Some(cols_id) = xml_doc_mut
            .find_first_child(root_id, "cols")
            .context("draviavemal-openxml_office::Failed to find cols element")?
        {
            let child_ids: Vec<NodeId> = xml_doc_mut
                .get_element(cols_id)
                .context("draviavemal-openxml_office::Failed to get cols element")?
                .get_child_contents()
                .as_ref()
                .map(|contents| {
                    contents
                        .iter()
                        .filter_map(|content| match content {
                            XmlElementContentType::Element((id, _, _)) => Some(*id),
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            let mut column_collection = VecDeque::with_capacity(child_ids.len());
            for col_id in child_ids {
                let col = xml_doc_mut
                    .get_element(col_id)
                    .context("draviavemal-openxml_office::Failed to get col element")?;
                let mut column_properties = ColumnProperties::default();
                if let Some(min) = col
                    .get_attribute("min")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.min = min
                        .parse()
                        .context("draviavemal-openxml_office::Failed to parse min value")?;
                }
                if let Some(max) = col
                    .get_attribute("max")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.max = max
                        .parse()
                        .context("draviavemal-openxml_office::Failed to parse min value")?;
                }
                if let Some(best_fit) = col
                    .get_attribute("bestFit")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.best_fit = if best_fit == "1" { Some(true) } else { None }
                }
                if let Some(hidden) = col
                    .get_attribute("hidden")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.hidden = if hidden == "1" { Some(true) } else { None }
                }
                if let Some(style) = col
                    .get_attribute("style")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.style_id =
                        Some(StyleId::new(style.parse().context(
                            "draviavemal-openxml_office::Failed to parse style ID",
                        )?));
                }
                if let Some(outline_level) = col
                    .get_attribute("outlineLevel")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.group_level = outline_level
                        .parse()
                        .context("draviavemal-openxml_office::Failed to parse style ID")?;
                }
                if let Some(custom_width) = col
                    .get_attribute("customWidth")
                    .map(|attribute| attribute.get_value())
                {
                    if custom_width == "1" {
                        column_properties.width = Some(
                            col.get_attribute("width")
                                .map(|attribute| attribute.get_value())
                                .context("draviavemal-openxml_office::Failed to get custom width")?
                                .parse()
                                .context(
                                    "draviavemal-openxml_office::Failed to parse custom width",
                                )?,
                        );
                    }
                }
                if let Some(collapsed) = col
                    .get_attribute("collapsed")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.collapsed = if collapsed == "1" { Some(true) } else { None }
                }
                column_collection.push_back(column_properties);
            }
            xml_doc_mut
                .remove_element_mut(cols_id)
                .context("draviavemal-openxml_office::Failed to remove cols element")?;
            return Ok(Some(column_collection));
        }
        Ok(None)
    }

    /// DeSerializing Worksheet View
    pub(crate) fn deserialize_worksheet_views(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<WorkSheetViews, AnyError> {
        let mut worksheet_views = WorkSheetViews::default();
        let root_id = xml_doc_mut.get_root_id();
        if let Some(sheet_views_id) = xml_doc_mut
            .find_first_child(root_id, "sheetViews")
            .context("draviavemal-openxml_office::Failed to find sheetViews element")?
        {
            let child_records: Vec<(NodeId, String)> = xml_doc_mut
                .get_element(sheet_views_id)
                .context("draviavemal-openxml_office::Failed to get sheetViews element")?
                .get_child_contents()
                .as_ref()
                .map(|contents| {
                    contents
                        .iter()
                        .filter_map(|content| match content {
                            XmlElementContentType::Element((id, tag, _)) => {
                                Some((*id, tag.clone()))
                            }
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            for (element_id, element_tag) in child_records {
                // Validate element that are not accounted
                if element_tag != "sheetView" {
                    return Err(AnyError::msg(
                        "draviavemal-openxml_office::Failed to Process Sheet Views child",
                    ));
                }
                let sheet_view_element = xml_doc_mut
                    .get_element(element_id)
                    .context("draviavemal-openxml_office::Failed to get Sheet View Element")?;
                let mut worksheet_view = WorkSheetView::default();
                worksheet_view.workbook_view_id = sheet_view_element
                    .get_attribute("workbookViewId")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Mandatory attribute \"workbookViewId\" is missing from sheetView")?
                    .to_string();
                // Windows protection
                if let Some(window_protection) = sheet_view_element
                    .get_attribute("windowProtection")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.window_protection = Some(
                        ConverterUtil::normalize_bool_property_bool(window_protection),
                    );
                }
                // Show formula
                if let Some(show_formula_bar) = sheet_view_element
                    .get_attribute("showFormulas")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_formula_bar = Some(
                        ConverterUtil::normalize_bool_property_bool(show_formula_bar),
                    );
                }
                // Show Grid Line
                if let Some(show_grid_line) = sheet_view_element
                    .get_attribute("showGridLines")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_grid_line =
                        Some(ConverterUtil::normalize_bool_property_bool(show_grid_line));
                }
                // Show row column header
                if let Some(show_row_col_header) = sheet_view_element
                    .get_attribute("showRowColHeaders")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_row_col_header = Some(
                        ConverterUtil::normalize_bool_property_bool(show_row_col_header),
                    );
                }
                // Show Zero
                if let Some(show_zero) = sheet_view_element
                    .get_attribute("showZeros")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_zero =
                        Some(ConverterUtil::normalize_bool_property_bool(show_zero));
                }
                // Right to Left
                if let Some(view_right_to_left) = sheet_view_element
                    .get_attribute("rightToLeft")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.view_right_to_left = Some(
                        ConverterUtil::normalize_bool_property_bool(view_right_to_left),
                    );
                }
                // Tab Selected
                if let Some(tab_selected) = sheet_view_element
                    .get_attribute("tabSelected")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.tab_selected =
                        Some(ConverterUtil::normalize_bool_property_bool(tab_selected));
                }
                // show ruler
                if let Some(show_ruler) = sheet_view_element
                    .get_attribute("showRuler")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_ruler =
                        Some(ConverterUtil::normalize_bool_property_bool(show_ruler));
                }
                // show white space
                if let Some(show_white_space) = sheet_view_element
                    .get_attribute("showWhiteSpace")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_white_space = Some(
                        ConverterUtil::normalize_bool_property_bool(show_white_space),
                    );
                }
                // Show outlined Symbols
                if let Some(show_outline_symbol) = sheet_view_element
                    .get_attribute("showOutlineSymbols")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_outline_symbol = Some(
                        ConverterUtil::normalize_bool_property_bool(show_outline_symbol),
                    );
                }
                // Default Grid Color
                if let Some(default_grid_color) = sheet_view_element
                    .get_attribute("defaultGridColor")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.default_grid_color = Some(
                        ConverterUtil::normalize_bool_property_bool(default_grid_color),
                    );
                }
                // Top Left Cell
                if let Some(top_left_cell) = sheet_view_element
                    .get_attribute("topLeftCell")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.top_left_cell = Some(top_left_cell.to_string());
                }
                // View Setting
                if let Some(view) = sheet_view_element
                    .get_attribute("view")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.view = Some(view.to_string());
                }
                // Zoom Scale
                if let Some(zoom_scale) = sheet_view_element
                    .get_attribute("zoomScale")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale = Some(zoom_scale.parse().context(
                        "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                    )?);
                }
                // Zoom Scale Normal
                if let Some(zoom_scale_normal) = sheet_view_element
                    .get_attribute("zoomScaleNormal")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale_normal = Some(zoom_scale_normal.parse().context(
                        "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                    )?);
                }
                // Zoom Scale Sheet Layout View
                if let Some(zoom_scale_sheet_layout) = sheet_view_element
                    .get_attribute("zoomScaleSheetLayoutView")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale_sheet_layout =
                        Some(zoom_scale_sheet_layout.parse().context(
                            "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                        )?);
                }
                // Zoom Scale Page Layout View
                if let Some(zoom_scale_page_layout) = sheet_view_element
                    .get_attribute("zoomScalePageLayoutView")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale_page_layout =
                        Some(zoom_scale_page_layout.parse().context(
                            "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                        )?);
                }
                worksheet_views.view_collection.push(worksheet_view);
            }
            xml_doc_mut
                .remove_element_mut(sheet_views_id)
                .context("draviavemal-openxml_office::Failed to remove sheetViews element")?;
        }
        Ok(worksheet_views)
    }

    /// Deserialize Sheet Data
    pub(crate) fn deserialize_sheet_data(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(Option<BTreeMap<u32, RowRecords>>, Dimension), AnyError> {
        let mut dimension = Dimension::default();
        let root_id = xml_doc_mut.get_root_id();
        if let Some(sheet_data_id) = xml_doc_mut
            .find_first_child(root_id, "sheetData")
            .context("draviavemal-openxml_office::Failed to find sheetData element")?
        {
            let mut sheet_data_collection: BTreeMap<u32, RowRecords> = BTreeMap::new();
            let row_ids: Vec<NodeId> = xml_doc_mut
                .get_element(sheet_data_id)
                .context("draviavemal-openxml_office::Failed to get sheetData element")?
                .get_child_contents()
                .as_ref()
                .map(|contents| {
                    contents
                        .iter()
                        .filter_map(|content| match content {
                            XmlElementContentType::Element((id, _, _)) => Some(*id),
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            // Loop All rows of sheet data
            for row_id in row_ids {
                let mut row_record = RowProperties::default();
                let row_element = xml_doc_mut
                    .get_element(row_id)
                    .context("draviavemal-openxml_office::Failed to get row element")?;
                // Get Row Id
                let row_index: u32 = row_element
                    .get_attribute("r")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Missing mandatory row id attribute")?
                    .parse()
                    .context("draviavemal-openxml_office::Failed to parse row id")?;
                if let Some(row_span) = row_element
                    .get_attribute("spans")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.span = Some(row_span.to_string());
                }
                if let Some(style_id) = row_element
                    .get_attribute("s")
                    .map(|attribute| attribute.get_value())
                {
                    if let Some(custom_formant) = row_element
                        .get_attribute("customFormat")
                        .map(|attribute| attribute.get_value())
                    {
                        row_record.style_id = if custom_formant == "1" {
                            Some(StyleId::new(style_id.parse().context(
                                "draviavemal-openxml_office::Failed to parse the row style id",
                            )?))
                        } else {
                            None
                        };
                    }
                }
                if let Some(hidden) = row_element
                    .get_attribute("hidden")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.hidden = if hidden == "1" { Some(true) } else { None };
                }
                if let Some(height) = row_element
                    .get_attribute("ht")
                    .map(|attribute| attribute.get_value())
                {
                    if let Some(custom_height) = row_element
                        .get_attribute("customHeight")
                        .map(|attribute| attribute.get_value())
                    {
                        row_record.height = if custom_height == "1" {
                            Some(height.parse().context(
                                "draviavemal-openxml_office::Failed to parse the row height",
                            )?)
                        } else {
                            None
                        };
                    }
                }
                if let Some(row_group_level) = row_element
                    .get_attribute("outlineLevel")
                    .map(|attribute| attribute.get_value())
                {
                    let outline_level = row_group_level.parse().context(
                        "draviavemal-openxml_office::Failed to parse the row group level",
                    )?;
                    row_record.group_level = if outline_level > 0 {
                        Some(outline_level)
                    } else {
                        None
                    };
                }
                if let Some(collapsed) = row_element
                    .get_attribute("collapsed")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.collapsed = if collapsed == "1" { Some(true) } else { None };
                }
                if let Some(thick_top) = row_element
                    .get_attribute("thickTop")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.thick_top = if thick_top == "1" { Some(true) } else { None };
                }
                if let Some(thick_bottom) = row_element
                    .get_attribute("thickBot")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.thick_bottom = if thick_bottom == "1" {
                        Some(true)
                    } else {
                        None
                    };
                }
                if let Some(place_holder) = row_element
                    .get_attribute("ph")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.place_holder = if place_holder == "1" {
                        Some(true)
                    } else {
                        None
                    };
                }
                let mut cell_records: BTreeMap<ColumnIndex, CellProperty> = BTreeMap::new();
                let col_ids: Vec<NodeId> = xml_doc_mut
                    .get_element(row_id)
                    .context("draviavemal-openxml_office::Failed to get row element")?
                    .get_child_contents()
                    .as_ref()
                    .map(|contents| {
                        contents
                            .iter()
                            .filter_map(|content| match content {
                                XmlElementContentType::Element((id, _, _)) => Some(*id),
                                _ => None,
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                // Loop All Columns of row
                for col_id in col_ids {
                    let mut cell_record = CellProperty::default();
                    let col_element = xml_doc_mut
                        .get_element(col_id)
                        .context("draviavemal-openxml_office::Failed to get col element")?;
                    // Get Col Id
                    let col_index = ConverterUtil::get_column_index(
                        col_element
                            .get_attribute("r")
                            .map(|attribute| attribute.get_value())
                            .context(
                                "draviavemal-openxml_office::Missing mandatory col id attribute",
                            )?,
                    )
                    .context(
                        "draviavemal-openxml_office::Failed to Convert col worksheet initialize",
                    )?;
                    if let Some(style_id) = col_element
                        .get_attribute("s")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.style_id = Some(StyleId::new(style_id.parse().context(
                            "draviavemal-openxml_office::Failed to parse the col style id",
                        )?));
                    }
                    if let Some(cell_type) = col_element
                        .get_attribute("t")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.data_type = CellDataType::get_enum(cell_type);
                    } else {
                        cell_record.data_type = CellDataType::Number;
                    }
                    if let Some(comment_id) = col_element
                        .get_attribute("cm")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.comment_id = Some(comment_id.parse().context(
                            "draviavemal-openxml_office::Failed to parse the col comment id",
                        )?);
                    };
                    if let Some(value_meta_id) = col_element
                        .get_attribute("vm")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.metadata = Some(value_meta_id.parse().context(
                            "draviavemal-openxml_office::Failed to parse the col value meta id",
                        )?);
                    };
                    if let Some(place_holder) = col_element
                        .get_attribute("ph")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.place_holder = if place_holder == "1" {
                            Some(true)
                        } else {
                            None
                        };
                    };
                    let cell_child_records: Vec<(NodeId, String)> = xml_doc_mut
                        .get_element(col_id)
                        .context("draviavemal-openxml_office::Failed to get col element")?
                        .get_child_contents()
                        .as_ref()
                        .map(|contents| {
                            contents
                                .iter()
                                .filter_map(|content| match content {
                                    XmlElementContentType::Element((id, tag, _)) => {
                                        Some((*id, tag.clone()))
                                    }
                                    _ => None,
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    for (cell_child_id, cell_child_tag) in cell_child_records {
                        match cell_child_tag.as_str() {
                            "v" => {
                                cell_record.value =
                                    WorkSheet::get_element_text(xml_doc_mut, cell_child_id)?;
                            }
                            "f" => {
                                cell_record.formula =
                                    WorkSheet::get_element_text(xml_doc_mut, cell_child_id)?;
                            }
                            "is" => {
                                if let Some(text_id) = xml_doc_mut
                                    .find_first_child(cell_child_id, "t")
                                    .context("draviavemal-openxml_office::Failed to find inline string text element")?
                                {
                                    cell_record.value =
                                        WorkSheet::get_element_text(xml_doc_mut, text_id)?;
                                }
                            }
                            _ => {
                                return Err(AnyError::msg("draviavemal-openxml_office::Found un-know element cell child"));
                            }
                        }
                    }
                    dimension.start_col = min(dimension.start_col, col_index);
                    dimension.end_col = max(dimension.end_col, col_index);
                    cell_records.insert(col_index, cell_record);
                }
                sheet_data_collection.insert(
                    row_index,
                    RowRecords {
                        row_property: row_record,
                        cell_records: if cell_records.len() > 0 {
                            Some(cell_records)
                        } else {
                            None
                        },
                    },
                );
            }
            xml_doc_mut
                .remove_element_mut(sheet_data_id)
                .context("draviavemal-openxml_office::Failed to remove sheetData element")?;
            return Ok((Some(sheet_data_collection), dimension));
        }
        Ok((None, dimension))
    }

    /// Reads the first text content of an element, returning None if there is no text.
    fn get_element_text(
        xml_doc_mut: &XmlDocument,
        element_id: NodeId,
    ) -> AnyResult<Option<String>, AnyError> {
        Ok(xml_doc_mut
            .get_element(element_id)
            .context("draviavemal-openxml_office::Failed to get element for text read")?
            .get_child_contents()
            .as_ref()
            .and_then(|contents| {
                contents.iter().find_map(|content| match content {
                    XmlElementContentType::Text(text) => Some(text.clone()),
                    _ => None,
                })
            }))
    }

    /// Deserialize Merge Cell Collection
    pub(crate) fn deserialize_merge_cells(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<Option<Vec<ReferenceRange>>> {
        let root_id = xml_doc_mut.get_root_id();
        if let Some(merge_cells_id) = xml_doc_mut
            .find_first_child(root_id, "mergeCells")
            .context("draviavemal-openxml_office::Failed to find mergeCells element")?
        {
            let mut merge_cell_collection = Vec::new();
            let merge_cell_ids: Vec<NodeId> = xml_doc_mut
                .get_element(merge_cells_id)
                .context("draviavemal-openxml_office::Failed to get mergeCells element")?
                .get_child_contents()
                .as_ref()
                .map(|contents| {
                    contents
                        .iter()
                        .filter_map(|content| match content {
                            XmlElementContentType::Element((id, _, _)) => Some(*id),
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            for merge_cell_id in merge_cell_ids {
                let merge_cell_element = xml_doc_mut
                    .get_element(merge_cell_id)
                    .context("draviavemal-openxml_office::Failed to Get Element")?;
                let merge_range = merge_cell_element
                    .get_attribute("ref")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Failed to get merge ref")?;
                if merge_range.contains(':') {
                    let range: Vec<&str> = merge_range.split(':').collect();
                    let (row_start, column_start) = ConverterUtil::get_cell_index(range[0])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    let (row_end, column_end) = ConverterUtil::get_cell_index(range[1])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    merge_cell_collection.push(ReferenceRange {
                        column_start,
                        row_start,
                        column_end,
                        row_end,
                    });
                } else {
                    let (row, col) = ConverterUtil::get_cell_index(merge_range)
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    merge_cell_collection.push(ReferenceRange {
                        column_start: col,
                        row_start: row,
                        column_end: col,
                        row_end: row,
                    });
                }
            }
            xml_doc_mut
                .remove_element_mut(merge_cells_id)
                .context("draviavemal-openxml_office::Failed to remove mergeCells element")?;
            if merge_cell_collection.len() > 0 {
                Ok(Some(merge_cell_collection))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Deserialize Hyperlink Collection
    pub(crate) fn deserialize_hyperlinks(
        xml_doc_mut: &mut XmlDocument,
        relationship_part: &Rc<RefCell<RelationsPart>>,
    ) -> AnyResult<Option<Vec<HyperLinks>>> {
        let root_id = xml_doc_mut.get_root_id();
        if let Some(hyperlinks_id) = xml_doc_mut
            .find_first_child(root_id, "hyperlinks")
            .context("draviavemal-openxml_office::Failed to find hyperlinks element")?
        {
            let mut hyperlink_collection = Vec::new();
            let hyperlink_ids: Vec<NodeId> = xml_doc_mut
                .get_element(hyperlinks_id)
                .context("draviavemal-openxml_office::Failed to get hyperlinks element")?
                .get_child_contents()
                .as_ref()
                .map(|contents| {
                    contents
                        .iter()
                        .filter_map(|content| match content {
                            XmlElementContentType::Element((id, _, _)) => Some(*id),
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            for hyperlink_element_id in hyperlink_ids {
                let hyperlink_element = xml_doc_mut
                    .get_element(hyperlink_element_id)
                    .context("draviavemal-openxml_office::Failed to Get Element")?;
                let display = hyperlink_element
                    .get_attribute("display")
                    .map(|attribute| attribute.get_value().to_string());
                let hyperlink_ref = hyperlink_element
                    .get_attribute("ref")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Failed to get hyperlink ref")?;
                let hyperlink_id = hyperlink_element
                    .get_attribute_by_uri(RELATIONSHIP_OFFICE_DOC_NS.uri, "id")
                    .map(|attribute| attribute.get_value().to_string());
                let range_reference = if hyperlink_ref.contains(':') {
                    let range: Vec<&str> = hyperlink_ref.split(':').collect();
                    let (row_start, column_start) = ConverterUtil::get_cell_index(range[0])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    let (row_end, column_end) = ConverterUtil::get_cell_index(range[1])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    ReferenceRange {
                        column_start,
                        row_start,
                        column_end,
                        row_end,
                    }
                } else {
                    let (row, col) = ConverterUtil::get_cell_index(hyperlink_ref)
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    ReferenceRange {
                        column_start: col,
                        row_start: row,
                        column_end: col,
                        row_end: row,
                    }
                };
                // If relationship ID exist pull from relationship part
                let link = if let Some(id) = hyperlink_id.as_ref() {
                    let link = relationship_part
                        .borrow()
                        .get_target_by_id(id)
                        .context("draviavemal-openxml_office::Failed to Pull Target From Relationship file")?
                        .context("draviavemal-openxml_office::No Target Found in the relationship")?;
                    relationship_part
                        .borrow_mut()
                        .delete_relationship_by_id_mut(&id);
                    link
                } else {
                    hyperlink_element
                        .get_attribute("location")
                        .map(|attribute| attribute.get_value())
                        .context("draviavemal-openxml_office::Failed to Get Internal Location")?
                        .to_string()
                };
                hyperlink_collection.push(HyperLinks {
                    id: hyperlink_id,
                    display,
                    link,
                    range: range_reference,
                });
            }
            xml_doc_mut
                .remove_element_mut(hyperlinks_id)
                .context("draviavemal-openxml_office::Failed to remove hyperlinks element")?;
            if hyperlink_collection.len() > 0 {
                Ok(Some(hyperlink_collection))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
