use crate::{
    chain_error,
    openxml_office_fbs::{self},
    StatusCode,
};
use draviavemal_openxml_office::spreadsheet_2007::{Excel, ExcelPropertiesModel};
use std::{ffi::c_char, mem::ManuallyDrop, slice::from_raw_parts};

#[no_mangle]
/// Creates a new Excel object.
///
/// Returns a pointer to the newly created Excel object.
/// If an error occurs, returns a null pointer.
pub extern "C" fn excel_create(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    if in_buffer.is_null() || in_buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(in_buffer, in_buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet::excel_create>(buffer_slice) {
        Ok(fbs_excel_create) => {
            let file_name = fbs_excel_create.file_name().map(|item| item.to_string());
            let excel_properties = ExcelPropertiesModel {
                is_editable: fbs_excel_create.excel_settings().is_editable(),
            };
            let excel = Excel::new(file_name, excel_properties);
            match excel {
                Ok(excel) => {
                    let excel_ptr = Box::into_raw(Box::new(excel)) as u64;
                    let mut builder = flatbuffers::FlatBufferBuilder::new();
                    let excel_create_return =
                        openxml_office_fbs::spreadsheet::excel_create_return::create(
                            &mut builder,
                            &openxml_office_fbs::spreadsheet::excel_create_returnArgs {
                                excel_ptr: excel_ptr,
                            },
                        );
                    builder.finish(excel_create_return, None);
                    let buf = builder.finished_data();
                    unsafe {
                        *out_buffer = buf.as_ptr() as *mut u8;
                        *out_buffer_size = buf.len();
                    }
                    StatusCode::Success as i8
                }
                Err(e) => {
                    unsafe { *out_error = chain_error(&e) };
                    StatusCode::UnknownError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}

#[no_mangle]
/// Add New Sheet to the Excel
pub extern "C" fn excel_add_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    if in_buffer.is_null() || in_buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(in_buffer, in_buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet::excel_add_sheet>(buffer_slice) {
        Ok(fbs_add_sheet) => {
            let excel_ptr = fbs_add_sheet.excel_ptr() as *mut Excel;
            let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
            match excel.add_sheet_mut(fbs_add_sheet.sheet_name().map(|item| item.to_string())) {
                Ok(worksheet) => {
                    let worksheet_ptr = Box::into_raw(Box::new(worksheet)) as u64;
                    let mut builder = flatbuffers::FlatBufferBuilder::new();
                    let excel_add_sheet =
                        openxml_office_fbs::spreadsheet::excel_add_sheet_return::create(
                            &mut builder,
                            &openxml_office_fbs::spreadsheet::excel_add_sheet_returnArgs {
                                worksheet_ptr: worksheet_ptr,
                            },
                        );
                    builder.finish(excel_add_sheet, None);
                    let buf = builder.finished_data();
                    unsafe {
                        *out_buffer = buf.as_ptr() as *mut u8;
                        *out_buffer_size = buf.len();
                    }
                    StatusCode::Success as i8
                }
                Err(e) => {
                    unsafe { *out_error = chain_error(&e) };
                    StatusCode::IoError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}

#[no_mangle]
/// Get Existing Sheet from Excel
pub extern "C" fn excel_rename_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    if in_buffer.is_null() || in_buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(in_buffer, in_buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet::excel_rename_sheet>(buffer_slice) {
        Ok(fbs_excel_rename_sheet) => {
            let excel_ptr = fbs_excel_rename_sheet.excel_ptr() as *mut Excel;
            let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
            match excel.rename_sheet_name_mut(
                fbs_excel_rename_sheet
                    .old_sheet_name()
                    .map(|item| item.to_string())
                    .unwrap(),
                fbs_excel_rename_sheet
                    .new_sheet_name()
                    .map(|item| item.to_string())
                    .unwrap(),
            ) {
                Ok(()) => StatusCode::Success as i8,
                Err(e) => {
                    unsafe { *out_error = chain_error(&e) };
                    StatusCode::IoError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}

#[no_mangle]
/// Get Existing Sheet from Excel
pub extern "C" fn excel_get_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    if in_buffer.is_null() || in_buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(in_buffer, in_buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet::excel_add_sheet>(buffer_slice) {
        Ok(fbs_add_sheet) => {
            let excel_ptr = fbs_add_sheet.excel_ptr() as *mut Excel;
            let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
            match excel.get_worksheet_mut(
                fbs_add_sheet
                    .sheet_name()
                    .map(|item| item.to_string())
                    .unwrap(),
            ) {
                Ok(worksheet) => {
                    let worksheet_ptr = Box::into_raw(Box::new(worksheet)) as u64;
                    let mut builder = flatbuffers::FlatBufferBuilder::new();
                    let excel_get_sheet_return =
                        openxml_office_fbs::spreadsheet::excel_get_sheet_return::create(
                            &mut builder,
                            &openxml_office_fbs::spreadsheet::excel_get_sheet_returnArgs {
                                worksheet_ptr: worksheet_ptr,
                            },
                        );
                    builder.finish(excel_get_sheet_return, None);
                    let buf = builder.finished_data();
                    unsafe {
                        *out_buffer = buf.as_ptr() as *mut u8;
                        *out_buffer_size = buf.len();
                    }
                    StatusCode::Success as i8
                }
                Err(e) => {
                    unsafe { *out_error = chain_error(&e) };
                    StatusCode::IoError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}

#[no_mangle]
/// List Sheet Name from Excel
pub extern "C" fn excel_list_sheet_name(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    if in_buffer.is_null() || in_buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(in_buffer, in_buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet::excel_list_sheet>(buffer_slice) {
        Ok(fbs_list_sheet_name) => {
            let excel_ptr = fbs_list_sheet_name.excel_ptr() as *mut Excel;
            let excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
            match excel.list_sheet_names() {
                Ok(sheet_names) => {
                    let mut builder = flatbuffers::FlatBufferBuilder::new();
                    let sheet_names_offsets: Vec<_> = sheet_names
                        .iter()
                        .map(|name| builder.create_string(name))
                        .collect();
                    let sheet_names_vector = builder.create_vector(&sheet_names_offsets);
                    let excel_list_sheet_return =
                        openxml_office_fbs::spreadsheet::excel_list_sheet_return::create(
                            &mut builder,
                            &openxml_office_fbs::spreadsheet::excel_list_sheet_returnArgs {
                                sheet_names: Some(sheet_names_vector),
                            },
                        );
                    builder.finish(excel_list_sheet_return, None);
                    let buf = builder.finished_data();
                    unsafe {
                        *out_buffer = buf.as_ptr() as *mut u8;
                        *out_buffer_size = buf.len();
                    }
                    StatusCode::Success as i8
                }
                Err(e) => {
                    unsafe { *out_error = chain_error(&e) };
                    StatusCode::IoError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}

#[no_mangle]
///Save the Excel File in provided file path
pub extern "C" fn excel_save_as(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    if in_buffer.is_null() || in_buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(in_buffer, in_buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet::excel_save_as>(buffer_slice) {
        Ok(fbs_save_as) => {
            let excel_ptr = fbs_save_as.excel_ptr() as *mut Excel;
            let excel = unsafe { *Box::from_raw(excel_ptr) };
            match excel.save_as(
                fbs_save_as
                    .file_name()
                    .map(|item| item.to_string())
                    .unwrap()
                    .as_str(),
            ) {
                Ok(full_path) => {
                    let mut builder = flatbuffers::FlatBufferBuilder::new();
                    let full_path_offset = builder.create_string(&full_path);
                    let excel_save_as_return =
                        openxml_office_fbs::spreadsheet::excel_save_as_return::create(
                            &mut builder,
                            &openxml_office_fbs::spreadsheet::excel_save_as_returnArgs {
                                full_path: Some(full_path_offset),
                            },
                        );
                    builder.finish(excel_save_as_return, None);
                    let buf = builder.finished_data();
                    unsafe {
                        *out_buffer = buf.as_ptr() as *mut u8;
                        *out_buffer_size = buf.len();
                    }
                    StatusCode::Success as i8
                }
                Err(err) => {
                    unsafe { *out_error = chain_error(&err) };
                    StatusCode::IoError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}
