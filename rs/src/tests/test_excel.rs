use crate::{
    global_2007::traits::XmlDocumentPartCommon, log_elapsed, spreadsheet_2007::models::StyleSetting,
};
use anyhow::Context;
use chrono::Utc;
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
        Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string()
    )
}

#[test]
fn blank_excel() {
    let file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
        },
    )
    .expect("Create New File Failed");
    file.save_as(&get_save_file(None))
        .expect("File Save Failed");
    assert_eq!(true, true);
}

#[test]
fn excel_handling() {
    let mut file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
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
    let mut file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            ..crate::spreadsheet_2007::ExcelPropertiesModel::default()
        },
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
    let mut file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
    )
    .expect("Create New File Failed");
    file.minimize_workbook_mut(true)
        .expect("Failed to minimize workbook");
    file.save_as(&format!("{}", &get_save_file(Some("min"))))
        .expect("File Save Failed");
    file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
    )
    .expect("Create New File Failed");
    file.set_visibility_mut(false)
        .expect("Failed to add static Sheet");
    file.save_as(&format!("{}", &get_save_file(Some("hide"))))
        .expect("File Save Failed");
    file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
    )
    .expect("Create New File Failed");
    file.hide_horizontal_scroll_mut(true)
        .expect("Failed to add static Sheet");
    file.save_as(&format!("{}", &get_save_file(Some("show_hor_scroll"))))
        .expect("File Save Failed");
    file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
    )
    .expect("Create New File Failed");
    file.hide_vertical_scroll_mut(true)
        .expect("Failed to add static Sheet");
    file.save_as(&format!("{}", &get_save_file(Some("show_ver_scroll"))))
        .expect("File Save Failed");
    file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
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
    let mut file = crate::spreadsheet_2007::Excel::new(
        Some("src/tests/TestFiles/basic_test.xlsx".to_string()),
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
        },
    )
    .expect("Open Existing File Failed");
    let mut row_prop = file
        .add_sheet_mut(Some("row_property".to_string()))
        .expect("failed to add Sheet");
    row_prop
        .set_row_index_properties_mut(
            &1,
            crate::spreadsheet_2007::models::RowProperties {
                height: Some(100 as f32),
                ..crate::spreadsheet_2007::models::RowProperties::default()
            },
        )
        .expect("Failed to set row height");
    row_prop
        .set_row_index_properties_mut(
            &3,
            crate::spreadsheet_2007::models::RowProperties {
                hidden: Some(true),
                ..crate::spreadsheet_2007::models::RowProperties::default()
            },
        )
        .expect("Failed to set row height");
    row_prop
        .set_row_index_properties_mut(
            &5,
            crate::spreadsheet_2007::models::RowProperties {
                thick_bottom: Some(true),
                ..crate::spreadsheet_2007::models::RowProperties::default()
            },
        )
        .expect("Failed to set row height");
    row_prop
        .set_row_index_properties_mut(
            &7,
            crate::spreadsheet_2007::models::RowProperties {
                thick_top: Some(true),
                ..crate::spreadsheet_2007::models::RowProperties::default()
            },
        )
        .expect("Failed to set row height");
    row_prop.flush().expect("Failed to write Data");
    file.save_as(&get_save_file(None))
        .expect("Save File Failed");
    assert_eq!(true, true);
}

#[test]
fn merge_cell_property() {
    let mut file = crate::spreadsheet_2007::Excel::new(
        Some("src/tests/TestFiles/merge.xlsx".to_string()),
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
        },
    )
    .expect("Open Existing File Failed");
    {
        let mut worksheet = file
            .get_worksheet_mut("Sheet1".to_string())
            .expect("Failed to open the sheet");
        let result = worksheet.list_merge_cell_();
    }
    file.save_as(&get_save_file(None))
        .expect("Save File Failed");
    assert_eq!(true, true);
}

#[test]
fn set_column_property() {
    let mut file = crate::spreadsheet_2007::Excel::new(
        Some("src/tests/TestFiles/basic_test.xlsx".to_string()),
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
        },
    )
    .expect("Open Existing File Failed");
    let mut col_prop = file
        .add_sheet_mut(Some("col_property".to_string()))
        .expect("failed to add Sheet");
    col_prop
        .set_column_index_properties_mut(
            &1,
            Some(crate::spreadsheet_2007::models::ColumnProperties {
                width: Some(200 as f32),
                ..crate::spreadsheet_2007::models::ColumnProperties::default()
            }),
        )
        .expect("Failed to Set Column prop");
    col_prop
        .set_column_index_properties_mut(
            &3,
            Some(crate::spreadsheet_2007::models::ColumnProperties {
                hidden: Some(true),
                ..crate::spreadsheet_2007::models::ColumnProperties::default()
            }),
        )
        .expect("Failed to Set Column prop");
    col_prop
        .set_column_index_properties_mut(
            &5,
            Some(crate::spreadsheet_2007::models::ColumnProperties {
                best_fit: Some(true),
                ..crate::spreadsheet_2007::models::ColumnProperties::default()
            }),
        )
        .expect("Failed to Set Column prop");
    col_prop.flush().expect("Failed to write Data");
    file.save_as(&get_save_file(None))
        .expect("Save File Failed");
    assert_eq!(true, true);
}

