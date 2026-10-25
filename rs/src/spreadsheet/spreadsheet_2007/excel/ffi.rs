use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::c_char,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    ffi::{openxml_office_fbs::spreadsheet::*, root_from_raw, set_error, write_buffer, StatusCode},
    spreadsheet_2007::{
        models::{
            BorderSetting, BorderStyleValues, CellStyleSetting, ColorSetting,
            ColorSettingTypeValues::{Indexed, Rgb, Theme},
            HorizontalAlignmentValues, NumberFormatValues, StyleId, VerticalAlignmentValues,
        },
        Excel, ExcelPropertiesModel, WorkSheet, WorkbookSettings,
    },
};

#[derive(Debug, Default)]
pub(crate) struct StyleIdArena {
    handles: RefCell<StyleIdHandles>,
}

#[derive(Debug, Default)]
struct StyleIdHandles {
    by_id: HashMap<u32, Box<StyleId>>,
    id_by_pointer: HashMap<u64, u32>,
}

impl StyleIdArena {
    pub(crate) fn insert(&self, style_id: StyleId) -> u64 {
        let mut handles = self.handles.borrow_mut();
        let style_id_value = style_id.get_id();
        if let Some(handle) = handles.by_id.get(&style_id_value) {
            return (&**handle as *const StyleId) as u64;
        }
        let handle = Box::new(style_id);
        let pointer = (&*handle as *const StyleId) as u64;
        handles.by_id.insert(style_id_value, handle);
        handles.id_by_pointer.insert(pointer, style_id_value);
        pointer
    }

    fn resolve(&self, pointer: u64) -> Option<StyleId> {
        let handles = self.handles.borrow();
        let style_id_value = handles.id_by_pointer.get(&pointer)?;
        handles.by_id.get(style_id_value).map(|style_id| **style_id)
    }

    pub(crate) fn resolve_optional(&self, pointer: Option<u64>) -> anyhow::Result<Option<StyleId>> {
        pointer
            .map(|pointer| {
                self.resolve(pointer)
                    .ok_or_else(|| anyhow::Error::msg("invalid or expired StyleId handle"))
            })
            .transpose()
    }
}

#[derive(Debug)]
struct FfiExcel {
    inner: Excel,
    style_ids: Rc<StyleIdArena>,
}

impl FfiExcel {
    fn new(inner: Excel) -> Self {
        Self {
            inner,
            style_ids: Rc::default(),
        }
    }

    fn worksheet(&self, inner: WorkSheet) -> FfiWorksheet {
        FfiWorksheet {
            inner,
            style_ids: Rc::clone(&self.style_ids),
        }
    }

    fn into_inner(self) -> Excel {
        self.inner
    }
}

impl Deref for FfiExcel {
    type Target = Excel;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for FfiExcel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug)]
pub(crate) struct FfiWorksheet {
    inner: WorkSheet,
    pub(crate) style_ids: Rc<StyleIdArena>,
}

