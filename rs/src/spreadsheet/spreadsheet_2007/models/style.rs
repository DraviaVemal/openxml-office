use crate::global_2007::traits::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberFormatValues {
    General,
    Integer,
    DecimalTwoPlaces,
    ThousandsSeparator,
    ThousandsSeparatorTwoDecimals,
    CurrencyNoDecimals,
    CurrencyNoDecimalsRed,
    CurrencyTwoDecimals,
    CurrencyTwoDecimalsRed,
    Percentage,
    PercentageTwoDecimals,
    Scientific,
    FractionOneDigit,
    FractionTwoDigits,
    DateMMDDYY,
    DateDMmmYY,
    DateDMmm,
    DateMmmYY,
    Time12Hour,
    Time12HourWithSeconds,
    Time24Hour,
    Time24HourWithSeconds,
    DateTimeMMDDYY,
    AccountingNoDecimals,
    AccountingNoDecimalsRed,
    AccountingTwoDecimals,
    AccountingTwoDecimalsRed,
    AccountingNegativeInParentheses,
    AccountingTwoDecimalsNegativeInParentheses,
    AccountingAlignedSymbols,
    AccountingAlignedSymbolsTwoDecimals,
    TimeMinutesSeconds,
    TimeHoursMinutesSeconds,
    ElapsedTimeWithFractions,
    ScientificOneDecimal,
    TextFormat,
    Custom,
}

