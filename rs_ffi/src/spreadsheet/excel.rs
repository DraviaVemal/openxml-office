use crate::{chain_error, openxml_office_fbs, StatusCode};
use draviavemal_openxml_office::{
    global_2007::traits::Enum,
    spreadsheet_2007::{
        models::{NumberFormatValues, StyleSetting},
        Excel, ExcelPropertiesModel,
    },
};
use std::{
    ffi::{c_char, c_void, CStr, CString},
    mem::ManuallyDrop,
    slice::from_raw_parts,
};

#[no_mangle]
/// Creates a new Excel object.
///
/// Returns a pointer to the newly created Excel object.
/// If an error occurs, returns a null pointer.
pub extern "C" fn excel_create(
    file_name: *const c_char,
    buffer: *const u8,
    buffer_size: usize,
    out_excel: *mut *mut c_void,
    out_error: *mut *const c_char,
) -> i8 {
    let file_name = if file_name.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(file_name) }
                .to_string_lossy()
                .into_owned(),
        )
    };
    if buffer.is_null() || buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(buffer, buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet_2007::ExcelPropertiesModel>(
        buffer_slice,
    ) {
        Ok(fbs_excel_properties) => {
            let excel_properties = ExcelPropertiesModel {
                is_editable: fbs_excel_properties.is_editable(),
            };
            let excel = if let Some(file_name) = file_name {
                Excel::new(Some(file_name), excel_properties)
            } else {
                Excel::new(None, excel_properties)
            };
            match excel {
                Ok(excel) => {
                    unsafe {
                        *out_excel = Box::into_raw(Box::new(excel)) as *mut c_void;
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
    excel_ptr: *const c_void,
    sheet_name: *const c_char,
    out_worksheet: *mut *mut c_void,
    out_error: *mut *const c_char,
) -> i8 {
    if excel_ptr.is_null() {
        eprintln!("Received null pointer");
        return StatusCode::InvalidArgument as i8;
    }
    let excel_ptr = excel_ptr as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    if sheet_name.is_null() {
        match excel.add_sheet_mut(None) {
            Result::Ok(worksheet) => {
                unsafe {
                    *out_worksheet = Box::into_raw(Box::new(worksheet)) as *mut c_void;
                }
                StatusCode::Success as i8
            }
            Err(e) => match CString::new(format!("Flat Buffer Parse Error. {}", e)) {
                Result::Ok(str) => {
                    unsafe { *out_error = str.into_raw() };
                    StatusCode::IoError as i8
                }
                Err(e) => {
                    eprintln!("Error String send Error. {}", e);
                    StatusCode::IoError as i8
                }
            },
        }
    } else {
        let sheet_name = unsafe { CStr::from_ptr(sheet_name) }
            .to_string_lossy()
            .into_owned();
        match excel.add_sheet_mut(Some(sheet_name)) {
            Result::Ok(worksheet) => {
                unsafe {
                    *out_worksheet = Box::into_raw(Box::new(worksheet)) as *mut c_void;
                }
                StatusCode::Success as i8
            }
            Err(e) => match CString::new(format!("Flat Buffer Parse Error. {}", e)) {
                Result::Ok(str) => {
                    unsafe { *out_error = str.into_raw() };
                    StatusCode::IoError as i8
                }
                Err(e) => {
                    eprintln!("Error String send Error. {}", e);
                    StatusCode::IoError as i8
                }
            },
        }
    }
}

#[no_mangle]
/// Get Existing Sheet from Excel
pub extern "C" fn excel_rename_sheet(
    excel_ptr: *const c_void,
    old_sheet_name: *const c_char,
    new_sheet_name: *const c_char,
    out_error: *mut *const c_char,
) -> i8 {
    if excel_ptr.is_null() || old_sheet_name.is_null() || new_sheet_name.is_null() {
        eprintln!("Received null pointer");
        return StatusCode::InvalidArgument as i8;
    }
    let excel_ptr = excel_ptr as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    let old_sheet_name = unsafe { CStr::from_ptr(old_sheet_name) }
        .to_string_lossy()
        .into_owned();
    let new_sheet_name = unsafe { CStr::from_ptr(new_sheet_name) }
        .to_string_lossy()
        .into_owned();
    match excel.rename_sheet_name_mut(old_sheet_name, new_sheet_name) {
        Result::Ok(()) => StatusCode::Success as i8,
        Err(e) => match CString::new(format!("Flat Buffer Parse Error. {}", e)) {
            Result::Ok(str) => {
                unsafe { *out_error = str.into_raw() };
                StatusCode::Success as i8
            }
            Err(e) => {
                eprintln!("Error String send Error. {}", e);
                StatusCode::IoError as i8
            }
        },
    }
}

#[no_mangle]
/// Get Existing Sheet from Excel
pub extern "C" fn excel_get_sheet(
    excel_ptr: *const c_void,
    sheet_name: *const c_char,
    out_worksheet: *mut *mut c_void,
    out_error: *mut *const c_char,
) -> i8 {
    if excel_ptr.is_null() || sheet_name.is_null() {
        eprintln!("Received null pointer");
        return StatusCode::InvalidArgument as i8;
    }
    let excel_ptr = excel_ptr as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    let sheet_name = unsafe { CStr::from_ptr(sheet_name) }
        .to_string_lossy()
        .into_owned();
    match excel.get_worksheet_mut(sheet_name) {
        Result::Ok(worksheet) => {
            unsafe {
                *out_worksheet = Box::into_raw(Box::new(worksheet)) as *mut c_void;
            }
            StatusCode::Success as i8
        }
        Err(e) => match CString::new(format!("Flat Buffer Parse Error. {}", e)) {
            Result::Ok(str) => {
                unsafe { *out_error = str.into_raw() };
                StatusCode::IoError as i8
            }
            Err(e) => {
                eprintln!("Error String send Error. {}", e);
                StatusCode::IoError as i8
            }
        },
    }
}

// #[no_mangle]
// /// List Sheet Name from Excel
// pub extern "C" fn excel_list_sheet_name(
//     excel_ptr: *const c_void,
//     out_error: *mut *const c_char,
// ) -> i8 {
//     if excel_ptr.is_null() {
//         eprintln!("Received null pointer");
//         return StatusCode::InvalidArgument as i8;
//     }
//     let excel_ptr = excel_ptr as *mut Excel;
//     let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
//     let sheet_names = excel.list_sheet_names();
//     return StatusCode::InvalidArgument as i8;
// }

#[no_mangle]
/// Hide specific sheet in workbook
pub extern "C" fn excel_hide_sheet(
    excel_ptr: *const c_void,
    sheet_name: *const c_char,
    out_error: *mut *const c_char,
) -> i8 {
    if excel_ptr.is_null() || sheet_name.is_null() {
        eprintln!("Received null pointer");
        return StatusCode::InvalidArgument as i8;
    }
    let excel_ptr = excel_ptr as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    let sheet_name = unsafe { CStr::from_ptr(sheet_name) }
        .to_string_lossy()
        .into_owned();
    match excel.hide_sheet_mut(sheet_name) {
        Result::Ok(()) => StatusCode::Success as i8,
        Err(e) => match CString::new(format!("Flat Buffer Parse Error. {}", e)) {
            Result::Ok(str) => {
                unsafe { *out_error = str.into_raw() };
                StatusCode::IoError as i8
            }
            Err(e) => {
                eprintln!("Error String send Error. {}", e);
                StatusCode::IoError as i8
            }
        },
    }
}

#[no_mangle]
/// Creates a new Excel object.
///
/// Returns a pointer to the newly created Excel object.
/// If an error occurs, returns a null pointer.
pub extern "C" fn get_style_id_mut(
    excel_ptr: *const c_void,
    buffer: *const u8,
    buffer_size: usize,
    out_style_id: *mut *mut u32,
    out_error: *mut *const c_char,
) -> i8 {
    if excel_ptr.is_null() || buffer.is_null() || buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let excel_ptr = excel_ptr as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    let buffer_slice = unsafe { from_raw_parts(buffer, buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::spreadsheet_2007::StyleSetting>(buffer_slice) {
        Ok(fbs_style_setting) => {
            let mut style_setting: StyleSetting = StyleSetting::default();
            // number format
            style_setting.number_format =
                NumberFormatValues::get_enum(&fbs_style_setting.number_format().to_string());
            style_setting.custom_number_format = fbs_style_setting
                .custom_number_format()
                .map(|s| s.to_string());
            // border
            // style_setting.border_left = fbs_style_setting.border_left();
            // font
            if let Some(font_family) = fbs_style_setting.font_family() {
                style_setting.font_family = font_family.to_string();
            }
            style_setting.font_size = fbs_style_setting.font_size();
            if let Some(_text_color) = fbs_style_setting.text_color() {
                // style_setting.text_color = text_color;
            }
            style_setting.is_bold = fbs_style_setting.is_bold();
            style_setting.is_italic = fbs_style_setting.is_bold();
            style_setting.is_underline = fbs_style_setting.is_bold();
            style_setting.is_double_underline = fbs_style_setting.is_bold();
            // fill
            // xfs
            style_setting.background_color =
                fbs_style_setting.background_color().map(|s| s.to_string());
            style_setting.foreground_color =
                fbs_style_setting.foreground_color().map(|s| s.to_string());
            style_setting.is_wrap_text = fbs_style_setting.is_wrap_text();
            match excel.get_style_id_mut(style_setting) {
                Ok(style_id) => {
                    unsafe {
                        *out_style_id = Box::into_raw(Box::new(style_id.get_id())) as *mut u32;
                    }
                    StatusCode::Success as i8
                }
                Err(err) => match CString::new(format!("Flat Buffer Parse Error. {}", err)) {
                    Result::Ok(str) => {
                        unsafe { *out_error = str.into_raw() };
                        StatusCode::IoError as i8
                    }
                    Err(e) => {
                        eprintln!("Error String send Error. {}", e);
                        StatusCode::IoError as i8
                    }
                },
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
    excel_ptr: *const c_void,
    file_name: *const c_char,
    out_error: *mut *const c_char,
) -> i8 {
    if excel_ptr.is_null() || file_name.is_null() {
        return StatusCode::InvalidArgument as i8;
    }
    let file_name = unsafe { CStr::from_ptr(file_name) }
        .to_string_lossy()
        .into_owned();
    let excel_ptr = excel_ptr as *mut Excel;
    let excel = unsafe { Box::from_raw(excel_ptr) };
    match excel.save_as(&file_name) {
        Result::Ok(_full_path) => StatusCode::Success as i8,
        Err(err) => match CString::new(format!("Flat Buffer Parse Error. {}", err)) {
            Result::Ok(str) => {
                unsafe { *out_error = str.into_raw() };
                StatusCode::IoError as i8
            }
            Err(e) => {
                eprintln!("Error String send Error. {}", e);
                StatusCode::IoError as i8
            }
        },
    }
}
