// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.
using DocumentFormat.OpenXml;
using OpenXMLOffice.Global_2013;
using C = DocumentFormat.OpenXml.Drawing.Charts;
namespace OpenXMLOffice.Global_2007
{
	/// <summary>
	/// Represents the types of scatter charts.
	/// </summary>
	public class ScatterChart<ApplicationSpecificSetting> : ChartAdvance<ApplicationSpecificSetting> where ApplicationSpecificSetting : class, ISizeAndPosition
	{
		/// <summary>
		/// Scatter Chart Setting
		/// </summary>
		protected ScatterChartSetting<ApplicationSpecificSetting> scatterChartSetting;
		internal ScatterChart(ScatterChartSetting<ApplicationSpecificSetting> scatterChartSetting) : base(scatterChartSetting)
		{
			this.scatterChartSetting = scatterChartSetting;
		}
		/// <summary>
		/// Create Scatter Chart with provided settings
		/// </summary>
		public ScatterChart(ScatterChartSetting<ApplicationSpecificSetting> scatterChartSetting, ChartData[][] dataCols, DataRange dataRange = null) : base(scatterChartSetting)
		{
			this.scatterChartSetting = scatterChartSetting;
			SetChartPlotArea(CreateChartPlotArea(dataCols, dataRange));
		}
		private C.PlotArea CreateChartPlotArea(ChartData[][] dataCols, DataRange dataRange)
		{
			if (scatterChartSetting.scatterChartType == ScatterChartTypes.BUBBLE)
			{
				scatterChartSetting.chartDataSetting.is3Ddata = true;
				if ((dataCols.Length - 1) % 2 != 0)
				{
					throw new ArgumentOutOfRangeException("Required 3D Data Size is not met.");
				}
			}
			C.PlotArea plotArea = new C.PlotArea();
			plotArea.Append(CreateLayout(scatterChartSetting.plotAreaOptions?.manualLayout));
			if (scatterChartSetting.scatterChartType == ScatterChartTypes.BUBBLE)
			{
				plotArea.Append(CreateChart<C.BubbleChart>(CreateDataSeries(scatterChartSetting.chartDataSetting, dataCols, dataRange)));
			}
			else
			{
				plotArea.Append(CreateChart<C.ScatterChart>(CreateDataSeries(scatterChartSetting.chartDataSetting, dataCols, dataRange)));
			}
			plotArea.Append(CreateValueAxis(new ValueAxisSetting()
			{
				id = CategoryAxisId,
				axisPosition = AxisPosition.BOTTOM,
				crossAxisId = ValueAxisId,
				fontSize = scatterChartSetting.chartAxesOptions.horizontalFontSize,
				isBold = scatterChartSetting.chartAxesOptions.isHorizontalBold,
				isItalic = scatterChartSetting.chartAxesOptions.isHorizontalItalic,
				isVisible = scatterChartSetting.chartAxesOptions.isHorizontalAxesEnabled,
				invertOrder = scatterChartSetting.chartAxesOptions.invertHorizontalAxesOrder,
			}));
			plotArea.Append(CreateValueAxis(new ValueAxisSetting()
			{
				id = ValueAxisId,
				crossAxisId = CategoryAxisId,
				fontSize = scatterChartSetting.chartAxesOptions.verticalFontSize,
				isBold = scatterChartSetting.chartAxesOptions.isVerticalBold,
				isItalic = scatterChartSetting.chartAxesOptions.isVerticalItalic,
				isVisible = scatterChartSetting.chartAxesOptions.isVerticalAxesEnabled,
				invertOrder = scatterChartSetting.chartAxesOptions.invertVerticalAxesOrder,
			}));
			plotArea.Append(CreateChartShapeProperties());
			return plotArea;
		}
		internal ChartType CreateChart<ChartType>(List<ChartDataGrouping> chartDataGroupings) where ChartType : OpenXmlCompositeElement, new()
		{
			ChartType chart = new ChartType();
			C.ScatterStyleValues scatterStyleValue;

			if (scatterChartSetting.scatterChartType == ScatterChartTypes.SCATTER_SMOOTH)
			{
				scatterStyleValue = C.ScatterStyleValues.Smooth;
			}
			else if (scatterChartSetting.scatterChartType == ScatterChartTypes.SCATTER_SMOOTH_MARKER)
			{
				scatterStyleValue = C.ScatterStyleValues.SmoothMarker;
			}
			else if (scatterChartSetting.scatterChartType == ScatterChartTypes.SCATTER_STRIGHT)
			{
				scatterStyleValue = C.ScatterStyleValues.Line;
			}
			else if (scatterChartSetting.scatterChartType == ScatterChartTypes.SCATTER_STRIGHT_MARKER)
			{
				scatterStyleValue = C.ScatterStyleValues.LineMarker;
			}
			else
			{
				scatterStyleValue = C.ScatterStyleValues.LineMarker;
			}

			chart.Append(new C.ScatterStyle
			{
				Val = scatterStyleValue
			});

			chart.Append(new C.VaryColors() { Val = false });
			int seriesIndex = 0;
			chartDataGroupings.ForEach(Series =>
			{
				chart.Append(CreateScatterChartSeries(seriesIndex, Series));
				seriesIndex++;
			});
			C.DataLabels dataLabels = CreateScatterDataLabels(scatterChartSetting.scatterChartDataLabel);
			if (dataLabels != null)
			{
				chart.Append(dataLabels);
			}
			if (scatterChartSetting.scatterChartType == ScatterChartTypes.BUBBLE)
			{
				chart.Append(new C.BubbleScale() { Val = 100 });
				chart.Append(new C.ShowNegativeBubbles() { Val = true });
			}
			chart.Append(new C.AxisId { Val = CategoryAxisId });
			chart.Append(new C.AxisId { Val = ValueAxisId });
			return chart;
		}
		private C.ScatterChartSeries CreateScatterChartSeries(int seriesIndex, ChartDataGrouping chartDataGrouping)
		{
			C.DataLabels dataLabels = seriesIndex < scatterChartSetting.scatterChartSeriesSettings.Count ?
				CreateScatterDataLabels(scatterChartSetting.scatterChartSeriesSettings?[seriesIndex]?.scatterChartDataLabel ?? new ScatterChartDataLabel(), chartDataGrouping.dataLabelCells?.Length ?? 0) : null;
			SolidFillModel GetSeriesBorderColor()
			{
				SolidFillModel solidFillModel = new SolidFillModel();
				string hexColor = scatterChartSetting.scatterChartSeriesSettings?
							.Select(item => item?.borderColor)
							.ToList().ElementAtOrDefault(seriesIndex);
				if (hexColor != null)
				{
					solidFillModel.hexColor = hexColor;
					return solidFillModel;
				}
				else
				{
					solidFillModel.schemeColorModel = new SchemeColorModel()
					{
						themeColorValues = ThemeColorValues.ACCENT_1 + (chartDataGrouping.id % AccentColurCount),
					};
				}
				return solidFillModel;
			}
			MarkerModel markerModel = new MarkerModel();
			if (new[] { ScatterChartTypes.SCATTER, ScatterChartTypes.SCATTER_SMOOTH_MARKER, ScatterChartTypes.SCATTER_STRIGHT_MARKER }.Contains(scatterChartSetting.scatterChartType))
			{
				markerModel.markerShapeValues = scatterChartSetting.scatterChartType == ScatterChartTypes.SCATTER ? MarkerModel.MarkerShapeValues.AUTO : MarkerModel.MarkerShapeValues.CIRCLE;
				markerModel.shapeProperties = new ShapePropertiesModel()
				{
					solidFill = new SolidFillModel()
					{
						schemeColorModel = new SchemeColorModel()
						{
							themeColorValues = ThemeColorValues.ACCENT_1 + (chartDataGrouping.id % AccentColurCount),
						}
					},
					outline = new OutlineModel()
					{
						solidFill = new SolidFillModel()
						{
							schemeColorModel = new SchemeColorModel()
							{
								themeColorValues = ThemeColorValues.ACCENT_1 + (chartDataGrouping.id % AccentColurCount),
							}
						}
					}
				};
			}
			C.ScatterChartSeries series = new C.ScatterChartSeries(
				new C.Index { Val = new UInt32Value((uint)chartDataGrouping.id) },
				new C.Order { Val = new UInt32Value((uint)chartDataGrouping.id) },
				CreateSeriesText(chartDataGrouping.seriesHeaderFormula, new[] { chartDataGrouping.seriesHeaderCells }));
			ShapePropertiesModel shapePropertiesModel = new ShapePropertiesModel()
			{
				outline = new OutlineModel()
				{
					solidFill = scatterChartSetting.scatterChartType == ScatterChartTypes.SCATTER ? null : GetSeriesBorderColor(),
				}
			};
			if (scatterChartSetting.scatterChartType == ScatterChartTypes.BUBBLE)
			{
				shapePropertiesModel.solidFill = new SolidFillModel()
				{
					schemeColorModel = new SchemeColorModel()
					{
						themeColorValues = ThemeColorValues.ACCENT_1 + (seriesIndex % AccentColurCount),
						tint = 75000,
					}
				};
				series.Append(new C.InvertIfNegative() { Val = false });
			}
			series.Append(CreateChartShapeProperties(shapePropertiesModel));
			if (scatterChartSetting.scatterChartType != ScatterChartTypes.BUBBLE)
			{
				series.Append(CreateMarker(markerModel));
			}
			if (dataLabels != null)
			{
				series.Append(dataLabels);
			}
			series.Append(CreateXValueAxisData(chartDataGrouping.xAxisFormula, chartDataGrouping.xAxisCells));
			series.Append(CreateYValueAxisData(chartDataGrouping.yAxisFormula, chartDataGrouping.yAxisCells));
			if (scatterChartSetting.scatterChartType == ScatterChartTypes.BUBBLE)
			{
				series.Append(CreateBubbleSizeAxisData(chartDataGrouping.zAxisFormula, chartDataGrouping.zAxisCells));
				series.Append(new C.Bubble3D() { Val = false });
			}
			else
			{
				series.Append(new C.Smooth() { Val = new[] { ScatterChartTypes.SCATTER_SMOOTH, ScatterChartTypes.SCATTER_SMOOTH_MARKER }.Contains(scatterChartSetting.scatterChartType) });
			}
			if (chartDataGrouping.dataLabelCells != null && chartDataGrouping.dataLabelFormula != null)
			{
				series.Append(new C.ExtensionList(new C.Extension(
					CreateDataLabelsRange(chartDataGrouping.dataLabelFormula, chartDataGrouping.dataLabelCells.Skip(1).ToArray())
				)
				{ Uri = "{02D57815-91ED-43cb-92C2-25804820EDAC}" }));
			}
			return series;
		}
		private C.DataLabels CreateScatterDataLabels(ScatterChartDataLabel scatterChartDataLabel, int? dataLabelCounter = 0)
		{
			if (scatterChartDataLabel.showValue || scatterChartSetting.chartDataSetting.advancedDataLabel.showValueFromColumn || scatterChartDataLabel.showCategoryName || scatterChartDataLabel.showLegendKey || scatterChartDataLabel.showSeriesName || scatterChartDataLabel.showBubbleSize)
			{
				C.DataLabels dataLabels = CreateDataLabels(scatterChartDataLabel, dataLabelCounter);
				dataLabels.Append(new C.ShowBubbleSize { Val = scatterChartDataLabel.showBubbleSize });
				C.DataLabelPositionValues dataLabelPositionValue;
				if (scatterChartDataLabel.dataLabelPosition == ScatterChartDataLabel.DataLabelPositionValues.LEFT)
				{
					dataLabelPositionValue = C.DataLabelPositionValues.Left;
				}
				else if (scatterChartDataLabel.dataLabelPosition == ScatterChartDataLabel.DataLabelPositionValues.RIGHT)
				{
					dataLabelPositionValue = C.DataLabelPositionValues.Right;
				}
				else if (scatterChartDataLabel.dataLabelPosition == ScatterChartDataLabel.DataLabelPositionValues.ABOVE)
				{
					dataLabelPositionValue = C.DataLabelPositionValues.Top;
				}
				else if (scatterChartDataLabel.dataLabelPosition == ScatterChartDataLabel.DataLabelPositionValues.BELOW)
				{
					dataLabelPositionValue = C.DataLabelPositionValues.Bottom;
				}
				else
				{
					dataLabelPositionValue = C.DataLabelPositionValues.Center;
				}
				dataLabels.InsertAt(new C.DataLabelPosition()
				{
					Val = dataLabelPositionValue
				}, 0);
				return dataLabels;
			}
			return null;
		}
	}
}
