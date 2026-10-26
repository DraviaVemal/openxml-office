pub(crate) enum ChartTextDirectionValues {
    HORIZONTAL,
    ROTATE_90,
    ROTATE_270,
    STACKED,
}

pub(crate) enum ChartVerticalTextAlignmentValues {
    RIGHT,
    CENTER,
    LEFT,
    RIGHT_MIDDLE,
    CENTER_MIDDLE,
    LEFT_MIDDLE,
    TOP,
    MIDDLE,
    BOTTOM,
    TOP_CENTER,
    MIDDLE_CENTER,
    BOTTOM_CENTER,
}

pub(crate) enum AxesLabelPosition {
    NEXT_TO_AXIS,
    LOW,
    HIGH,
    NONE,
}

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

pub(crate) enum AreaChartTypes {
    CLUSTERED,
    STACKED,
    PERCENT_STACKED,
    CLUSTERED_3D,
    STACKED_3D,
    PERCENT_STACKED_3D,
}
