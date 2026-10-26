use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Weak,
};

use crate::{
    global_2007::files::OfficeDocument,
    global::traits::Enum,
    spreadsheet_2007::services::style::enum_items::{
        BorderStyleValues, ColorSettingTypeValues, FontSchemeValues, HorizontalAlignmentValues,
        NumberFormatValues, PatternTypeValues, VerticalAlignmentValues,
    },
};
use draviavemal_xml_rs::XmlDocument;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct ColorSetting {
    pub color_setting_type: ColorSettingTypeValues,
    pub value: String,
}

impl Default for ColorSetting {
    fn default() -> Self {
        Self {
            color_setting_type: ColorSettingTypeValues::Theme,
            value: "1".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct BorderSetting {
    pub border_color: Option<ColorSetting>,
    pub style: BorderStyleValues,
}

impl Default for BorderSetting {
    fn default() -> Self {
        Self {
            border_color: None,
            style: BorderStyleValues::None,
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub(crate) struct BorderStyle {
    pub(crate) id: u32,
    pub(crate) bottom: BorderSetting,
    pub(crate) left: BorderSetting,
    pub(crate) right: BorderSetting,
    pub(crate) top: BorderSetting,
    pub(crate) diagonal: BorderSetting,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            id: 0,
            bottom: BorderSetting::default(),
            left: BorderSetting::default(),
            right: BorderSetting::default(),
            top: BorderSetting::default(),
            diagonal: BorderSetting::default(),
        }
    }
}

#[derive(Debug, Hash)]
pub(crate) struct NumberFormat {
    pub(crate) format_id: usize,
    pub(crate) format_type: NumberFormatValues,
    pub(crate) format_code: String,
}

impl Default for NumberFormat {
    fn default() -> Self {
        Self {
            format_id: 0,
            format_type: NumberFormatValues::General,
            format_code: NumberFormatValues::get_string(NumberFormatValues::General),
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub(crate) struct FontStyle {
    pub(crate) name: String,
    pub(crate) size: u8,
    pub(crate) color: ColorSetting,
    pub(crate) family: u32,
    pub(crate) font_scheme: FontSchemeValues,
    pub(crate) is_bold: bool,
    pub(crate) is_double_underline: bool,
    pub(crate) is_italic: bool,
    pub(crate) is_underline: bool,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            name: "Calibri".to_string(),
            size: 11,
            color: ColorSetting {
                value: "1".to_string(),
                ..ColorSetting::default()
            },
            family: 2,
            font_scheme: FontSchemeValues::None,
            is_bold: false,
            is_double_underline: false,
            is_italic: false,
            is_underline: false,
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub(crate) struct CellXfs {
    pub(crate) format_id: u16,
    pub(crate) number_format_id: u16,
    pub(crate) font_id: u16,
    pub(crate) fill_id: u16,
    pub(crate) border_id: u16,
    pub(crate) apply_font: u8,
    pub(crate) apply_alignment: u8,
    pub(crate) apply_fill: u8,
    pub(crate) apply_border: u8,
    pub(crate) apply_number_format: u8,
    pub(crate) apply_protection: u8,
    pub(crate) is_wrap_text: u8,
    pub(crate) horizontal_alignment: HorizontalAlignmentValues,
    pub(crate) vertical_alignment: VerticalAlignmentValues,
}

impl Default for CellXfs {
    fn default() -> Self {
        Self {
            format_id: 0,
            number_format_id: 0,
            font_id: 0,
            fill_id: 0,
            border_id: 0,
            apply_font: 0,
            apply_alignment: 0,
            apply_fill: 0,
            apply_border: 0,
            apply_number_format: 0,
            apply_protection: 0,
            is_wrap_text: 0,
            horizontal_alignment: HorizontalAlignmentValues::None,
            vertical_alignment: VerticalAlignmentValues::None,
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub(crate) struct FillStyle {
    pub(crate) pattern_type: PatternTypeValues,
    pub(crate) background_color: Option<ColorSetting>,
    pub(crate) foreground_color: Option<ColorSetting>,
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            foreground_color: None,
            pattern_type: PatternTypeValues::None,
        }
    }
}

/// Get Column Cell Input Combined for styling
#[derive(Debug, Hash)]
pub struct CellStyleSetting {
    // num format
    pub number_format: NumberFormatValues,
    pub custom_number_format: Option<String>,
    // border
    pub border_left: BorderSetting,
    pub border_top: BorderSetting,
    pub border_right: BorderSetting,
    pub border_bottom: BorderSetting,
    pub border_diagonal: BorderSetting,
    // font
    pub font_family: String,
    pub font_size: u8,
    pub text_color: ColorSetting,
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_underline: bool,
    pub is_double_underline: bool,
    pub is_wrap_text: bool,
    // fill
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
    pub(crate) pattern_type: PatternTypeValues,
    // xfs
    pub horizontal_alignment: HorizontalAlignmentValues,
    pub vertical_alignment: VerticalAlignmentValues,
    // mis
    pub(crate) protect: Option<()>,
}

impl CellStyleSetting {
    pub fn set_number_format(mut self, number_format: NumberFormatValues) -> CellStyleSetting {
        self.number_format = number_format;
        self
    }
    pub fn set_custom_number_format(
        mut self,
        custom_number_format: Option<String>,
    ) -> CellStyleSetting {
        self.custom_number_format = custom_number_format;
        self
    }
    pub fn set_border_left(mut self, border_left: BorderSetting) -> CellStyleSetting {
        self.border_left = border_left;
        self
    }
    pub fn set_border_top(mut self, border_top: BorderSetting) -> CellStyleSetting {
        self.border_top = border_top;
        self
    }
    pub fn set_border_right(mut self, border_right: BorderSetting) -> CellStyleSetting {
        self.border_right = border_right;
        self
    }
    pub fn set_border_bottom(mut self, border_bottom: BorderSetting) -> CellStyleSetting {
        self.border_bottom = border_bottom;
        self
    }
    pub fn set_border_diagonal(mut self, border_diagonal: BorderSetting) -> CellStyleSetting {
        self.border_diagonal = border_diagonal;
        self
    }
    pub fn set_font_family(mut self, font_family: String) -> CellStyleSetting {
        self.font_family = font_family;
        self
    }
    pub fn set_font_size(mut self, font_size: u8) -> CellStyleSetting {
        self.font_size = font_size;
        self
    }
    pub fn set_text_color(mut self, text_color: ColorSetting) -> CellStyleSetting {
        self.text_color = text_color;
        self
    }
    pub fn set_is_bold(mut self, is_bold: bool) -> CellStyleSetting {
        self.is_bold = is_bold;
        self
    }
    pub fn set_is_italic(mut self, is_italic: bool) -> CellStyleSetting {
        self.is_italic = is_italic;
        self
    }
    pub fn set_is_underline(mut self, is_underline: bool) -> CellStyleSetting {
        self.is_underline = is_underline;
        self
    }
    pub fn set_is_double_underline(mut self, is_double_underline: bool) -> CellStyleSetting {
        self.is_double_underline = is_double_underline;
        self
    }
    pub fn set_is_wrap_text(mut self, is_wrap_text: bool) -> CellStyleSetting {
        self.is_wrap_text = is_wrap_text;
        self
    }
    pub fn set_background_color(mut self, background_color: Option<String>) -> CellStyleSetting {
        self.background_color = background_color;
        self
    }
    pub fn set_foreground_color(mut self, foreground_color: Option<String>) -> CellStyleSetting {
        self.foreground_color = foreground_color;
        self
    }
    pub fn set_horizontal_alignment(
        mut self,
        horizontal_alignment: HorizontalAlignmentValues,
    ) -> CellStyleSetting {
        self.horizontal_alignment = horizontal_alignment;
        self
    }
    pub fn set_vertical_alignment(
        mut self,
        vertical_alignment: VerticalAlignmentValues,
    ) -> CellStyleSetting {
        self.vertical_alignment = vertical_alignment;
        self
    }
}

impl Default for CellStyleSetting {
    fn default() -> Self {
        Self {
            // number format
            number_format: NumberFormatValues::General,
            custom_number_format: None,
            // border
            border_left: BorderSetting::default(),
            border_top: BorderSetting::default(),
            border_right: BorderSetting::default(),
            border_bottom: BorderSetting::default(),
            border_diagonal: BorderSetting::default(),
            // font
            font_family: "Calibri".to_string(),
            font_size: 11,
            is_bold: false,
            is_italic: false,
            is_underline: false,
            is_double_underline: false,
            is_wrap_text: false,
            text_color: ColorSetting {
                color_setting_type: ColorSettingTypeValues::Theme,
                value: "1".to_string(),
            },
            // fill
            background_color: None,
            foreground_color: None,
            pattern_type: PatternTypeValues::None,
            // xfs
            horizontal_alignment: HorizontalAlignmentValues::None,
            vertical_alignment: VerticalAlignmentValues::None,
            // mis
            protect: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StyleId {
    pub(crate) id: u32,
}

impl StyleId {
    pub(crate) fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub(crate) struct StylePart {
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) file_path: String,
    pub(crate) cache_id: HashMap<u64, u32>,
    pub(crate) cache_order: VecDeque<u64>,
    pub(crate) cache_capacity: u8,
    pub(crate) number_format_collection: Vec<(u64, NumberFormat)>,
    pub(crate) font_collection: Vec<(u64, FontStyle)>,
    pub(crate) fill_collection: Vec<(u64, FillStyle)>,
    pub(crate) border_collection: Vec<(u64, BorderStyle)>,
    pub(crate) cell_style_xfs_collection: Vec<(u64, CellXfs)>,
    pub(crate) cell_xfs_collection: Vec<(u64, CellXfs)>,
}
