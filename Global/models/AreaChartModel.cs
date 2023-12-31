namespace OpenXMLOffice.Global
{
    public class AreaChartSeriesSetting
    {
        #region Public Fields

        public string? NumberFormat;
        public string? FillColor;
        public string? BorderColor;
        public AreaChartDataLabel AreaChartDataLabel = new();
        #endregion Public Fields
    }
    public class AreaChartDataLabel
    {
        public enum eDataLabelPosition
        {
            NONE,
            SHOW,
            // CALLOUT
        }

        public eDataLabelPosition DataLabelPosition = eDataLabelPosition.NONE;
    }
    public class AreaChartSetting : ChartSetting
    {
        public ChartAxisOptions ChartAxisOptions = new();
        public ChartAxesOptions ChartAxesOptions = new();
        public ChartGridLinesOptions ChartGridLinesOptions = new();
        public List<AreaChartSeriesSetting>? SeriesSettings;
    }
}