impl Enum<NumberFormatValues> for NumberFormatValues {
    fn get_string(input_enum: NumberFormatValues) -> String {
        match input_enum {
            NumberFormatValues::General => "0".to_string(),
            NumberFormatValues::Integer => "1".to_string(),
            NumberFormatValues::DecimalTwoPlaces => "2".to_string(),
            NumberFormatValues::ThousandsSeparator => "3".to_string(),
            NumberFormatValues::ThousandsSeparatorTwoDecimals => "4".to_string(),
            NumberFormatValues::CurrencyNoDecimals => "5".to_string(),
            NumberFormatValues::CurrencyNoDecimalsRed => "6".to_string(),
            NumberFormatValues::CurrencyTwoDecimals => "7".to_string(),
            NumberFormatValues::CurrencyTwoDecimalsRed => "8".to_string(),
            NumberFormatValues::Percentage => "9".to_string(),
            NumberFormatValues::PercentageTwoDecimals => "10".to_string(),
            NumberFormatValues::Scientific => "11".to_string(),
            NumberFormatValues::FractionOneDigit => "12".to_string(),
            NumberFormatValues::FractionTwoDigits => "13".to_string(),
            NumberFormatValues::DateMMDDYY => "14".to_string(),
            NumberFormatValues::DateDMmmYY => "15".to_string(),
            NumberFormatValues::DateDMmm => "16".to_string(),
            NumberFormatValues::DateMmmYY => "17".to_string(),
            NumberFormatValues::Time12Hour => "18".to_string(),
            NumberFormatValues::Time12HourWithSeconds => "19".to_string(),
            NumberFormatValues::Time24Hour => "20".to_string(),
            NumberFormatValues::Time24HourWithSeconds => "21".to_string(),
            NumberFormatValues::DateTimeMMDDYY => "22".to_string(),
            NumberFormatValues::AccountingNoDecimals => "37".to_string(),
            NumberFormatValues::AccountingNoDecimalsRed => "38".to_string(),
            NumberFormatValues::AccountingTwoDecimals => "39".to_string(),
            NumberFormatValues::AccountingTwoDecimalsRed => "40".to_string(),
            NumberFormatValues::AccountingNegativeInParentheses => "41".to_string(),
            NumberFormatValues::AccountingTwoDecimalsNegativeInParentheses => "42".to_string(),
            NumberFormatValues::AccountingAlignedSymbols => "43".to_string(),
            NumberFormatValues::AccountingAlignedSymbolsTwoDecimals => "44".to_string(),
            NumberFormatValues::TimeMinutesSeconds => "45".to_string(),
            NumberFormatValues::TimeHoursMinutesSeconds => "46".to_string(),
            NumberFormatValues::ElapsedTimeWithFractions => "47".to_string(),
            NumberFormatValues::ScientificOneDecimal => "48".to_string(),
            NumberFormatValues::TextFormat => "49".to_string(),
            NumberFormatValues::Custom => "164".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> Self {
        match input_string {
            "0" => NumberFormatValues::General,
            "1" => NumberFormatValues::Integer,
            "2" => NumberFormatValues::DecimalTwoPlaces,
            "3" => NumberFormatValues::ThousandsSeparator,
            "4" => NumberFormatValues::ThousandsSeparatorTwoDecimals,
            "5" => NumberFormatValues::CurrencyNoDecimals,
            "6" => NumberFormatValues::CurrencyNoDecimalsRed,
            "7" => NumberFormatValues::CurrencyTwoDecimals,
            "8" => NumberFormatValues::CurrencyTwoDecimalsRed,
            "9" => NumberFormatValues::Percentage,
            "10" => NumberFormatValues::PercentageTwoDecimals,
            "11" => NumberFormatValues::Scientific,
            "12" => NumberFormatValues::FractionOneDigit,
            "13" => NumberFormatValues::FractionTwoDigits,
            "14" => NumberFormatValues::DateMMDDYY,
            "15" => NumberFormatValues::DateDMmmYY,
            "16" => NumberFormatValues::DateDMmm,
            "17" => NumberFormatValues::DateMmmYY,
            "18" => NumberFormatValues::Time12Hour,
            "19" => NumberFormatValues::Time12HourWithSeconds,
            "20" => NumberFormatValues::Time24Hour,
            "21" => NumberFormatValues::Time24HourWithSeconds,
            "22" => NumberFormatValues::DateTimeMMDDYY,
            "37" => NumberFormatValues::AccountingNoDecimals,
            "38" => NumberFormatValues::AccountingNoDecimalsRed,
            "39" => NumberFormatValues::AccountingTwoDecimals,
            "40" => NumberFormatValues::AccountingTwoDecimalsRed,
            "41" => NumberFormatValues::AccountingNegativeInParentheses,
            "42" => NumberFormatValues::AccountingTwoDecimalsNegativeInParentheses,
            "43" => NumberFormatValues::AccountingAlignedSymbols,
            "44" => NumberFormatValues::AccountingAlignedSymbolsTwoDecimals,
            "45" => NumberFormatValues::TimeMinutesSeconds,
            "46" => NumberFormatValues::TimeHoursMinutesSeconds,
            "47" => NumberFormatValues::ElapsedTimeWithFractions,
            "48" => NumberFormatValues::ScientificOneDecimal,
            "49" => NumberFormatValues::TextFormat,
            _ => NumberFormatValues::Custom,
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub(crate) enum FontSchemeValues {
    None,
    Minor,
    Major,
}

impl Enum<FontSchemeValues> for FontSchemeValues {
    fn get_string(input_enum: FontSchemeValues) -> String {
        match input_enum {
            FontSchemeValues::Major => "major".to_string(),
            FontSchemeValues::Minor => "minor".to_string(),
            FontSchemeValues::None => "none".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> Self {
        match input_string {
            "major" => FontSchemeValues::Major,
            "minor" => FontSchemeValues::Minor,
            _ => FontSchemeValues::None,
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub enum PatternTypeValues {
    None,
    Gray125,
    Solid,
}

impl Enum<PatternTypeValues> for PatternTypeValues {
    fn get_string(input_enum: PatternTypeValues) -> String {
        match input_enum {
            PatternTypeValues::Solid => "solid".to_string(),
            PatternTypeValues::Gray125 => "gray125".to_string(),
            PatternTypeValues::None => "none".to_string(),
        }
    }

    fn get_enum(input_string: &str) -> PatternTypeValues {
        match input_string {
            "solid" => PatternTypeValues::Solid,
            "gray125" => PatternTypeValues::Gray125,
            _ => PatternTypeValues::None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash)]
pub enum BorderStyleValues {
    None,
    Thin,
    Thick,
    Dotted,
    Double,
    Dashed,
    DashDot,
    DashDotDot,
    Medium,
    MediumDashed,
    MediumDashDot,
    MediumDashDotDot,
    SlantDashDot,
    Hair,
}
impl Enum<BorderStyleValues> for BorderStyleValues {
    fn get_string(input_enum: BorderStyleValues) -> String {
        match input_enum {
            BorderStyleValues::DashDot => "dashDot".to_string(),
            BorderStyleValues::DashDotDot => "dashDotDot".to_string(),
            BorderStyleValues::Dashed => "dashed".to_string(),
            BorderStyleValues::Dotted => "dotted".to_string(),
            BorderStyleValues::Double => "double".to_string(),
            BorderStyleValues::Hair => "hair".to_string(),
            BorderStyleValues::Medium => "medium".to_string(),
            BorderStyleValues::MediumDashDot => "mediumDashDot".to_string(),
            BorderStyleValues::MediumDashDotDot => "mediumDashDotDot".to_string(),
            BorderStyleValues::MediumDashed => "mediumDashed".to_string(),
            BorderStyleValues::SlantDashDot => "slantDashDot".to_string(),
            BorderStyleValues::Thick => "thick".to_string(),
            BorderStyleValues::Thin => "thin".to_string(),
            BorderStyleValues::None => "none".to_string(),
        }
    }

    fn get_enum(input_string: &str) -> BorderStyleValues {
        match input_string {
            "dashDot" => BorderStyleValues::DashDot,
            "dashDotDot" => BorderStyleValues::DashDotDot,
            "dashed" => BorderStyleValues::Dashed,
            "dotted" => BorderStyleValues::Dotted,
            "double" => BorderStyleValues::Double,
            "hair" => BorderStyleValues::Hair,
            "medium" => BorderStyleValues::Medium,
            "mediumDashDot" => BorderStyleValues::MediumDashDot,
            "mediumDashDotDot" => BorderStyleValues::MediumDashDotDot,
            "mediumDashed" => BorderStyleValues::MediumDashed,
            "slantDashDot" => BorderStyleValues::SlantDashDot,
            "thick" => BorderStyleValues::Thick,
            "thin" => BorderStyleValues::Thin,
            _ => BorderStyleValues::None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub enum ColorSettingTypeValues {
    Indexed,
    Theme,
    Rgb,
}

impl Enum<ColorSettingTypeValues> for ColorSettingTypeValues {
    fn get_string(input_enum: ColorSettingTypeValues) -> String {
        match input_enum {
            ColorSettingTypeValues::Theme => "theme".to_string(),
            ColorSettingTypeValues::Rgb => "rgb".to_string(),
            ColorSettingTypeValues::Indexed => "indexed".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> ColorSettingTypeValues {
        match input_string {
            "theme" => ColorSettingTypeValues::Theme,
            "rgb" => ColorSettingTypeValues::Rgb,
            _ => ColorSettingTypeValues::Indexed,
        }
    }
}

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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash)]
pub enum HorizontalAlignmentValues {
    None,
    LEFT,
    CENTER,
    RIGHT,
    JUSTIFY,
}

impl Enum<HorizontalAlignmentValues> for HorizontalAlignmentValues {
    fn get_string(input_enum: HorizontalAlignmentValues) -> String {
        match input_enum {
            HorizontalAlignmentValues::LEFT => "left".to_string(),
            HorizontalAlignmentValues::CENTER => "center".to_string(),
            HorizontalAlignmentValues::RIGHT => "right".to_string(),
            HorizontalAlignmentValues::JUSTIFY => "justify".to_string(),
            HorizontalAlignmentValues::None => "none".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> HorizontalAlignmentValues {
        match input_string {
            "left" => HorizontalAlignmentValues::LEFT,
            "center" => HorizontalAlignmentValues::CENTER,
            "right" => HorizontalAlignmentValues::RIGHT,
            "justify" => HorizontalAlignmentValues::JUSTIFY,
            _ => HorizontalAlignmentValues::None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash)]
pub enum VerticalAlignmentValues {
    None,
    Top,
    Middle,
    Bottom,
}

impl Enum<VerticalAlignmentValues> for VerticalAlignmentValues {
    fn get_string(input_enum: VerticalAlignmentValues) -> String {
        match input_enum {
            VerticalAlignmentValues::Top => "top".to_string(),
            VerticalAlignmentValues::Middle => "center".to_string(),
            VerticalAlignmentValues::Bottom => "bottom".to_string(),
            VerticalAlignmentValues::None => "none".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> VerticalAlignmentValues {
        match input_string {
            "top" => VerticalAlignmentValues::Top,
            "center" => VerticalAlignmentValues::Middle,
            "bottom" => VerticalAlignmentValues::Bottom,
            _ => VerticalAlignmentValues::None,
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
