use crate::{
    openxml_office_fbs::spreadsheet::{
        Border_style_values, Color_setting_type_values, Excel_add_sheet, Excel_add_sheet_return,
        Excel_add_sheet_returnArgs, Excel_create, Excel_create_return, Excel_create_returnArgs,
        Excel_get_sheet_return, Excel_get_sheet_returnArgs, Excel_get_style_id,
        Excel_get_style_id_return, Excel_get_style_id_returnArgs, Excel_list_sheet,
        Excel_list_sheet_return, Excel_list_sheet_returnArgs, Excel_rename_sheet, Excel_save_as,
        Excel_save_as_return, Excel_save_as_returnArgs, Horizontal_alignment_values,
        Number_format_values, Vertical_alignment_values, Excel_set_active_sheet,
        Excel_set_visibility, Excel_minimize_workbook, Excel_hide_sheet_tabs,
        Excel_hide_vertical_scroll, Excel_hide_horizontal_scroll, Excel_hide_sheet,
    },
    root_from_raw, set_error, write_buffer, StatusCode,
};
use draviavemal_openxml_office::spreadsheet_2007::{
    models::{
        BorderSetting, BorderStyleValues, ColorSetting,
        ColorSettingTypeValues::{Indexed, Rgb, Theme},
        HorizontalAlignmentValues, NumberFormatValues, CellStyleSetting, VerticalAlignmentValues,
    },
    Excel, ExcelPropertiesModel,
};
use std::{ffi::c_char, mem::ManuallyDrop};

