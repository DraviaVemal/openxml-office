use std::{ffi::c_char, mem::ManuallyDrop};

use crate::{
    openxml_office_fbs::{
        global::global_image_type,
        spreadsheet::{
            worksheet_add_picture, worksheet_cell_data_type, worksheet_cell_package,
            worksheet_cell_packageArgs, worksheet_cell_property as fbs_cell_property,
            worksheet_cell_propertyArgs, worksheet_delete_sheet, worksheet_excel_hyperlink_type,
            worksheet_flush, worksheet_get_range_cell_properties,
            worksheet_get_range_cell_properties_return,
            worksheet_get_range_cell_properties_returnArgs, worksheet_hyperlink as fbs_hyperlink,
            worksheet_hyperlinkArgs, worksheet_list_hyperlinks, worksheet_list_hyperlinks_return,
            worksheet_list_hyperlinks_returnArgs, worksheet_list_merge_cell,
            worksheet_list_merge_cell_return, worksheet_list_merge_cell_returnArgs,
            worksheet_reference_range as fbs_reference_range, worksheet_reference_rangeArgs,
            worksheet_remove_hyperlink, worksheet_remove_merge_cell,
            worksheet_set_cell_index_value, worksheet_set_cell_ref_value,
            worksheet_set_column_index_properties, worksheet_set_column_ref_properties,
            worksheet_set_hyperlink, worksheet_set_merge_cell, worksheet_set_row_index_properties,
        },
    },
    root_from_raw, set_error, write_buffer, StatusCode,
};
use anyhow::Error as AnyError;
use draviavemal_openxml_office::{
    global_2007::{
        models::{AnchorPosition, PictureSetting},
        traits::XmlDocumentPartFlush,
    },
    spreadsheet_2007::{
        models::{
            CellDataType, CellProperty, ColumnProperties, ExcelHyperlinkProperties,
            ExcelHyperlinkPropertyTypeValues, ReferenceRange, RowProperties, StyleId,
        },
        WorkSheet,
    },
};

