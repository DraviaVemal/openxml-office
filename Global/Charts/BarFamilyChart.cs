// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using DocumentFormat.OpenXml;
using A = DocumentFormat.OpenXml.Drawing;
using C = DocumentFormat.OpenXml.Drawing.Charts;

namespace OpenXMLOffice.Global
{
    /// <summary>
    /// Represents the settings for a bar chart.
    /// </summary>
    public class BarFamilyChart : ChartBase
    {
        #region Protected Fields

        /// <summary>
        /// Bar Chart Setting
        /// </summary>
        protected readonly BarChartSetting BarChartSetting;

        #endregion Protected Fields

        #region Protected Constructors

        /// <summary>
        /// Create Bar Chart with provided settings
        /// </summary>
        /// <param name="BarChartSetting">
        /// </param>
        /// <param name="DataCols">
        /// </param>
        protected BarFamilyChart(BarChartSetting BarChartSetting, ChartData[][] DataCols) : base(BarChartSetting)
        {
            this.BarChartSetting = BarChartSetting;
            SetChartPlotArea(CreateChartPlotArea(DataCols));
        }

        #endregion Protected Constructors

        #region Private Methods

        private C.BarChartSeries CreateBarChartSeries(int seriesIndex, ChartDataGrouping ChartDataGrouping, A.SolidFill SolidFill, C.DataLabels? DataLabels)
        {
            C.BarChartSeries series = new(
                new C.Index { Val = new UInt32Value((uint)seriesIndex) },
                new C.Order { Val = new UInt32Value((uint)seriesIndex) },
                CreateSeriesText(ChartDataGrouping.SeriesHeaderFormula!, new[] { ChartDataGrouping.SeriesHeaderCells! }),
                new C.InvertIfNegative { Val = true });
            C.ShapeProperties ShapeProperties = CreateShapeProperties();
            ShapeProperties.Append(SolidFill);
            ShapeProperties.Append(new A.Outline(new A.NoFill()));
            ShapeProperties.Append(new A.EffectList());
            if (DataLabels != null)
            {
                series.Append(DataLabels);
            }
            series.Append(ShapeProperties);
            series.Append(CreateCategoryAxisData(ChartDataGrouping.XaxisFormula!, ChartDataGrouping.XaxisCells!));
            series.Append(CreateValueAxisData(ChartDataGrouping.YaxisFormula!, ChartDataGrouping.YaxisCells!));
            if (ChartDataGrouping.DataLabelCells != null && ChartDataGrouping.DataLabelFormula != null)
            {
                series.Append(new C.ExtensionList(new C.Extension(
                    CreateDataLabelsRange(ChartDataGrouping.DataLabelFormula, ChartDataGrouping.DataLabelCells.Skip(1).ToArray())
                )
                { Uri = GeneratorUtils.GenerateNewGUID() }));
            }
            return series;
        }

        private C.DataLabels? CreateBarDataLabels(BarChartDataLabel BarChartDataLabel, int? DataLabelCounter = 0)
        {
            if (BarChartDataLabel.ShowValue || BarChartDataLabel.ShowValueFromColumn || BarChartDataLabel.ShowCategoryName || BarChartDataLabel.ShowLegendKey || BarChartDataLabel.ShowSeriesName || DataLabelCounter > 0)
            {
                C.DataLabels DataLabels = CreateDataLabels(BarChartDataLabel, DataLabelCounter);
                if (BarChartSetting.BarChartTypes != BarChartTypes.CLUSTERED && BarChartDataLabel.DataLabelPosition == BarChartDataLabel.DataLabelPositionValues.OUTSIDE_END)
                {
                    throw new ArgumentException("'Outside End' Data Label Is only Available with Cluster chart type");
                }
                DataLabels.InsertAt(new C.DataLabelPosition()
                {
                    Val = BarChartDataLabel.DataLabelPosition switch
                    {
                        BarChartDataLabel.DataLabelPositionValues.OUTSIDE_END => C.DataLabelPositionValues.OutsideEnd,
                        BarChartDataLabel.DataLabelPositionValues.INSIDE_END => C.DataLabelPositionValues.InsideEnd,
                        BarChartDataLabel.DataLabelPositionValues.INSIDE_BASE => C.DataLabelPositionValues.InsideBase,
                        _ => C.DataLabelPositionValues.Center
                    }
                }, 0);
                DataLabels.InsertAt(new C.ShapeProperties(new A.NoFill(), new A.Outline(new A.NoFill()), new A.EffectList()), 0);
                A.Paragraph Paragraph = new(new A.ParagraphProperties(new A.DefaultRunProperties(
                    new A.SolidFill(new A.SchemeColor(new A.LuminanceModulation() { Val = 75000 }, new A.LuminanceOffset() { Val = 25000 }) { Val = A.SchemeColorValues.Text1 }),
                    new A.LatinFont() { Typeface = "+mn-lt" }, new A.EastAsianFont() { Typeface = "+mn-ea" }, new A.ComplexScriptFont() { Typeface = "+mn-cs" })
                {
                    FontSize = (int)BarChartDataLabel.FontSize * 100,
                    Bold = BarChartDataLabel.IsBold,
                    Italic = BarChartDataLabel.IsItalic,
                    Underline = A.TextUnderlineValues.None,
                    Strike = A.TextStrikeValues.NoStrike,
                    Kerning = 1200,
                    Baseline = 0
                }), new A.EndParagraphRunProperties() { Language = "en-US" });
                DataLabels.InsertAt(new C.TextProperties(new A.BodyProperties(new A.ShapeAutoFit())
                {
                    Rotation = 0,
                    UseParagraphSpacing = true,
                    VerticalOverflow = A.TextVerticalOverflowValues.Ellipsis,
                    Vertical = A.TextVerticalValues.Horizontal,
                    Wrap = A.TextWrappingValues.Square,
                    LeftInset = 38100,
                    TopInset = 19050,
                    RightInset = 38100,
                    BottomInset = 19050,
                    Anchor = A.TextAnchoringTypeValues.Center,
                    AnchorCenter = true
                }, new A.ListStyle(),
               Paragraph), 0);
                return DataLabels;
            }
            return null;
        }

