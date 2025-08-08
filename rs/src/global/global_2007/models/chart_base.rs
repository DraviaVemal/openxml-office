use crate::global_2007::models::TextOptions;

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

pub(crate) trait SizeAndPosition {}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AnchorPosition {
    pub(crate) column: i32,
    pub(crate) column_offset: i32,
    pub(crate) row: i32,
    pub(crate) row_offset: i32,
}

impl Default for AnchorPosition {
    fn default() -> Self {
        AnchorPosition {
            column: 1,
            column_offset: 0,
            row: 1,
            row_offset: 0,
        }
    }
}

pub(crate) struct PresentationSetting {
    pub(crate) height: i32,
    pub(crate) width: i32,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Default for PresentationSetting {
    fn default() -> Self {
        PresentationSetting {
            height: 6858000,
            width: 12192000,
            x: 0,
            y: 0,
        }
    }
}
impl SizeAndPosition for PresentationSetting {}

pub(crate) struct ExcelSetting {
    pub(crate) to: AnchorPosition,
    pub(crate) from: AnchorPosition,
}
impl SizeAndPosition for ExcelSetting {}

pub(crate) struct ChartTextOptions {
    common_text_options: TextOptions,
    text_direction_value: ChartTextDirectionValues,
    chart_vertical_text_alignment_value: ChartVerticalTextAlignmentValues,
    text_angle: i16,
}

pub(crate) trait AxisTypeOptions {}

pub(crate) struct CategoryAxis {
    specific_interval_unit: u32,
}
impl AxisTypeOptions for CategoryAxis {}

pub(crate) struct ValueAxis {
    bounds_minimum: f32,
    bounds_maximum: f32,
    units_major: f32,
    units_minor: f32,
}
impl AxisTypeOptions for ValueAxis {}

pub(crate) struct ChartAxesLabel {
    chart_text_option: ChartTextOptions,
    axes_label_position: AxesLabelPosition,
    in_reverse_order: bool,
}

pub(crate) struct ChartAxisTitle {
    chart_text_option: ChartTextOptions,
}

pub(crate) struct AxisOptions<AxisType>
where
    AxisType: AxisTypeOptions + Default,
{
    axis_line_color: String,
    is_axes_visible: bool,
    axis_type_option: AxisType,
    chart_axes_options: ChartAxesLabel,
    chart_axis_title: ChartAxisTitle,
}

pub(crate) struct ChartSetting<ApplicationSpecificSetting>
where
    ApplicationSpecificSetting: SizeAndPosition + Default,
{
    pub(crate) category_axis_id: u32,
    pub(crate) value_axis_id: u32,
    pub(crate) is_3d_chart: bool,
    pub(crate) is_secondary_axis: bool,
    pub(crate) application_specific_setting: ApplicationSpecificSetting,
}

pub(crate) struct XAxisOptions<AxisType>
where
    AxisType: AxisTypeOptions,
{
    axis_type_options: AxisType,
}
pub(crate) struct YAxisOptions<AxisType>
where
    AxisType: AxisTypeOptions,
{
    axis_type_options: AxisType,
}
pub(crate) struct ZAxisOptions<AxisType>
where
    AxisType: AxisTypeOptions,
{
    axis_type_options: AxisType,
}

pub(crate) struct ChartAxisOptions<XAxisType, YAxisType, ZAxisType>
where
    XAxisType: AxisTypeOptions,
    YAxisType: AxisTypeOptions,
    ZAxisType: AxisTypeOptions,
{
    x_axis_options: XAxisOptions<XAxisType>,
    y_axis_options: YAxisOptions<YAxisType>,
    z_axis_options: ZAxisOptions<ZAxisType>,
}

pub(crate) struct ChartDataLabel {
    text_option: TextOptions,
    separator: String,
    show_category_name: bool,
    show_legend_key: bool,
    show_series_name: bool,
    show_value: bool,
    show_percentage: bool,
    format_code: String,
}

pub(crate) struct ChartSeriesSetting {}
