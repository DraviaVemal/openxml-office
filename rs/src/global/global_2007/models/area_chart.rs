use crate::global_2007::models::{
    CategoryAxis, ChartAxisOptions, ChartDataLabel, ChartSeriesSetting, ChartSetting,
    SizeAndPosition, ValueAxis,
};

pub(crate) enum AreaChartTypes {
    CLUSTERED,
    STACKED,
    PERCENT_STACKED,
    CLUSTERED_3D,
    STACKED_3D,
    PERCENT_STACKED_3D,
}

pub(crate) struct AreaChartDataLabel {
    chart_data_label: ChartDataLabel,
}

pub(crate) struct AreaChartSeriesSetting {
    chart_series_setting: ChartSeriesSetting,
    area_chart_data_label: AreaChartDataLabel,
    fill_color: String,
}

pub(crate) struct AreaChartSetting<ApplicationSpecificSetting>
where
    ApplicationSpecificSetting: SizeAndPosition + Default,
{
    pub common_chart_setting: ChartSetting<ApplicationSpecificSetting>,
    area_chart_data_label: AreaChartDataLabel,
    area_chart_series_settings: Vec<AreaChartSeriesSetting>,
    area_chart_type: AreaChartTypes,
    chart_axis_options: ChartAxisOptions<CategoryAxis, ValueAxis, ValueAxis>,
}