#[no_mangle]
pub extern "C" fn set_column_ref_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_column_ref_properties = match unsafe {
        root_from_raw::<worksheet_set_column_ref_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let cell_ref = if let Some(cell_ref) = fbs_set_column_ref_properties.cell_ref() {
        cell_ref
    } else {
        return unsafe {
            set_error(
                out_error,
                &AnyError::msg("draviavemal-openxml_office::cell_ref is required"),
                StatusCode::InvalidArgument,
            )
        };
    };
    let worksheet_ptr = fbs_set_column_ref_properties.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_properties =
        if let Some(column_properties) = fbs_set_column_ref_properties.column_properties() {
            let mut new_column_properties = ColumnProperties::default();
            new_column_properties.best_fit = column_properties.best_fit();
            new_column_properties.hidden = column_properties.hidden();
            new_column_properties.style_id =
                if let Some(style_id_ptr) = column_properties.style_id_ptr() {
                    Some(unsafe { *(style_id_ptr as *const StyleId) })
                } else {
                    None
                };
            new_column_properties.width = column_properties.width();
            Some(new_column_properties)
        } else {
            None
        };
    match worksheet.set_column_ref_properties_mut(cell_ref, column_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn set_column_index_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_column_index_properties = match unsafe {
        root_from_raw::<worksheet_set_column_index_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let column_index = fbs_set_column_index_properties.column_index();
    let worksheet_ptr = fbs_set_column_index_properties.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_properties =
        if let Some(column_properties) = fbs_set_column_index_properties.column_properties() {
            let mut new_column_properties = ColumnProperties::default();
            new_column_properties.best_fit = column_properties.best_fit();
            new_column_properties.hidden = column_properties.hidden();
            new_column_properties.style_id =
                if let Some(style_id_ptr) = column_properties.style_id_ptr() {
                    Some(unsafe { *(style_id_ptr as *const StyleId) })
                } else {
                    None
                };
            new_column_properties.width = column_properties.width();
            Some(new_column_properties)
        } else {
            None
        };
    match worksheet.set_column_index_properties_mut(&column_index, column_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn set_row_index_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_row_index_properties = match unsafe {
        root_from_raw::<worksheet_set_row_index_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let row_index = fbs_set_row_index_properties.row_index();
    let worksheet_ptr = fbs_set_row_index_properties.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let row_properties = fbs_set_row_index_properties.row_properties();
    let mut new_row_properties = RowProperties::default();
    new_row_properties.height = row_properties.height();
    new_row_properties.thick_top = row_properties.thick_top();
    new_row_properties.thick_bottom = row_properties.thick_bottom();
    new_row_properties.hidden = row_properties.hidden();
    new_row_properties.style_id = if let Some(style_id_ptr) = row_properties.style_id_ptr() {
        Some(unsafe { *(style_id_ptr as *const StyleId) })
    } else {
        None
    };
    new_row_properties.height = row_properties.height();
    match worksheet.set_row_index_properties_mut(&row_index, new_row_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn set_cell_ref_value(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_cell_ref_value = match unsafe {
        root_from_raw::<worksheet_set_cell_ref_value>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let cell_ref = match fbs_set_cell_ref_value.cell_ref() {
        Some(cell_ref) => cell_ref,
        None => {
            return unsafe {
                set_error(
                    out_error,
                    &AnyError::msg("draviavemal-openxml_office::Invalid cell reference"),
                    StatusCode::InvalidArgument,
                )
            }
        }
    };
    let worksheet_ptr = fbs_set_cell_ref_value.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_cells = fbs_set_cell_ref_value.column_cells();
    let mut cell_properties = Vec::new();
    for column_cell in column_cells {
        let mut cell_property = CellProperty::default();
        cell_property.value = column_cell.value().map(|item| item.to_string());
        cell_property.formula = column_cell.formula().map(|item| item.to_string());
        match column_cell.data_type() {
            worksheet_cell_data_type::string => cell_property.data_type = CellDataType::String,
            worksheet_cell_data_type::number => cell_property.data_type = CellDataType::Number,
            worksheet_cell_data_type::boolean => cell_property.data_type = CellDataType::Boolean,
            worksheet_cell_data_type::shared_string => {
                cell_property.data_type = CellDataType::ShareString
            }
            worksheet_cell_data_type::inline_string => {
                cell_property.data_type = CellDataType::InlineString
            }
            worksheet_cell_data_type::error => cell_property.data_type = CellDataType::Error,
            _ => cell_property.data_type = CellDataType::Auto,
        }
        cell_property.style_id = if let Some(style_id_ptr) = column_cell.style_id_ptr() {
            Some(unsafe { *(style_id_ptr as *const StyleId) })
        } else {
            None
        };
        cell_property.style_id = if let Some(style_id_ptr) = column_cell.style_id_ptr() {
            Some(unsafe { *(style_id_ptr as *const StyleId) })
        } else {
            None
        };
        cell_properties.push(cell_property);
    }
    match worksheet.set_cell_ref_value_mut(&cell_ref, cell_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn set_cell_index_value(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_cell_index_value = match unsafe {
        root_from_raw::<worksheet_set_cell_index_value>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs_set_cell_index_value.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_cells = fbs_set_cell_index_value.column_cells();
    let mut cell_properties = Vec::new();
    for column_cell in column_cells {
        let mut cell_property = CellProperty::default();
        cell_property.value = column_cell.value().map(|item| item.to_string());
        cell_property.formula = column_cell.formula().map(|item| item.to_string());
        match column_cell.data_type() {
            worksheet_cell_data_type::string => cell_property.data_type = CellDataType::String,
            worksheet_cell_data_type::number => cell_property.data_type = CellDataType::Number,
            worksheet_cell_data_type::boolean => cell_property.data_type = CellDataType::Boolean,
            worksheet_cell_data_type::shared_string => {
                cell_property.data_type = CellDataType::ShareString
            }
            worksheet_cell_data_type::inline_string => {
                cell_property.data_type = CellDataType::InlineString
            }
            worksheet_cell_data_type::error => cell_property.data_type = CellDataType::Error,
            _ => cell_property.data_type = CellDataType::Auto,
        }
        cell_property.style_id = if let Some(style_id_ptr) = column_cell.style_id_ptr() {
            Some(unsafe { *(style_id_ptr as *const StyleId) })
        } else {
            None
        };
        cell_property.style_id = if let Some(style_id_ptr) = column_cell.style_id_ptr() {
            Some(unsafe { *(style_id_ptr as *const StyleId) })
        } else {
            None
        };
        cell_properties.push(cell_property);
    }
    match worksheet.set_cell_index_value_mut(
        fbs_set_cell_index_value.row_index(),
        fbs_set_cell_index_value.column_index(),
        cell_properties,
    ) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn worksheet_flush(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_cell_index_value =
        match unsafe { root_from_raw::<worksheet_flush>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let worksheet_ptr = fbs_set_cell_index_value.worksheet_ptr() as *mut WorkSheet;
    let worksheet = unsafe { *Box::from_raw(worksheet_ptr) };
    match worksheet.flush() {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

fn fbs_to_reference_range(fbs: &fbs_reference_range) -> ReferenceRange {
    ReferenceRange {
        column_start: fbs.column_start(),
        column_end: fbs.column_end(),
        row_start: fbs.row_start(),
        row_end: fbs.row_end(),
    }
}

#[no_mangle]
/// Merge a range of cells in the worksheet
pub extern "C" fn worksheet_set_merge_cell(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_set_merge_cell>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&fbs.ref_range());
    match worksheet.set_merge_cell_mut(ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Remove a merged cell range from the worksheet
pub extern "C" fn worksheet_remove_merge_cell(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_remove_merge_cell>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&fbs.ref_range());
    match worksheet.remove_merge_cell_mut(ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Add a hyperlink to a range of cells
pub extern "C" fn worksheet_set_hyperlink(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_set_hyperlink>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let link = match fbs.link() {
        Some(l) => l.to_string(),
        None => {
            return unsafe {
                set_error(
                    out_error,
                    &AnyError::msg("draviavemal-openxml_office::link is required"),
                    StatusCode::InvalidArgument,
                )
            }
        }
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let display = fbs.display().map(|s| s.to_string());
    let ref_range = fbs_to_reference_range(&fbs.ref_range());
    match worksheet.set_hyperlink_mut(display, link, ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Remove a hyperlink from a cell range
pub extern "C" fn worksheet_remove_hyperlink(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_remove_hyperlink>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&fbs.ref_range());
    match worksheet.remove_hyperlink_mut(ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Delete the worksheet and all its components
pub extern "C" fn worksheet_delete_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_delete_sheet>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let worksheet = unsafe { *Box::from_raw(worksheet_ptr) };
    match worksheet.delete_sheet_mut() {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Get cell properties for a range of cells
pub extern "C" fn worksheet_get_range_cell_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_get_range_cell_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&fbs.ref_range());
    match worksheet.get_range_cell_properties(ref_range) {
        Ok(cell_packages) => {
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let pkg_offsets: Vec<_> = cell_packages
                .iter()
                .map(|pkg| {
                    let cell_ref_offset = builder.create_string(&pkg.cell_ref);
                    let value_offset = pkg
                        .cell_property
                        .value
                        .as_deref()
                        .map(|v| builder.create_string(v));
                    let formula_offset = pkg
                        .cell_property
                        .formula
                        .as_deref()
                        .map(|f| builder.create_string(f));
                    let cell_prop = fbs_cell_property::create(
                        &mut builder,
                        &worksheet_cell_propertyArgs {
                            value: value_offset,
                            formula: formula_offset,
                            data_type: match pkg.cell_property.data_type {
                                CellDataType::Number => worksheet_cell_data_type::number,
                                CellDataType::Boolean => worksheet_cell_data_type::boolean,
                                CellDataType::String => worksheet_cell_data_type::string,
                                CellDataType::ShareString => {
                                    worksheet_cell_data_type::shared_string
                                }
                                CellDataType::InlineString => {
                                    worksheet_cell_data_type::inline_string
                                }
                                CellDataType::Error => worksheet_cell_data_type::error,
                                _ => worksheet_cell_data_type::auto,
                            },
                            style_id_ptr: pkg
                                .cell_property
                                .style_id
                                .as_ref()
                                .map(|s| Box::into_raw(Box::new(*s)) as u64),
                        },
                    );
                    worksheet_cell_package::create(
                        &mut builder,
                        &worksheet_cell_packageArgs {
                            cell_ref: Some(cell_ref_offset),
                            row_index: pkg.row_index,
                            column_index: pkg.column_index,
                            cell_property: Some(cell_prop),
                        },
                    )
                })
                .collect();
            let packages_vector = builder.create_vector(&pkg_offsets);
            let result = worksheet_get_range_cell_properties_return::create(
                &mut builder,
                &worksheet_get_range_cell_properties_returnArgs {
                    cell_packages: Some(packages_vector),
                },
            );
            builder.finish(result, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// List all merged cell ranges in the worksheet
pub extern "C" fn worksheet_list_merge_cell(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_list_merge_cell>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ranges = worksheet.list_merge_cell_().unwrap_or_default();
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let range_offsets: Vec<_> = ranges
        .iter()
        .map(|r| {
            fbs_reference_range::create(
                &mut builder,
                &worksheet_reference_rangeArgs {
                    column_start: r.column_start,
                    column_end: r.column_end,
                    row_start: r.row_start,
                    row_end: r.row_end,
                },
            )
        })
        .collect();
    let ranges_vector = builder.create_vector(&range_offsets);
    let result = worksheet_list_merge_cell_return::create(
        &mut builder,
        &worksheet_list_merge_cell_returnArgs {
            ranges: Some(ranges_vector),
        },
    );
    builder.finish(result, None);
    unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
    StatusCode::Success as i8
}

#[no_mangle]
/// List all hyperlinks in the worksheet
pub extern "C" fn worksheet_list_hyperlinks(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<worksheet_list_hyperlinks>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let hyperlinks = worksheet.list_hyperlinks().unwrap_or_default();
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let link_offsets: Vec<_> = hyperlinks
        .iter()
        .map(|(display, link, range)| {
            let display_offset = display.as_deref().map(|d| builder.create_string(d));
            let link_offset = builder.create_string(link);
            let range_offset = fbs_reference_range::create(
                &mut builder,
                &worksheet_reference_rangeArgs {
                    column_start: range.column_start,
                    column_end: range.column_end,
                    row_start: range.row_start,
                    row_end: range.row_end,
                },
            );
            fbs_hyperlink::create(
                &mut builder,
                &worksheet_hyperlinkArgs {
                    display: display_offset,
                    link: Some(link_offset),
                    ref_range: Some(range_offset),
                },
            )
        })
        .collect();
    let links_vector = builder.create_vector(&link_offsets);
    let result = worksheet_list_hyperlinks_return::create(
        &mut builder,
        &worksheet_list_hyperlinks_returnArgs {
            hyperlinks: Some(links_vector),
        },
    );
    builder.finish(result, None);
    unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
    StatusCode::Success as i8
}

#[no_mangle]
/// Add a picture to the worksheet at the specified anchor positions
pub extern "C" fn worksheet_add_picture(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    use draviavemal_openxml_office::{
        global_2007::models::ImageType, spreadsheet_2007::models::ExcelPictureSetting,
    };
    let fbs = match unsafe {
        root_from_raw::<worksheet_add_picture>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let image_path = match fbs.image_path() {
        Some(p) => p,
        None => {
            return unsafe {
                set_error(
                    out_error,
                    &AnyError::msg("draviavemal-openxml_office::image_path is required"),
                    StatusCode::InvalidArgument,
                )
            }
        }
    };
    let worksheet_ptr = fbs.worksheet_ptr() as *mut WorkSheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let setting = fbs.picture_setting();
    let from_fbs = setting.from();
    let to_fbs = setting.to();
    let hyperlink_properties = setting.hyperlink().map(|h| ExcelHyperlinkProperties {
        display: h.display().map(|s| s.to_string()),
        link_type: match h.link_type() {
            worksheet_excel_hyperlink_type::web_url => ExcelHyperlinkPropertyTypeValues::WEB_URL,
            worksheet_excel_hyperlink_type::target_sheet => {
                ExcelHyperlinkPropertyTypeValues::TARGET_SHEET
            }
            _ => ExcelHyperlinkPropertyTypeValues::EXISTING_FILE,
        },
        link: h.link().unwrap_or("").to_string(),
    });

    let picture_setting = ExcelPictureSetting {
        hyperlink_properties,
        picture_setting: PictureSetting {
            image_type: match setting.picture_settings().image_type() {
                global_image_type::png => ImageType::PNG,
                global_image_type::gif => ImageType::GIF,
                global_image_type::bmp => ImageType::BMP,
                global_image_type::tiff => ImageType::TIFF,
                _ => ImageType::JPEG,
            },
        },
        from: AnchorPosition {
            column: from_fbs.column(),
            column_offset: from_fbs.column_offset(),
            row: from_fbs.row(),
            row_offset: from_fbs.row_offset(),
        },
        to: AnchorPosition {
            column: to_fbs.column(),
            column_offset: to_fbs.column_offset(),
            row: to_fbs.row(),
            row_offset: to_fbs.row_offset(),
        },
    };
    match worksheet.add_picture_mut(image_path, picture_setting) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}