impl Deref for FfiWorksheet {
    type Target = WorkSheet;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for FfiWorksheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FfiWorksheet {
    pub(crate) fn into_inner(self) -> WorkSheet {
        self.inner
    }
}

#[no_mangle]
/// Create an Excel object.
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
            let excel_ptr = Box::into_raw(Box::new(FfiExcel::new(excel))) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_create_return = Excel_create_return::create(
                &mut builder,
                &Excel_create_returnArgs {
                    excel_ptr: excel_ptr,
                },
            );
            builder.finish(excel_create_return, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Add a worksheet.
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
    let excel_ptr = fbs_add_sheet.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.add_sheet_mut(fbs_add_sheet.sheet_name().map(|item| item.to_string())) {
        Ok(worksheet) => {
            let worksheet_ptr = Box::into_raw(Box::new(excel.worksheet(worksheet))) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_add_sheet = Excel_add_sheet_return::create(
                &mut builder,
                &Excel_add_sheet_returnArgs {
                    worksheet_ptr: worksheet_ptr,
                },
            );
            builder.finish(excel_add_sheet, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Rename a worksheet.
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
    let excel_ptr = fbs_excel_rename_sheet.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.rename_sheet_name_ref_mut(
        fbs_excel_rename_sheet.old_sheet_name().unwrap_or_default(),
        fbs_excel_rename_sheet.new_sheet_name().unwrap_or_default(),
    ) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Get a worksheet.
pub extern "C" fn Excel_get_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request =
        match unsafe { root_from_raw::<Excel_get_sheet>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.get_worksheet_ref_mut(request.sheet_name().unwrap_or_default()) {
        Ok(worksheet) => {
            let worksheet_ptr = Box::into_raw(Box::new(excel.worksheet(worksheet))) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_get_sheet_return = Excel_get_sheet_return::create(
                &mut builder,
                &Excel_get_sheet_returnArgs {
                    worksheet_ptr: worksheet_ptr,
                },
            );
            builder.finish(excel_get_sheet_return, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// List worksheet names.
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
    let excel_ptr = fbs_list_sheet_name.excel_ptr() as *mut FfiExcel;
    let excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let mut sheet_names_offsets = Vec::new();
    match excel.for_each_sheet_name(|name| {
        sheet_names_offsets.push(builder.create_string(name));
    }) {
        Ok(()) => {
            let sheet_names_vector = builder.create_vector(&sheet_names_offsets);
            let excel_list_sheet_return = Excel_list_sheet_return::create(
                &mut builder,
                &Excel_list_sheet_returnArgs {
                    sheet_names: Some(sheet_names_vector),
                },
            );
            builder.finish(excel_list_sheet_return, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Get or create a style ID.
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
    let excel_ptr = fbs_get_style_id.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };

    let mut style_setting = CellStyleSetting::default();
    // Number format
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

    // Borders
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

    // Font
    if let Some(font_family) = fbs_get_style_id.style_setting().font_family() {
        style_setting.font_family = font_family.to_string();
    }
    style_setting.font_size = fbs_get_style_id.style_setting().font_size();
    style_setting.is_bold = fbs_get_style_id.style_setting().is_bold();
    style_setting.is_italic = fbs_get_style_id.style_setting().is_italic();
    style_setting.is_underline = fbs_get_style_id.style_setting().is_underline();
    style_setting.is_double_underline = fbs_get_style_id.style_setting().is_double_underline();
    style_setting.is_wrap_text = fbs_get_style_id.style_setting().is_wrap_text();

    // Fill
    style_setting.background_color = fbs_get_style_id
        .style_setting()
        .background_color()
        .map(|item| item.to_string());
    style_setting.foreground_color = fbs_get_style_id
        .style_setting()
        .foreground_color()
        .map(|item| item.to_string());

    // Cell formats
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
            let style_id_ptr = excel.style_ids.insert(style_id);
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let excel_get_style_id_return = Excel_get_style_id_return::create(
                &mut builder,
                &Excel_get_style_id_returnArgs { style_id_ptr },
            );
            builder.finish(excel_get_style_id_return, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Save the workbook.
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
    let excel_ptr = fbs_save_as.excel_ptr() as *mut FfiExcel;
    let excel = unsafe { Box::from_raw(excel_ptr) }.into_inner();
    match excel.save_as(fbs_save_as.file_name().unwrap_or_default()) {
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
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Set the active worksheet.
pub extern "C" fn Excel_set_active_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_set_active_sheet>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.set_active_sheet_ref_mut(request.sheet_name().unwrap_or_default()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Set workbook visibility.
pub extern "C" fn Excel_set_visibility(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_set_visibility>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.set_visibility_mut(request.is_visible()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Minimize or restore the workbook.
pub extern "C" fn Excel_minimize_workbook(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_minimize_workbook>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.minimize_workbook_mut(request.is_minimized()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Show or hide sheet tabs.
pub extern "C" fn Excel_hide_sheet_tabs(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_hide_sheet_tabs>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_sheet_tabs_mut(request.hide_tab()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Show or hide the vertical scrollbar.
pub extern "C" fn Excel_hide_vertical_scroll(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_hide_vertical_scroll>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_vertical_scroll_mut(request.hide_vertical_scroll()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Show or hide the horizontal scrollbar.
pub extern "C" fn Excel_hide_horizontal_scroll(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_hide_horizontal_scroll>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_horizontal_scroll_mut(request.hide_horizontal_scroll()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
/// Hide a worksheet.
pub extern "C" fn Excel_hide_sheet(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request =
        match unsafe { root_from_raw::<Excel_hide_sheet>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.hide_sheet_ref_mut(request.sheet_name().unwrap_or_default()) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}

#[no_mangle]
pub extern "C" fn Excel_set_workbook_settings(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> i8 {
    let request = match unsafe {
        root_from_raw::<Excel_set_workbook_settings>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let excel_ptr = request.excel_ptr() as *mut FfiExcel;
    let mut excel = unsafe { ManuallyDrop::new(Box::from_raw(excel_ptr)) };
    match excel.set_workbook_settings_mut(WorkbookSettings {
        active_sheet: request.active_sheet().map(str::to_owned),
        is_visible: request.is_visible(),
        is_minimized: request.is_minimized(),
        hide_tab: request.hide_tab(),
        hide_vertical_scroll: request.hide_vertical_scroll(),
        hide_horizontal_scroll: request.hide_horizontal_scroll(),
        hide_sheet: request.hide_sheet().map(str::to_owned),
    }) {
        Ok(()) => StatusCode::Success as i8,
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}
