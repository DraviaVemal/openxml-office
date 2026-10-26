use std::{ffi::c_char, mem::ManuallyDrop};

use crate::spreadsheet_2007::excel::ffi::{FfiWorksheet, StyleIdArena};
use crate::{
    ffi::{
        openxml_office_fbs::{
            global::Global_image_type,
            spreadsheet::{
                Worksheet_cell_property as fbs_cell_property, Worksheet_hyperlink as fbs_hyperlink,
                Worksheet_reference_range as fbs_reference_range, *,
            },
        },
        root_from_raw, set_error, write_buffer, StatusCode,
    },
    global_2007::{
        parts::{AnchorPosition, ImageType, PictureSetting},
        traits::XmlDocumentPartFlush,
    },
    spreadsheet_2007::{
        CellDataType, CellProperty, ColumnProperties, ExcelHyperlinkProperties,
        ExcelHyperlinkPropertyTypeValues, ExcelPictureSetting, ReferenceRange, RowProperties,
        StyleId,
    },
};
use anyhow::Error as AnyError;

fn resolve_style_id(
    arena: &StyleIdArena,
    pointer: Option<u64>,
    out_error: *mut *const c_char,
) -> Result<Option<StyleId>, i8> {
    arena
        .resolve_optional(pointer)
        .map_err(|error| unsafe { set_error(out_error, &error, StatusCode::InvalidArgument) })
}

fn fbs_to_reference_range(reference_range: &Worksheet_reference_range) -> ReferenceRange {
    ReferenceRange {
        column_start: reference_range.column_start(),
        column_end: reference_range.column_end(),
        row_start: reference_range.row_start(),
        row_end: reference_range.row_end(),
    }
}

