#[cfg(test)]
mod spreadsheet_test {
    use chrono::Utc;
    use draviavemal_openxml_office::{
        global_2007::traits::XmlDocumentPartClose,
        log_elapsed,
        spreadsheet_2007::models::{CellProperties, ReferenceRange, StyleSetting},
    };
    use std::fs::{create_dir, exists};

    fn get_save_file(dynamic_path: Option<&str>) -> String {
        let result_path = "test_results";
        if !exists(result_path).expect("Dir Check Failed") {
            create_dir(result_path).expect("Failed to Create")
        }
        format!(
            "{}/test-{}{}.xlsx",
            result_path,
            dynamic_path.unwrap_or(""),
            Utc::now().format("%Y-%m-%d-%H-%M-%S-%3f").to_string()
        )
    }

    #[test]
    fn blank_excel() {
        let file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Create New File Failed");
        file.save_as(&get_save_file(None))
            .expect("File Save Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn excel_handling() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.add_sheet_mut(Some("Test".to_string()))
            .expect("Failed to add static Sheet");
        file.add_sheet_mut(Some("bust".to_string()))
            .expect("Failed to add static Sheet");
        file.add_sheet_mut(None)
            .expect("Failed to add static Sheet");
        file.add_sheet_mut(Some("Active".to_string()))
            .expect("Failed to add static Sheet");
        file.set_active_sheet_mut("Active".to_string())
            .expect("Failed To Set Active Sheet");
        file.add_sheet_mut(Some("hideSheet".to_string()))
            .expect("Failed to add static Sheet");
        file.hide_sheet_mut("hideSheet".to_string())
            .expect("Failed to hide the sheet");
        file.save_as(&get_save_file(None))
            .expect("File Save Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn sheet_handling() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.add_sheet_mut(Some("Test".to_string()))
            .expect("Failed to add static Sheet");
        file.add_sheet_mut(Some("bust".to_string()))
            .expect("Failed to add static Sheet");
        file.add_sheet_mut(Some("RenameThisSheet".to_string()))
            .expect("Failed to add static Sheet");
        file.add_sheet_mut(None)
            .expect("Failed to add dynamic Sheet");
        {
            let close_sheet = file
                .add_sheet_mut(Some("deleteThis".to_string()))
                .expect("Failed to add static Sheet");
            close_sheet.flush().expect("Failed to Close Work Sheet");
        }
        {
            let delete_sheet = file
                .get_worksheet_mut("deleteThis".to_string())
                .expect("Failed to Get the Worksheet");
            delete_sheet
                .delete_sheet_mut()
                .expect("Failed to Delete Sheet");
        }
        file.rename_sheet_name_mut("RenameThisSheet".to_string(), "RenamedSheet".to_string())
            .expect("Failed to rename the sheet");
        file.save_as(&get_save_file(None))
            .expect("File Save Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn excel_workbook_view() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.minimize_workbook_mut(true)
            .expect("Failed to minimize workbook");
        file.save_as(&format!("{}", &get_save_file(Some("min"))))
            .expect("File Save Failed");
        file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.set_visibility_mut(false)
            .expect("Failed to add static Sheet");
        file.save_as(&format!("{}", &get_save_file(Some("hide"))))
            .expect("File Save Failed");
        file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.hide_horizontal_scroll_mut(true)
            .expect("Failed to add static Sheet");
        file.save_as(&format!("{}", &get_save_file(Some("show_hor_scroll"))))
            .expect("File Save Failed");
        file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.hide_vertical_scroll_mut(true)
            .expect("Failed to add static Sheet");
        file.save_as(&format!("{}", &get_save_file(Some("show_ver_scroll"))))
            .expect("File Save Failed");
        file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Create New File Failed");
        file.hide_sheet_tabs_mut(true)
            .expect("Failed to add static Sheet");
        file.save_as(&format!("{}", &get_save_file(Some("show_tab"))))
            .expect("File Save Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn set_row_property() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/basic_test.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        let mut row_prop = file
            .add_sheet_mut(Some("row_property".to_string()))
            .expect("failed to add Sheet");
        row_prop
            .set_row_index_properties_mut(
                &1,
                draviavemal_openxml_office::spreadsheet_2007::models::RowProperties::default()
                    .set_height(Some(100 as f32)),
            )
            .expect("Failed to set row height");
        row_prop
            .set_row_index_properties_mut(
                &3,
                draviavemal_openxml_office::spreadsheet_2007::models::RowProperties::default()
                    .set_hidden(Some(true)),
            )
            .expect("Failed to set row height");
        row_prop
            .set_row_index_properties_mut(
                &5,
                draviavemal_openxml_office::spreadsheet_2007::models::RowProperties::default()
                    .set_thick_top(Some(true)),
            )
            .expect("Failed to set thick top");
        row_prop
            .set_row_index_properties_mut(
                &7,
                draviavemal_openxml_office::spreadsheet_2007::models::RowProperties::default()
                    .set_thick_bottom(Some(true)),
            )
            .expect("Failed to set thick bottom");

        row_prop.flush().expect("Failed to write Data");
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn merge_cell_property() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/merge_links.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        {
            let worksheet = file
                .get_worksheet_mut("Sheet1".to_string())
                .expect("Failed to open the sheet");
            let result = worksheet.list_merge_cell_();
            assert!(result.is_some())
        }
        {
            let mut edit_worksheet = file
                .get_worksheet_mut("edit".to_string())
                .expect("Failed to open the sheet");
            let ranges = edit_worksheet.list_merge_cell_();
            if let Some(ranges) = ranges {
                edit_worksheet
                    .remove_merge_cell_mut(ranges[2].clone())
                    .expect("Failed to Remove Merge Range");
            }
            edit_worksheet
                .set_merge_cell_mut(
                    draviavemal_openxml_office::spreadsheet_2007::models::ReferenceRange {
                        column_start: 1,
                        column_end: 1,
                        row_start: 1,
                        row_end: 10,
                    },
                )
                .expect("Failed to Insert Merge Range");
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn hyperlink_cell_property() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/merge_links.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        {
            let worksheet = file
                .get_worksheet_mut("edit".to_string())
                .expect("Failed to open the sheet");
            let result = worksheet.list_hyperlinks();
            assert!(result.is_some())
        }
        {
            let mut edit_worksheet = file
                .get_worksheet_mut("edit".to_string())
                .expect("Failed to open the sheet");
            let ranges = edit_worksheet.list_hyperlinks();
            if let Some(ranges) = ranges {
                edit_worksheet
                    .remove_hyperlink_mut(ranges[2].2.clone())
                    .expect("Failed to Remove Merge Range");
            }
        }
        {
            let mut edit_worksheet = file
                .get_worksheet_mut("edit".to_string())
                .expect("Failed to open the sheet");
            edit_worksheet
                .set_hyperlink_mut(
                    Some("Test".to_string()),
                    "https://www.draviavemal.com".to_string(),
                    ReferenceRange {
                        row_start: 10,
                        row_end: 10,
                        column_start: 10,
                        column_end: 10,
                    },
                )
                .expect("Failed to Insert Hyperlink")
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn get_range_data() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/basic_test.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        let worksheet = file
            .get_worksheet_mut("formula".to_string())
            .expect("Failed to get worksheet");
        let data = worksheet
            .get_range_cell_properties(ReferenceRange {
                column_start: 1,
                column_end: 0,
                row_start: 3,
                row_end: 10,
            })
            .expect("Failed to get range data");
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert!(true);
    }

    #[test]
    fn set_column_property_new() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        let mut col_prop = file
            .add_sheet_mut(Some("col_property".to_string()))
            .expect("failed to add Sheet");
        col_prop
        .set_column_index_properties_mut(
            &1,
            Some(
                draviavemal_openxml_office::spreadsheet_2007::models::ColumnProperties::default()
                    .set_width(Some(200 as f32)),
            ),
        )
        .expect("Failed to Set width Column prop");
        col_prop
        .set_column_index_properties_mut(
            &3,
            Some(
                draviavemal_openxml_office::spreadsheet_2007::models::ColumnProperties::default()
                    .set_hidden(Some(true)),
            ),
        )
        .expect("Failed to Set hidden prop");
        col_prop
        .set_column_index_properties_mut(
            &5,
            Some(
                draviavemal_openxml_office::spreadsheet_2007::models::ColumnProperties::default()
                    .set_best_fit(Some(true)),
            ),
        )
        .expect("Failed to Set best fit Column prop");
        col_prop
            .set_row_value_index_mut(
                2,
                1,
                vec![
                    CellProperties::default().set_value(Some("Cell Value 1".to_string())),
                    CellProperties::default().set_value(Some("Cell Value 2".to_string())),
                    CellProperties::default().set_value(Some("Cell Value 3".to_string())),
                    CellProperties::default().set_value(Some("Cell Value 4".to_string())),
                    CellProperties::default().set_value(Some("Cell Value 5".to_string())),
                    CellProperties::default().set_value(Some("Cell Value 6".to_string())),
                    CellProperties::default().set_value(Some("Cell Value 7".to_string())),
                ],
            )
            .expect("Failed to set Column Value");
        col_prop.flush().expect("Failed to write Data");
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn set_column_property_edit() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/basic_test.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        let mut col_prop = file
            .add_sheet_mut(Some("col_property".to_string()))
            .expect("failed to add Sheet");
        col_prop
        .set_column_index_properties_mut(
            &1,
            Some(
                draviavemal_openxml_office::spreadsheet_2007::models::ColumnProperties::default()
                    .set_width(Some(200 as f32)),
            ),
        )
        .expect("Failed to Set width Column prop");
        col_prop
        .set_column_index_properties_mut(
            &3,
            Some(
                draviavemal_openxml_office::spreadsheet_2007::models::ColumnProperties::default()
                    .set_hidden(Some(true)),
            ),
        )
        .expect("Failed to Set hidden prop");
        col_prop
        .set_column_index_properties_mut(
            &5,
            Some(
                draviavemal_openxml_office::spreadsheet_2007::models::ColumnProperties::default()
                    .set_best_fit(Some(true)),
            ),
        )
        .expect("Failed to Set best fit Column prop");
        col_prop.flush().expect("Failed to write Data");
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn set_cell_style() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/basic_test.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        let style_id = file
            .get_style_id_mut(
                draviavemal_openxml_office::spreadsheet_2007::models::StyleSetting::default(),
            )
            .expect("Failed to get Style Id");
        {
            let mut formula = file
                .get_worksheet_mut("formula".to_string())
                .expect("Failed to find the worksheet");
            formula
                .set_row_value_ref_mut(
                    "V3",
                    vec![
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Dravia".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Vemal".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Vemal".to_string()))
                        .set_style_id(Some(style_id)),
                ],
                )
                .expect("Failed To Set Row Value");
        }
        {
            file.get_worksheet_mut("Style".to_string())
                .expect("Failed to find the worksheet");
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn blank_style_excel() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Open Existing File Failed");
        let bold_id = file
            .get_style_id_mut(StyleSetting::default().set_is_bold(true))
            .expect("Failed bold ID");
        let italic_id = file
            .get_style_id_mut(StyleSetting::default().set_is_italic(true))
            .expect("Failed italic ID");
        let underline_id = file
            .get_style_id_mut(StyleSetting::default().set_is_underline(true))
            .expect("Failed underline ID");
        let double_id = file
            .get_style_id_mut(StyleSetting::default().set_is_double_underline(true))
            .expect("Failed double underline ID");
        let wrap_text_id = file
            .get_style_id_mut(StyleSetting::default().set_is_wrap_text(true))
            .expect("Failed wrape ID");
        {
            let mut formula = file
                .add_sheet_mut(None)
                .expect("Failed to find the worksheet");
            formula
                .set_row_value_ref_mut(
                    "V3",
                    vec![
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Dravia".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Vemal".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Bold".to_string()))
                        .set_style_id(Some(bold_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Italic".to_string()))
                        .set_style_id(Some(italic_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("underline".to_string()))
                        .set_style_id(Some(underline_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("double underline".to_string()))
                        .set_style_id(Some(double_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some(
                            "This is a very long line to wrap the column. Test the wrap string"
                                .to_string(),
                        ))
                        .set_style_id(Some(wrap_text_id)),
                ],
                )
                .expect("Failed To Set Row Value");
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
    }

    #[test]
    fn edit_excel() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/basic_test.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        let bold_id = file
            .get_style_id_mut(StyleSetting::default().set_is_bold(true))
            .expect("Failed bold ID");
        let italic_id = file
            .get_style_id_mut(StyleSetting::default().set_is_italic(true))
            .expect("Failed italic ID");
        let underline_id = file
            .get_style_id_mut(StyleSetting::default().set_is_underline(true))
            .expect("Failed underline ID");
        let double_id = file
            .get_style_id_mut(StyleSetting::default().set_is_double_underline(true))
            .expect("Failed double underline ID");
        let wrap_text_id = file
            .get_style_id_mut(StyleSetting::default().set_is_wrap_text(true))
            .expect("Failed wrape ID");
        {
            let mut formula = file
                .get_worksheet_mut("formula".to_string())
                .expect("Failed to find the worksheet");
            formula
                .set_row_value_ref_mut(
                    "V3",
                    vec![
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Dravia".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Vemal".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Bold".to_string()))
                        .set_style_id(Some(bold_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Italic".to_string()))
                        .set_style_id(Some(italic_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("underline".to_string()))
                        .set_style_id(Some(underline_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("double underline".to_string()))
                        .set_style_id(Some(double_id)),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some(
                            "This is a very long line to wrap the column. Test the wrap string"
                                .to_string(),
                        ))
                        .set_style_id(Some(wrap_text_id)),
                ],
                )
                .expect("Failed To Set Row Value");
        }
        {
            file.get_worksheet_mut("Style".to_string())
                .expect("Failed to find the worksheet");
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    #[ignore]
    fn edit_large_excel() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            Some("src/TestFiles/large_file.xlsx".to_string()),
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel {
                is_editable: true,
                ..draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default()
            },
        )
        .expect("Open Existing File Failed");
        {
            let mut sheet = file
                .get_worksheet_mut("Sheet1".to_string())
                .expect("Failed to find the worksheet");
            sheet
                .set_row_value_ref_mut(
                    "V3",
                    vec![
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Dravia".to_string())),
                    draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default()
                        .set_value(Some("Vemal".to_string())),
                ],
                )
                .expect("Failed To Set Row Value");
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }

    #[test]
    #[ignore]
    fn large_excel() {
        let mut file = draviavemal_openxml_office::spreadsheet_2007::Excel::new(
            None,
            draviavemal_openxml_office::spreadsheet_2007::ExcelPropertiesModel::default(),
        )
        .expect("Open Existing File Failed");
        {
            let mut sheet = file
                .add_sheet_mut(None)
                .expect("Failed to find the worksheet");
            log_elapsed!(
                || {
                    for row in 1..100_000 {
                        sheet
                        .set_row_value_index_mut(
                            row,
                            1,
                            (1..10)
                                .map(|_| draviavemal_openxml_office::spreadsheet_2007::models::CellProperties::default().set_value(Some("Test".to_string())))
                                .collect(),
                        )
                        .expect("Failed to Set Row Value");
                    }
                },
                "Insert Record Time"
            );
        }
        file.save_as(&get_save_file(None))
            .expect("Save File Failed");
        assert_eq!(true, true);
    }
}