        private C.PlotArea CreateChartPlotArea(ChartData[][] DataCols)
        {
            C.PlotArea plotArea = new();
            plotArea.Append(new C.Layout());
            C.BarChart BarChart = new(
                new C.BarDirection { Val = C.BarDirectionValues.Bar },
                new C.BarGrouping
                {
                    Val = BarChartSetting.BarChartTypes switch
                    {
                        BarChartTypes.STACKED => C.BarGroupingValues.Stacked,
                        BarChartTypes.PERCENT_STACKED => C.BarGroupingValues.PercentStacked,
                        // Clusted
                        _ => C.BarGroupingValues.Clustered
                    }
                },
                new C.VaryColors { Val = false });
            int seriesIndex = 0;
            CreateDataSeries(DataCols, BarChartSetting.ChartDataSetting).ForEach(Series =>
            {
                C.DataLabels? GetDataLabels()
                {
                    if (seriesIndex < BarChartSetting.BarChartSeriesSettings.Count)
                    {
                        return CreateBarDataLabels(BarChartSetting.BarChartSeriesSettings?[seriesIndex]?.BarChartDataLabel ?? new BarChartDataLabel(), Series.DataLabelCells?.Length ?? 0);
                    }
                    return null;
                }
                BarChart.Append(CreateBarChartSeries(seriesIndex, Series,
                    CreateSolidFill(BarChartSetting.BarChartSeriesSettings
                            .Where(item => item?.FillColor != null)
                            .Select(item => item?.FillColor!)
                            .ToList(), seriesIndex),
                    GetDataLabels()));
                seriesIndex++;
            });
            if (BarChartSetting.BarChartTypes == BarChartTypes.CLUSTERED)
            {
                BarChart.Append(new C.GapWidth { Val = (UInt16Value)BarChartSetting.BarGraphicsSetting.CategoryGap });
                BarChart.Append(new C.Overlap { Val = (SByteValue)BarChartSetting.BarGraphicsSetting.SeriesGap });
            }
            else
            {
                BarChart.Append(new C.GapWidth { Val = 150 });
                BarChart.Append(new C.Overlap { Val = 100 });
            }
            C.DataLabels? DataLabels = CreateBarDataLabels(BarChartSetting.BarChartDataLabel);
            if (DataLabels != null)
            {
                BarChart.Append(DataLabels);
            }
            BarChart.Append(new C.AxisId { Val = 1362418656 });
            BarChart.Append(new C.AxisId { Val = 1358349936 });
            plotArea.Append(BarChart);
            plotArea.Append(CreateCategoryAxis(new CategoryAxisSetting()
            {
                Id = 1362418656,
                AxisPosition = AxisPosition.LEFT,
                CrossAxisId = 1358349936,
                FontSize = BarChartSetting.ChartAxesOptions.VerticalFontSize,
                IsBold = BarChartSetting.ChartAxesOptions.IsVerticalBold,
                IsItalic = BarChartSetting.ChartAxesOptions.IsVerticalItalic,
            }));
            plotArea.Append(CreateValueAxis(new ValueAxisSetting()
            {
                Id = 1358349936,
                AxisPosition = AxisPosition.BOTTOM,
                CrossAxisId = 1362418656,
                FontSize = BarChartSetting.ChartAxesOptions.HorizontalFontSize,
                IsBold = BarChartSetting.ChartAxesOptions.IsHorizontalBold,
                IsItalic = BarChartSetting.ChartAxesOptions.IsHorizontalItalic,
            }));
            C.ShapeProperties ShapeProperties = CreateShapeProperties();
            ShapeProperties.Append(new A.NoFill());
            ShapeProperties.Append(new A.Outline(new A.NoFill()));
            ShapeProperties.Append(new A.EffectList());
            plotArea.Append(ShapeProperties);
            return plotArea;
        }

        #endregion Private Methods
    }
}