#[no_mangle]
pub extern "C" fn Set_column_ref_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_column_ref_properties = match unsafe {
        root_from_raw::<Worksheet_set_column_ref_properties>(in_buffer, in_buffer_size, out_error)
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
    let worksheet_ptr = fbs_set_column_ref_properties.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_properties =
        if let Some(column_properties) = fbs_set_column_ref_properties.column_properties() {
            let mut new_column_properties = ColumnProperties::default();
            new_column_properties.best_fit = column_properties.best_fit();
            new_column_properties.hidden = column_properties.hidden();
            new_column_properties.style_id = match resolve_style_id(
                &worksheet.style_ids,
                column_properties.style_id_ptr(),
                out_error,
            ) {
                Ok(style_id) => style_id,
                Err(status) => return status,
            };
            new_column_properties.width = column_properties.width();
            Some(new_column_properties)
        } else {
            None
        };
    match worksheet.set_column_ref_properties_mut(cell_ref, column_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn Set_column_index_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_column_index_properties = match unsafe {
        root_from_raw::<Worksheet_set_column_index_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let column_index = fbs_set_column_index_properties.column_index();
    let worksheet_ptr = fbs_set_column_index_properties.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_properties =
        if let Some(column_properties) = fbs_set_column_index_properties.column_properties() {
            let mut new_column_properties = ColumnProperties::default();
            new_column_properties.best_fit = column_properties.best_fit();
            new_column_properties.hidden = column_properties.hidden();
            new_column_properties.style_id = match resolve_style_id(
                &worksheet.style_ids,
                column_properties.style_id_ptr(),
                out_error,
            ) {
                Ok(style_id) => style_id,
                Err(status) => return status,
            };
            new_column_properties.width = column_properties.width();
            Some(new_column_properties)
        } else {
            None
        };
    match worksheet.set_column_index_properties_mut(&column_index, column_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn Set_row_index_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_row_index_properties = match unsafe {
        root_from_raw::<Worksheet_set_row_index_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let row_index = fbs_set_row_index_properties.row_index();
    let worksheet_ptr = fbs_set_row_index_properties.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let row_properties = fbs_set_row_index_properties.row_properties();
    let mut new_row_properties = RowProperties::default();
    new_row_properties.height = row_properties.height();
    new_row_properties.thick_top = row_properties.thick_top();
    new_row_properties.thick_bottom = row_properties.thick_bottom();
    new_row_properties.hidden = row_properties.hidden();
    new_row_properties.style_id = match resolve_style_id(
        &worksheet.style_ids,
        row_properties.style_id_ptr(),
        out_error,
    ) {
        Ok(style_id) => style_id,
        Err(status) => return status,
    };
    new_row_properties.height = row_properties.height();
    match worksheet.set_row_index_properties_mut(&row_index, new_row_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn Set_cell_ref_value(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_cell_ref_value = match unsafe {
        root_from_raw::<Worksheet_set_cell_ref_value>(in_buffer, in_buffer_size, out_error)
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
    let worksheet_ptr = fbs_set_cell_ref_value.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_cells = fbs_set_cell_ref_value.column_cells();
    let mut cell_properties = Vec::new();
    for column_cell in column_cells {
        let mut cell_property = CellProperty::default();
        cell_property.value = column_cell.value().map(|item| item.to_string());
        cell_property.formula = column_cell.formula().map(|item| item.to_string());
        match column_cell.data_type() {
            Worksheet_cell_data_type::string => cell_property.data_type = CellDataType::String,
            Worksheet_cell_data_type::number => cell_property.data_type = CellDataType::Number,
            Worksheet_cell_data_type::boolean => cell_property.data_type = CellDataType::Boolean,
            Worksheet_cell_data_type::shared_string => {
                cell_property.data_type = CellDataType::ShareString
            }
            Worksheet_cell_data_type::inline_string => {
                cell_property.data_type = CellDataType::InlineString
            }
            Worksheet_cell_data_type::error => cell_property.data_type = CellDataType::Error,
            _ => cell_property.data_type = CellDataType::Auto,
        }
        cell_property.style_id =
            match resolve_style_id(&worksheet.style_ids, column_cell.style_id_ptr(), out_error) {
                Ok(style_id) => style_id,
                Err(status) => return status,
            };
        cell_properties.push(cell_property);
    }
    match worksheet.set_cell_ref_value_mut(&cell_ref, cell_properties) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn Set_cell_index_value(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_cell_index_value = match unsafe {
        root_from_raw::<Worksheet_set_cell_index_value>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = fbs_set_cell_index_value.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let column_cells = fbs_set_cell_index_value.column_cells();
    let mut cell_properties = Vec::new();
    for column_cell in column_cells {
        let mut cell_property = CellProperty::default();
        cell_property.value = column_cell.value().map(|item| item.to_string());
        cell_property.formula = column_cell.formula().map(|item| item.to_string());
        match column_cell.data_type() {
            Worksheet_cell_data_type::string => cell_property.data_type = CellDataType::String,
            Worksheet_cell_data_type::number => cell_property.data_type = CellDataType::Number,
            Worksheet_cell_data_type::boolean => cell_property.data_type = CellDataType::Boolean,
            Worksheet_cell_data_type::shared_string => {
                cell_property.data_type = CellDataType::ShareString
            }
            Worksheet_cell_data_type::inline_string => {
                cell_property.data_type = CellDataType::InlineString
            }
            Worksheet_cell_data_type::error => cell_property.data_type = CellDataType::Error,
            _ => cell_property.data_type = CellDataType::Auto,
        }
        cell_property.style_id =
            match resolve_style_id(&worksheet.style_ids, column_cell.style_id_ptr(), out_error) {
                Ok(style_id) => style_id,
                Err(status) => return status,
            };
        cell_properties.push(cell_property);
    }
    match worksheet.set_cell_index_value_mut(
        fbs_set_cell_index_value.row_index(),
        fbs_set_cell_index_value.column_index(),
        cell_properties,
    ) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Merge a range of cells in the worksheet
pub extern "C" fn Worksheet_set_merge_cell(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_set_merge_cell>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&request.ref_range());
    match worksheet.set_merge_cell_mut(ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Remove a merged cell range from the worksheet
pub extern "C" fn Worksheet_remove_merge_cell(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_remove_merge_cell>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&request.ref_range());
    match worksheet.remove_merge_cell_mut(ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Add a hyperlink to a range of cells
pub extern "C" fn Worksheet_set_hyperlink(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_set_hyperlink>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let link = match request.link() {
        Some(link_value) => link_value.to_string(),
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
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let display = request.display().map(|display_text| display_text.to_string());
    let ref_range = fbs_to_reference_range(&request.ref_range());
    match worksheet.set_hyperlink_mut(display, link, ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Remove a hyperlink from a cell range
pub extern "C" fn Worksheet_remove_hyperlink(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_remove_hyperlink>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&request.ref_range());
    match worksheet.remove_hyperlink_mut(ref_range) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
pub extern "C" fn Worksheet_flush(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_set_cell_index_value =
        match unsafe { root_from_raw::<Worksheet_flush>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let worksheet_ptr = fbs_set_cell_index_value.worksheet_ptr() as *mut FfiWorksheet;
    let worksheet = unsafe { Box::from_raw(worksheet_ptr) }.into_inner();
    match worksheet.flush() {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Delete the worksheet and all its components
pub extern "C" fn Worksheet_delete_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_delete_sheet>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let worksheet = unsafe { Box::from_raw(worksheet_ptr) }.into_inner();
    match worksheet.delete_sheet_mut() {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Add a picture to the worksheet at the specified anchor positions
pub extern "C" fn Worksheet_add_picture(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let picture_request = match unsafe {
        root_from_raw::<Worksheet_add_picture>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let image_path = match picture_request.image_path() {
        Some(image_path) => image_path,
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
    let worksheet_ptr = picture_request.worksheet_ptr() as *mut FfiWorksheet;
    let mut worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let setting_request = picture_request.picture_setting();
    let from_position = setting_request.from();
    let to_position = setting_request.to();
    let hyperlink_properties =
        setting_request
            .hyperlink()
            .map(|hyperlink_request| ExcelHyperlinkProperties {
                display: hyperlink_request
                    .display()
                    .map(|display| display.to_string()),
                link_type: match hyperlink_request.link_type() {
                    Worksheet_excel_hyperlink_type::web_url => {
                        ExcelHyperlinkPropertyTypeValues::WEB_URL
                    }
                    Worksheet_excel_hyperlink_type::target_sheet => {
                        ExcelHyperlinkPropertyTypeValues::TARGET_SHEET
                    }
                    _ => ExcelHyperlinkPropertyTypeValues::EXISTING_FILE,
                },
                link: hyperlink_request.link().unwrap_or("").to_string(),
            });

    let picture_setting = ExcelPictureSetting {
        hyperlink_properties,
        picture_setting: PictureSetting {
            image_type: match setting_request.picture_settings().image_type() {
                Global_image_type::png => ImageType::PNG,
                Global_image_type::gif => ImageType::GIF,
                Global_image_type::bmp => ImageType::BMP,
                Global_image_type::tiff => ImageType::TIFF,
                _ => ImageType::JPEG,
            },
        },
        from: AnchorPosition {
            column: from_position.column(),
            column_offset: from_position.column_offset(),
            row: from_position.row(),
            row_offset: from_position.row_offset(),
        },
        to: AnchorPosition {
            column: to_position.column(),
            column_offset: to_position.column_offset(),
            row: to_position.row(),
            row_offset: to_position.row_offset(),
        },
    };
    match worksheet.add_picture_mut(image_path, picture_setting) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Get cell properties for a range of cells
pub extern "C" fn Worksheet_get_range_cell_properties(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_get_range_cell_properties>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let ref_range = fbs_to_reference_range(&request.ref_range());
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
                        .map(|value| builder.create_string(value));
                    let formula_offset = pkg
                        .cell_property
                        .formula
                        .as_deref()
                        .map(|formula| builder.create_string(formula));
                    let cell_prop = fbs_cell_property::create(
                        &mut builder,
                        &Worksheet_cell_propertyArgs {
                            value: value_offset,
                            formula: formula_offset,
                            data_type: match pkg.cell_property.data_type {
                                CellDataType::Number => Worksheet_cell_data_type::number,
                                CellDataType::Boolean => Worksheet_cell_data_type::boolean,
                                CellDataType::String => Worksheet_cell_data_type::string,
                                CellDataType::ShareString => {
                                    Worksheet_cell_data_type::shared_string
                                }
                                CellDataType::InlineString => {
                                    Worksheet_cell_data_type::inline_string
                                }
                                CellDataType::Error => Worksheet_cell_data_type::error,
                                _ => Worksheet_cell_data_type::auto,
                            },
                            style_id_ptr: pkg
                                .cell_property
                                .style_id
                                .as_ref()
                                .map(|style_id| worksheet.style_ids.insert(*style_id)),
                        },
                    );
                    Worksheet_cell_package::create(
                        &mut builder,
                        &Worksheet_cell_packageArgs {
                            cell_ref: Some(cell_ref_offset),
                            row_index: pkg.row_index,
                            column_index: pkg.column_index,
                            cell_property: Some(cell_prop),
                        },
                    )
                })
                .collect();
            let packages_vector = builder.create_vector(&pkg_offsets);
            let result = Worksheet_get_range_cell_properties_return::create(
                &mut builder,
                &Worksheet_get_range_cell_properties_returnArgs {
                    cell_packages: Some(packages_vector),
                },
            );
            builder.finish(result, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// List all merged cell ranges in the worksheet
pub extern "C" fn Worksheet_list_merge_cell(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_list_merge_cell>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let mut range_offsets = Vec::new();
    worksheet.for_each_merge_cell(|merge_range| {
        range_offsets.push(fbs_reference_range::create(
            &mut builder,
            &Worksheet_reference_rangeArgs {
                column_start: merge_range.column_start,
                column_end: merge_range.column_end,
                row_start: merge_range.row_start,
                row_end: merge_range.row_end,
            },
        ));
    });
    let ranges_vector = builder.create_vector(&range_offsets);
    let result = Worksheet_list_merge_cell_return::create(
        &mut builder,
        &Worksheet_list_merge_cell_returnArgs {
            ranges: Some(ranges_vector),
        },
    );
    builder.finish(result, None);
    unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
    StatusCode::Success as i8
}

#[no_mangle]
/// List all hyperlinks in the worksheet
pub extern "C" fn Worksheet_list_hyperlinks(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Worksheet_list_hyperlinks>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let worksheet_ptr = request.worksheet_ptr() as *mut FfiWorksheet;
    let worksheet = unsafe { ManuallyDrop::new(Box::from_raw(worksheet_ptr)) };
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let mut link_offsets = Vec::new();
    worksheet.for_each_hyperlink(|display, link, range| {
        let display_offset = display.map(|display_text| builder.create_string(display_text));
        let link_offset = builder.create_string(link);
        let range_offset = fbs_reference_range::create(
            &mut builder,
            &Worksheet_reference_rangeArgs {
                column_start: range.column_start,
                column_end: range.column_end,
                row_start: range.row_start,
                row_end: range.row_end,
            },
        );
        link_offsets.push(fbs_hyperlink::create(
            &mut builder,
            &Worksheet_hyperlinkArgs {
                display: display_offset,
                link: Some(link_offset),
                ref_range: Some(range_offset),
            },
        ));
    });
    let links_vector = builder.create_vector(&link_offsets);
    let result = Worksheet_list_hyperlinks_return::create(
        &mut builder,
        &Worksheet_list_hyperlinks_returnArgs {
            hyperlinks: Some(links_vector),
        },
    );
    builder.finish(result, None);
    unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
    StatusCode::Success as i8
}
