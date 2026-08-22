#[derive(Debug, Clone)]
pub enum PowerPointHyperlinkPropertyTypeValues {
    EXISTING_FILE,
    WEB_URL,
    TARGET_SLIDE,
    NEXT_SLIDE,
    PREVIOUS_SLIDE,
    FIRST_SLIDE,
    LAST_SLIDE,
}

#[derive(Debug, Clone)]
pub enum UnderLineValues {
    NONE,
    DASH,
    DASH_HEAVY,
    DASH_LONG,
    DASH_LONG_HEAVY,
    DOT_DASH,
    DOT_DASH_HEAVY,
    DOT_DOT_DASH,
    DOT_DOT_DASH_HEAVY,
    DOTTED,
    DOUBLE,
    HEAVY,
    HEAVY_DOTTED,
    SINGLE,
    WAVY,
    WAVY_DOUBLE,
    WAVY_HEAVY,
    WORDS,
}

#[derive(Debug, Clone)]
pub enum StrikeValues {
    NO_STRIKE,
    SINGLE_STRIKE,
    DOUBLE_STRIKE,
}

#[derive(Debug, Clone)]
pub struct TextOptions {
    text_value: Option<String>,
    is_bold: bool,
    is_italic: bool,
    font_size: f32,
    font_color: Option<String>,
    font_family: String,
    under_line_values: UnderLineValues,
    strike_values: StrikeValues,
}

impl Default for TextOptions {
    fn default() -> Self {
        TextOptions {
            text_value: None,
            is_bold: false,
            is_italic: false,
            font_size: 11.97,
            font_color: None,
            font_family: "(Calibri (Body))".to_string(),
            under_line_values: UnderLineValues::NONE,
            strike_values: StrikeValues::NO_STRIKE,
        }
    }
}
