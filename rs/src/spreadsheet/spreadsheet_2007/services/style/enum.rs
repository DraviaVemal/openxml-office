use crate::global::traits::Enum;
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