#[no_mangle]
/// Creates a new Excel object.
///
/// Returns a pointer to the newly created Excel object.
/// If an error occurs, returns a null pointer.
pub extern "C" fn Excel_create(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_excel_create =
        match unsafe { root_from_raw::<Excel_create>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let file_name = fbs_excel_create.file_name().map(|item| item.to_string());
    let excel_properties = ExcelPropertiesModel {
        is_editable: fbs_excel_create.excel_settings().is_editable(),
    };
    match Excel::new(file_name, excel_properties) {
        Ok(excel) => {
            let excel_ptr = Box::into_raw(Box::new(excel)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_create_return = Excel_create_return::create(
                &mut builder,
                &Excel_create_returnArgs {
                    excel_ptr: excel_ptr,
                },
            );
            builder.finish(excel_create_return, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Add New Sheet to the Excel
pub extern "C" fn Excel_add_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_add_sheet =
        match unsafe { root_from_raw::<Excel_add_sheet>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let excel_ptr = fbs_add_sheet.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.add_sheet_mut(fbs_add_sheet.sheet_name().map(|item| item.to_string())) {
        Ok(worksheet) => {
            let worksheet_ptr = Box::into_raw(Box::new(worksheet)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_add_sheet = Excel_add_sheet_return::create(
                &mut builder,
                &Excel_add_sheet_returnArgs {
                    worksheet_ptr: worksheet_ptr,
                },
            );
            builder.finish(excel_add_sheet, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Get Existing Sheet from Excel
pub extern "C" fn Excel_rename_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_excel_rename_sheet = match unsafe {
        root_from_raw::<Excel_rename_sheet>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
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
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Get Existing Sheet from Excel
pub extern "C" fn Excel_get_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_add_sheet =
        match unsafe { root_from_raw::<Excel_add_sheet>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
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
            let excel_get_sheet_return = Excel_get_sheet_return::create(
                &mut builder,
                &Excel_get_sheet_returnArgs {
                    worksheet_ptr: worksheet_ptr,
                },
            );
            builder.finish(excel_get_sheet_return, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// List Sheet Name from Excel
pub extern "C" fn Excel_list_sheet_name(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_list_sheet_name =
        match unsafe { root_from_raw::<Excel_list_sheet>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
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
            let excel_list_sheet_return = Excel_list_sheet_return::create(
                &mut builder,
                &Excel_list_sheet_returnArgs {
                    sheet_names: Some(sheet_names_vector),
                },
            );
            builder.finish(excel_list_sheet_return, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Get Style ID from Excel
pub extern "C" fn Excel_get_style_id(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_get_style_id = match unsafe {
        root_from_raw::<Excel_get_style_id>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs_get_style_id.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };

    let mut style_setting = CellStyleSetting::default();
    // num format
    style_setting.number_format = match fbs_get_style_id.style_setting().number_format() {
        Number_format_values::integer => NumberFormatValues::Integer,
        Number_format_values::decimal_two_places => NumberFormatValues::DecimalTwoPlaces,
        Number_format_values::thousands_separator => NumberFormatValues::ThousandsSeparator,
        Number_format_values::thousands_separator_two_decimals => {
            NumberFormatValues::ThousandsSeparatorTwoDecimals
        }
        Number_format_values::currency_no_decimals => NumberFormatValues::CurrencyNoDecimals,
        Number_format_values::currency_no_decimals_red => NumberFormatValues::CurrencyNoDecimalsRed,
        Number_format_values::currency_two_decimals => NumberFormatValues::CurrencyTwoDecimals,
        Number_format_values::currency_two_decimals_red => {
            NumberFormatValues::CurrencyTwoDecimalsRed
        }
        Number_format_values::percentage => NumberFormatValues::Percentage,
        Number_format_values::percentage_two_decimals => NumberFormatValues::PercentageTwoDecimals,
        Number_format_values::scientific => NumberFormatValues::Scientific,
        Number_format_values::fraction_one_digit => NumberFormatValues::FractionOneDigit,
        Number_format_values::fraction_two_digits => NumberFormatValues::FractionTwoDigits,
        Number_format_values::date_mmddyy => NumberFormatValues::DateMMDDYY,
        Number_format_values::date_dmmmyy => NumberFormatValues::DateDMmmYY,
        Number_format_values::date_dmmm => NumberFormatValues::DateDMmm,
        Number_format_values::date_mmmyy => NumberFormatValues::DateMmmYY,
        Number_format_values::time_12_hour => NumberFormatValues::Time12Hour,
        Number_format_values::time_12_hour_with_seconds => {
            NumberFormatValues::Time12HourWithSeconds
        }
        Number_format_values::time_24_hour => NumberFormatValues::Time24Hour,
        Number_format_values::time_24_hour_with_seconds => {
            NumberFormatValues::Time24HourWithSeconds
        }
        Number_format_values::date_time_mmddyy => NumberFormatValues::DateTimeMMDDYY,
        Number_format_values::accounting_no_decimals => NumberFormatValues::AccountingNoDecimals,
        Number_format_values::accounting_no_decimals_red => {
            NumberFormatValues::AccountingNoDecimalsRed
        }
        Number_format_values::accounting_two_decimals => NumberFormatValues::AccountingTwoDecimals,
        Number_format_values::accounting_two_decimals_red => {
            NumberFormatValues::AccountingTwoDecimalsRed
        }
        Number_format_values::accounting_negative_in_parentheses => {
            NumberFormatValues::AccountingNegativeInParentheses
        }
        Number_format_values::accounting_two_decimals_negative_in_parentheses => {
            NumberFormatValues::AccountingTwoDecimalsNegativeInParentheses
        }
        Number_format_values::accounting_aligned_symbols => {
            NumberFormatValues::AccountingAlignedSymbols
        }
        Number_format_values::accounting_aligned_symbols_two_decimals => {
            NumberFormatValues::AccountingAlignedSymbolsTwoDecimals
        }
        Number_format_values::time_minutes_seconds => NumberFormatValues::TimeMinutesSeconds,
        Number_format_values::time_hours_minutes_seconds => {
            NumberFormatValues::TimeHoursMinutesSeconds
        }
        Number_format_values::elapsed_time_with_fractions => {
            NumberFormatValues::ElapsedTimeWithFractions
        }
        Number_format_values::scientific_one_decimal => NumberFormatValues::ScientificOneDecimal,
        Number_format_values::text_format => NumberFormatValues::TextFormat,
        Number_format_values::custom => NumberFormatValues::Custom,
        _ => NumberFormatValues::General,
    };
    style_setting.custom_number_format = fbs_get_style_id
        .style_setting()
        .custom_number_format()
        .map(|item| item.to_string());

    // border
    let mut border_left = BorderSetting::default();
    if let Some(color_setting) = fbs_get_style_id
        .style_setting()
        .border_left()
        .border_color()
    {
        let mut clr_setting = ColorSetting::default();
        clr_setting.color_setting_type = match color_setting.color_setting_type() {
            Color_setting_type_values::rgb => Rgb,
            Color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_left.border_color = Some(clr_setting);
    }
    border_left.style = match fbs_get_style_id.style_setting().border_left().style() {
        Border_style_values::thin => BorderStyleValues::Thin,
        Border_style_values::thick => BorderStyleValues::Thick,
        _ => BorderStyleValues::None,
    };
    style_setting.border_left = border_left;

    let mut border_top = BorderSetting::default();
    if let Some(color_setting) = fbs_get_style_id.style_setting().border_top().border_color() {
        let mut clr_setting = ColorSetting::default();
        clr_setting.color_setting_type = match color_setting.color_setting_type() {
            Color_setting_type_values::rgb => Rgb,
            Color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_top.border_color = Some(clr_setting);
    }
    border_top.style = match fbs_get_style_id.style_setting().border_top().style() {
        Border_style_values::thin => BorderStyleValues::Thin,
        Border_style_values::thick => BorderStyleValues::Thick,
        _ => BorderStyleValues::None,
    };
    style_setting.border_top = border_top;

    let mut border_right = BorderSetting::default();
    if let Some(color_setting) = fbs_get_style_id
        .style_setting()
        .border_right()
        .border_color()
    {
        let mut clr_setting = ColorSetting::default();
        clr_setting.color_setting_type = match color_setting.color_setting_type() {
            Color_setting_type_values::rgb => Rgb,
            Color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_right.border_color = Some(clr_setting);
    }
    border_right.style = match fbs_get_style_id.style_setting().border_right().style() {
        Border_style_values::thin => BorderStyleValues::Thin,
        Border_style_values::thick => BorderStyleValues::Thick,
        _ => BorderStyleValues::None,
    };
    style_setting.border_right = border_right;

    let mut border_bottom = BorderSetting::default();
    if let Some(color_setting) = fbs_get_style_id
        .style_setting()
        .border_bottom()
        .border_color()
    {
        let mut clr_setting = ColorSetting::default();
        clr_setting.color_setting_type = match color_setting.color_setting_type() {
            Color_setting_type_values::rgb => Rgb,
            Color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_bottom.border_color = Some(clr_setting);
    }
    border_bottom.style = match fbs_get_style_id.style_setting().border_bottom().style() {
        Border_style_values::thin => BorderStyleValues::Thin,
        Border_style_values::thick => BorderStyleValues::Thick,
        _ => BorderStyleValues::None,
    };
    style_setting.border_bottom = border_bottom;

    let mut border_diagonal = BorderSetting::default();
    if let Some(color_setting) = fbs_get_style_id
        .style_setting()
        .border_diagonal()
        .border_color()
    {
        let mut clr_setting = ColorSetting::default();
        clr_setting.color_setting_type = match color_setting.color_setting_type() {
            Color_setting_type_values::rgb => Rgb,
            Color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_diagonal.border_color = Some(clr_setting);
    }
    border_diagonal.style = match fbs_get_style_id.style_setting().border_diagonal().style() {
        Border_style_values::thin => BorderStyleValues::Thin,
        Border_style_values::thick => BorderStyleValues::Thick,
        _ => BorderStyleValues::None,
    };
    style_setting.border_diagonal = border_diagonal;

    // font
    if let Some(font_family) = fbs_get_style_id.style_setting().font_family() {
        style_setting.font_family = font_family.to_string();
    }
    style_setting.font_size = fbs_get_style_id.style_setting().font_size();
    style_setting.is_bold = fbs_get_style_id.style_setting().is_bold();
    style_setting.is_italic = fbs_get_style_id.style_setting().is_italic();
    style_setting.is_underline = fbs_get_style_id.style_setting().is_underline();
    style_setting.is_double_underline = fbs_get_style_id.style_setting().is_double_underline();
    style_setting.is_wrap_text = fbs_get_style_id.style_setting().is_wrap_text();

    // fill
    style_setting.background_color = fbs_get_style_id
        .style_setting()
        .background_color()
        .map(|item| item.to_string());
    style_setting.foreground_color = fbs_get_style_id
        .style_setting()
        .foreground_color()
        .map(|item| item.to_string());

    // xfs
    style_setting.horizontal_alignment =
        match fbs_get_style_id.style_setting().horizontal_alignment() {
            Horizontal_alignment_values::left => HorizontalAlignmentValues::LEFT,
            Horizontal_alignment_values::center => HorizontalAlignmentValues::CENTER,
            Horizontal_alignment_values::right => HorizontalAlignmentValues::RIGHT,
            Horizontal_alignment_values::justify => HorizontalAlignmentValues::JUSTIFY,
            _ => HorizontalAlignmentValues::None,
        };
    style_setting.vertical_alignment = match fbs_get_style_id.style_setting().vertical_alignment() {
        Vertical_alignment_values::top => VerticalAlignmentValues::Top,
        Vertical_alignment_values::middle => VerticalAlignmentValues::Middle,
        Vertical_alignment_values::bottom => VerticalAlignmentValues::Bottom,
        _ => VerticalAlignmentValues::None,
    };

    match excel.get_style_id_mut(style_setting) {
        Ok(style_id) => {
            let style_id_ptr = Box::into_raw(Box::new(style_id)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_get_style_id_return = Excel_get_style_id_return::create(
                &mut builder,
                &Excel_get_style_id_returnArgs {
                    style_id_ptr: style_id_ptr,
                },
            );
            builder.finish(excel_get_style_id_return, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
///Save the Excel File in provided file path
pub extern "C" fn Excel_save_as(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_save_as =
        match unsafe { root_from_raw::<Excel_save_as>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
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
            let excel_save_as_return = Excel_save_as_return::create(
                &mut builder,
                &Excel_save_as_returnArgs {
                    full_path: Some(full_path_offset),
                },
            );
            builder.finish(excel_save_as_return, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(err) => unsafe { set_error(out_error, &err, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Set the active sheet that opens by default when the workbook is opened
pub extern "C" fn Excel_set_active_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_set_active_sheet>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.set_active_sheet_mut(fbs.sheet_name().map(|s| s.to_string()).unwrap_or_default()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Set the visibility of the workbook window
pub extern "C" fn Excel_set_visibility(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_set_visibility>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.set_visibility_mut(fbs.is_visible()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Minimize or restore the workbook window
pub extern "C" fn Excel_minimize_workbook(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_minimize_workbook>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.minimize_workbook_mut(fbs.is_minimized()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Show or hide the sheet tab bar
pub extern "C" fn Excel_hide_sheet_tabs(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_hide_sheet_tabs>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_sheet_tabs_mut(fbs.hide_tab()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Show or hide the vertical scroll bar
pub extern "C" fn Excel_hide_vertical_scroll(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_hide_vertical_scroll>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_vertical_scroll_mut(fbs.hide_vertical_scroll()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Show or hide the horizontal scroll bar
pub extern "C" fn Excel_hide_horizontal_scroll(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_hide_horizontal_scroll>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_horizontal_scroll_mut(fbs.hide_horizontal_scroll()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Hide a specific sheet in the workbook
pub extern "C" fn Excel_hide_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs = match unsafe {
        root_from_raw::<Excel_hide_sheet>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_sheet_mut(fbs.sheet_name().map(|s| s.to_string()).unwrap_or_default()) {
        Ok(()) => StatusCode::Success as i8,
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::IoError) },
    }
}
