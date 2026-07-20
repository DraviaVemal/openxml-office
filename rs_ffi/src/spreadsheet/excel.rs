use crate::{
    openxml_office_fbs::spreadsheet::{
        border_style_values, color_setting_type_values, excel_add_sheet, excel_add_sheet_return,
        excel_add_sheet_returnArgs, excel_create, excel_create_return, excel_create_returnArgs,
        excel_get_sheet_return, excel_get_sheet_returnArgs, excel_get_style_id,
        excel_get_style_id_return, excel_get_style_id_returnArgs, excel_list_sheet,
        excel_list_sheet_return, excel_list_sheet_returnArgs, excel_rename_sheet, excel_save_as,
        excel_save_as_return, excel_save_as_returnArgs, horizontal_alignment_values,
        number_format_values, vertical_alignment_values,
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
pub extern "C" fn excel_create(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_excel_create =
        match unsafe { root_from_raw::<excel_create>(in_buffer, in_buffer_size, out_error) } {
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
            let excel_create_return = excel_create_return::create(
                &mut builder,
                &excel_create_returnArgs {
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
pub extern "C" fn excel_add_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_add_sheet =
        match unsafe { root_from_raw::<excel_add_sheet>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let excel_ptr = fbs_add_sheet.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.add_sheet_mut(fbs_add_sheet.sheet_name().map(|item| item.to_string())) {
        Ok(worksheet) => {
            let worksheet_ptr = Box::into_raw(Box::new(worksheet)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_add_sheet = excel_add_sheet_return::create(
                &mut builder,
                &excel_add_sheet_returnArgs {
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
pub extern "C" fn excel_rename_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_excel_rename_sheet = match unsafe {
        root_from_raw::<excel_rename_sheet>(in_buffer, in_buffer_size, out_error)
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
pub extern "C" fn excel_get_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_add_sheet =
        match unsafe { root_from_raw::<excel_add_sheet>(in_buffer, in_buffer_size, out_error) } {
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
            let excel_get_sheet_return = excel_get_sheet_return::create(
                &mut builder,
                &excel_get_sheet_returnArgs {
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
pub extern "C" fn excel_list_sheet_name(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_list_sheet_name =
        match unsafe { root_from_raw::<excel_list_sheet>(in_buffer, in_buffer_size, out_error) } {
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
            let excel_list_sheet_return = excel_list_sheet_return::create(
                &mut builder,
                &excel_list_sheet_returnArgs {
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
pub extern "C" fn excel_get_style_id(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_get_style_id = match unsafe {
        root_from_raw::<excel_get_style_id>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = fbs_get_style_id.excel_ptr() as *mut Excel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };

    let mut style_setting = CellStyleSetting::default();
    // num format
    style_setting.number_format = match fbs_get_style_id.style_setting().number_format() {
        number_format_values::integer => NumberFormatValues::Integer,
        number_format_values::decimal_two_places => NumberFormatValues::DecimalTwoPlaces,
        number_format_values::thousands_separator => NumberFormatValues::ThousandsSeparator,
        number_format_values::thousands_separator_two_decimals => {
            NumberFormatValues::ThousandsSeparatorTwoDecimals
        }
        number_format_values::currency_no_decimals => NumberFormatValues::CurrencyNoDecimals,
        number_format_values::currency_no_decimals_red => NumberFormatValues::CurrencyNoDecimalsRed,
        number_format_values::currency_two_decimals => NumberFormatValues::CurrencyTwoDecimals,
        number_format_values::currency_two_decimals_red => {
            NumberFormatValues::CurrencyTwoDecimalsRed
        }
        number_format_values::percentage => NumberFormatValues::Percentage,
        number_format_values::percentage_two_decimals => NumberFormatValues::PercentageTwoDecimals,
        number_format_values::scientific => NumberFormatValues::Scientific,
        number_format_values::fraction_one_digit => NumberFormatValues::FractionOneDigit,
        number_format_values::fraction_two_digits => NumberFormatValues::FractionTwoDigits,
        number_format_values::date_mmddyy => NumberFormatValues::DateMMDDYY,
        number_format_values::date_dmmmyy => NumberFormatValues::DateDMmmYY,
        number_format_values::date_dmmm => NumberFormatValues::DateDMmm,
        number_format_values::date_mmmyy => NumberFormatValues::DateMmmYY,
        number_format_values::time_12_hour => NumberFormatValues::Time12Hour,
        number_format_values::time_12_hour_with_seconds => {
            NumberFormatValues::Time12HourWithSeconds
        }
        number_format_values::time_24_hour => NumberFormatValues::Time24Hour,
        number_format_values::time_24_hour_with_seconds => {
            NumberFormatValues::Time24HourWithSeconds
        }
        number_format_values::date_time_mmddyy => NumberFormatValues::DateTimeMMDDYY,
        number_format_values::accounting_no_decimals => NumberFormatValues::AccountingNoDecimals,
        number_format_values::accounting_no_decimals_red => {
            NumberFormatValues::AccountingNoDecimalsRed
        }
        number_format_values::accounting_two_decimals => NumberFormatValues::AccountingTwoDecimals,
        number_format_values::accounting_two_decimals_red => {
            NumberFormatValues::AccountingTwoDecimalsRed
        }
        number_format_values::accounting_negative_in_parentheses => {
            NumberFormatValues::AccountingNegativeInParentheses
        }
        number_format_values::accounting_two_decimals_negative_in_parentheses => {
            NumberFormatValues::AccountingTwoDecimalsNegativeInParentheses
        }
        number_format_values::accounting_aligned_symbols => {
            NumberFormatValues::AccountingAlignedSymbols
        }
        number_format_values::accounting_aligned_symbols_two_decimals => {
            NumberFormatValues::AccountingAlignedSymbolsTwoDecimals
        }
        number_format_values::time_minutes_seconds => NumberFormatValues::TimeMinutesSeconds,
        number_format_values::time_hours_minutes_seconds => {
            NumberFormatValues::TimeHoursMinutesSeconds
        }
        number_format_values::elapsed_time_with_fractions => {
            NumberFormatValues::ElapsedTimeWithFractions
        }
        number_format_values::scientific_one_decimal => NumberFormatValues::ScientificOneDecimal,
        number_format_values::text_format => NumberFormatValues::TextFormat,
        number_format_values::custom => NumberFormatValues::Custom,
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
            color_setting_type_values::rgb => Rgb,
            color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_left.border_color = Some(clr_setting);
    }
    border_left.style = match fbs_get_style_id.style_setting().border_left().style() {
        border_style_values::thin => BorderStyleValues::Thin,
        border_style_values::thick => BorderStyleValues::Thick,
        _ => BorderStyleValues::None,
    };
    style_setting.border_left = border_left;

    let mut border_top = BorderSetting::default();
    if let Some(color_setting) = fbs_get_style_id.style_setting().border_top().border_color() {
        let mut clr_setting = ColorSetting::default();
        clr_setting.color_setting_type = match color_setting.color_setting_type() {
            color_setting_type_values::rgb => Rgb,
            color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_top.border_color = Some(clr_setting);
    }
    border_top.style = match fbs_get_style_id.style_setting().border_top().style() {
        border_style_values::thin => BorderStyleValues::Thin,
        border_style_values::thick => BorderStyleValues::Thick,
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
            color_setting_type_values::rgb => Rgb,
            color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_right.border_color = Some(clr_setting);
    }
    border_right.style = match fbs_get_style_id.style_setting().border_right().style() {
        border_style_values::thin => BorderStyleValues::Thin,
        border_style_values::thick => BorderStyleValues::Thick,
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
            color_setting_type_values::rgb => Rgb,
            color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_bottom.border_color = Some(clr_setting);
    }
    border_bottom.style = match fbs_get_style_id.style_setting().border_bottom().style() {
        border_style_values::thin => BorderStyleValues::Thin,
        border_style_values::thick => BorderStyleValues::Thick,
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
            color_setting_type_values::rgb => Rgb,
            color_setting_type_values::theme => Theme,
            _ => Indexed,
        };
        border_diagonal.border_color = Some(clr_setting);
    }
    border_diagonal.style = match fbs_get_style_id.style_setting().border_diagonal().style() {
        border_style_values::thin => BorderStyleValues::Thin,
        border_style_values::thick => BorderStyleValues::Thick,
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
            horizontal_alignment_values::left => HorizontalAlignmentValues::LEFT,
            horizontal_alignment_values::center => HorizontalAlignmentValues::CENTER,
            horizontal_alignment_values::right => HorizontalAlignmentValues::RIGHT,
            horizontal_alignment_values::justify => HorizontalAlignmentValues::JUSTIFY,
            _ => HorizontalAlignmentValues::None,
        };
    style_setting.vertical_alignment = match fbs_get_style_id.style_setting().vertical_alignment() {
        vertical_alignment_values::top => VerticalAlignmentValues::Top,
        vertical_alignment_values::middle => VerticalAlignmentValues::Middle,
        vertical_alignment_values::bottom => VerticalAlignmentValues::Bottom,
        _ => VerticalAlignmentValues::None,
    };

    match excel.get_style_id_mut(style_setting) {
        Ok(style_id) => {
            let style_id_ptr = Box::into_raw(Box::new(style_id)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_get_style_id_return = excel_get_style_id_return::create(
                &mut builder,
                &excel_get_style_id_returnArgs {
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
pub extern "C" fn excel_save_as(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_save_as =
        match unsafe { root_from_raw::<excel_save_as>(in_buffer, in_buffer_size, out_error) } {
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
            let excel_save_as_return = excel_save_as_return::create(
                &mut builder,
                &excel_save_as_returnArgs {
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
