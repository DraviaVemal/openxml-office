use std::{ffi::c_char, mem::ManuallyDrop};

use crate::{
    openxml_office_fbs::spreadsheet::{
        worksheet_cell_data_type, worksheet_flush, worksheet_set_cell_index_value,
        worksheet_set_cell_ref_value, worksheet_set_column_index_properties,
        worksheet_set_column_ref_properties, worksheet_set_row_index_properties,
    },
    root_from_raw, set_error, StatusCode,
};
use anyhow::anyhow;
use draviavemal_openxml_office::{
    global_2007::traits::XmlDocumentPartFlush,
    spreadsheet_2007::{
        models::{CellDataType, CellProperty, ColumnProperties, RowProperties, StyleId},
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
                &anyhow!("cell_ref is required"),
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
                    &anyhow!("Invalid cell reference"),
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