#[test]
fn set_cell_style() {
    let mut file = crate::spreadsheet_2007::Excel::new(
        Some("src/tests/TestFiles/basic_test.xlsx".to_string()),
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
        },
    )
    .expect("Open Existing File Failed");
    let style_id = file
        .get_style_id_mut(crate::spreadsheet_2007::models::StyleSetting {
            ..crate::spreadsheet_2007::models::StyleSetting::default()
        })
        .expect("Failed to get Style Id");
    {
        let mut formula = file
            .get_worksheet_mut("formula".to_string())
            .expect("Failed to find the worksheet");
        formula
            .set_row_value_ref_mut(
                "V3",
                vec![
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Dravia".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Vemal".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("style".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        style_id: Some(style_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
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
    let mut file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
    )
    .expect("Open Existing File Failed");
    let bold_id = file
        .get_style_id_mut(StyleSetting {
            is_bold: true,
            ..StyleSetting::default()
        })
        .expect("Failed bold ID");
    let italic_id = file
        .get_style_id_mut(StyleSetting {
            is_italic: true,
            ..StyleSetting::default()
        })
        .expect("Failed italic ID");
    let underline_id = file
        .get_style_id_mut(StyleSetting {
            is_underline: true,
            ..StyleSetting::default()
        })
        .expect("Failed underline ID");
    let double_id = file
        .get_style_id_mut(StyleSetting {
            is_double_underline: true,
            ..StyleSetting::default()
        })
        .expect("Failed double underline ID");
    let wrap_text_id = file
        .get_style_id_mut(StyleSetting {
            is_wrap_text: true,
            ..StyleSetting::default()
        })
        .expect("Failed wrape ID");
    {
        let mut formula = file
            .add_sheet_mut(None)
            .expect("Failed to find the worksheet");
        formula
            .set_row_value_ref_mut(
                "V3",
                vec![
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Dravia".to_string()),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Vemal".to_string()),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Bold".to_string()),
                        style_id: Some(bold_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Italic".to_string()),
                        style_id: Some(italic_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("underline".to_string()),
                        style_id: Some(underline_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("double underline".to_string()),
                        style_id: Some(double_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some(
                            "This is a very long line to wrap the column. Test the wrap string"
                                .to_string(),
                        ),
                        style_id: Some(wrap_text_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                ],
            )
            .expect("Failed To Set Row Value");
    }
    file.save_as(&get_save_file(None))
        .expect("Save File Failed");
}

#[test]
fn edit_excel() {
    let mut file = crate::spreadsheet_2007::Excel::new(
        Some("src/tests/TestFiles/basic_test.xlsx".to_string()),
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
        },
    )
    .expect("Open Existing File Failed");
    let bold_id = file
        .get_style_id_mut(StyleSetting {
            is_bold: true,
            ..StyleSetting::default()
        })
        .expect("Failed bold ID");
    let italic_id = file
        .get_style_id_mut(StyleSetting {
            is_italic: true,
            ..StyleSetting::default()
        })
        .expect("Failed italic ID");
    let underline_id = file
        .get_style_id_mut(StyleSetting {
            is_underline: true,
            ..StyleSetting::default()
        })
        .expect("Failed underline ID");
    let double_id = file
        .get_style_id_mut(StyleSetting {
            is_double_underline: true,
            ..StyleSetting::default()
        })
        .expect("Failed double underline ID");
    let wrap_text_id = file
        .get_style_id_mut(StyleSetting {
            is_wrap_text: true,
            ..StyleSetting::default()
        })
        .expect("Failed wrape ID");
    {
        let mut formula = file
            .get_worksheet_mut("formula".to_string())
            .expect("Failed to find the worksheet");
        formula
            .set_row_value_ref_mut(
                "V3",
                vec![
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Dravia".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Vemal".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Bold".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        style_id: Some(bold_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Italic".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        style_id: Some(italic_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("underline".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        style_id: Some(underline_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("double underline".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        style_id: Some(double_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some(
                            "This is a very long line to wrap the column. Test the wrap string"
                                .to_string(),
                        ),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        style_id: Some(wrap_text_id),
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
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
    let mut file = crate::spreadsheet_2007::Excel::new(
        Some("src/tests/TestFiles/large_file.xlsx".to_string()),
        crate::spreadsheet_2007::ExcelPropertiesModel {
            is_in_memory: false,
            is_editable: true,
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
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Dravia".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
                    crate::spreadsheet_2007::models::CellProperties {
                        value: Some("Vemal".to_string()),
                        data_type: crate::spreadsheet_2007::models::CellDataType::Auto,
                        ..crate::spreadsheet_2007::models::CellProperties::default()
                    },
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
    let mut file = crate::spreadsheet_2007::Excel::new(
        None,
        crate::spreadsheet_2007::ExcelPropertiesModel::default(),
    )
    .expect("Open Existing File Failed");
    let mut range = rand::thread_rng();
    let cell_type = [
        crate::spreadsheet_2007::models::CellDataType::Auto,
        crate::spreadsheet_2007::models::CellDataType::InlineString,
        crate::spreadsheet_2007::models::CellDataType::Number,
        crate::spreadsheet_2007::models::CellDataType::ShareString,
        crate::spreadsheet_2007::models::CellDataType::String,
    ];
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
                                .map(|_| crate::spreadsheet_2007::models::CellProperties {
                                    value: Some("Test".to_string()),
                                    ..crate::spreadsheet_2007::models::CellProperties::default()
                                })
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
