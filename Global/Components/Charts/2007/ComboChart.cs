// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using System;
using System.Collections.Generic;
using System.Linq;
using OpenXMLOffice.Global_2013;
using C = DocumentFormat.OpenXml.Drawing.Charts;
namespace OpenXMLOffice.Global_2007
{
	/// <summary>
	///
	/// </summary>
	public class ComboChart<ApplicationSpecificSetting> : ChartAdvance<ApplicationSpecificSetting> where ApplicationSpecificSetting : class, ISizeAndPosition, new()
	{
		/// <summary>
		///
		/// </summary>
		public ComboChartSetting<ApplicationSpecificSetting> comboChartSetting { get; private set; }
		/// <summary>
		///
		/// </summary>
		public ComboChart(ComboChartSetting<ApplicationSpecificSetting> comboChartSetting, ChartData[][] dataCols, DataRange dataRange = null) : base(comboChartSetting)
		{
			this.comboChartSetting = comboChartSetting;
			SetChartPlotArea(CreateChartPlotArea(dataCols, dataRange));
		}
		private C.PlotArea CreateChartPlotArea(ChartData[][] dataCols, DataRange dataRange)
		{
			bool isSecondaryAxisActive = false;
			if (comboChartSetting.ComboChartsSettingList.Count == 0)
			{
				throw new ArgumentException("Combo Chart Series Settings is empty");
			}
			C.PlotArea plotArea = new C.PlotArea();
			plotArea.Append(CreateLayout(comboChartSetting.plotAreaOptions != null ? comboChartSetting.plotAreaOptions.manualLayout : null));
			uint chartPosition = 0;
			comboChartSetting.ComboChartsSettingList.ForEach(chartSetting =>
			{
				if (((ChartSetting<ApplicationSpecificSetting>)chartSetting).isSecondaryAxis)
				{
					isSecondaryAxisActive = true;
					((ChartSetting<ApplicationSpecificSetting>)chartSetting).categoryAxisId = SecondaryCategoryAxisId;
					((ChartSetting<ApplicationSpecificSetting>)chartSetting).valueAxisId = SecondaryValueAxisId;
				}
				((ChartSetting<ApplicationSpecificSetting>)chartSetting).chartDataSetting = new ChartDataSetting();
				AreaChartSetting<ApplicationSpecificSetting> areaChartSetting = chartSetting as AreaChartSetting<ApplicationSpecificSetting>;
				if (areaChartSetting != null)
				{
					AreaChart<ApplicationSpecificSetting> areaChart = new AreaChart<ApplicationSpecificSetting>(areaChartSetting);
					plotArea.Append(areaChart.CreateAreaChart<C.AreaChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
				}
				BarChartSetting<ApplicationSpecificSetting> barChartSetting = chartSetting as BarChartSetting<ApplicationSpecificSetting>;
				if (barChartSetting != null)
				{
					BarChart<ApplicationSpecificSetting> barChart = new BarChart<ApplicationSpecificSetting>(barChartSetting);
					plotArea.Append(barChart.CreateBarChart<C.BarChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
				}
				ColumnChartSetting<ApplicationSpecificSetting> columnChartSetting = chartSetting as ColumnChartSetting<ApplicationSpecificSetting>;
				if (columnChartSetting != null)
				{
					ColumnChart<ApplicationSpecificSetting> columnChart = new ColumnChart<ApplicationSpecificSetting>(columnChartSetting);
					plotArea.Append(columnChart.CreateColumnChart<C.BarChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
				}
				LineChartSetting<ApplicationSpecificSetting> lineChartSetting = chartSetting as LineChartSetting<ApplicationSpecificSetting>;
				if (lineChartSetting != null)
				{
					LineChart<ApplicationSpecificSetting> lineChart = new LineChart<ApplicationSpecificSetting>(lineChartSetting);
					plotArea.Append(lineChart.CreateLineChart(GetChartPositionData(dataCols, chartPosition, dataRange)));
				}
				PieChartSetting<ApplicationSpecificSetting> pieChartSetting = chartSetting as PieChartSetting<ApplicationSpecificSetting>;
				if (pieChartSetting != null)
				{
					PieChart<ApplicationSpecificSetting> pieChart = new PieChart<ApplicationSpecificSetting>(pieChartSetting);
					if (pieChartSetting.pieChartType == PieChartTypes.DOUGHNUT)
					{
						plotArea.Append(pieChart.CreateChart<C.DoughnutChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
					}
					else
					{
						plotArea.Append(pieChart.CreateChart<C.PieChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
					}
				}
				ScatterChartSetting<ApplicationSpecificSetting> scatterChartSetting = chartSetting as ScatterChartSetting<ApplicationSpecificSetting>;
				if (scatterChartSetting != null)
				{
					ScatterChart<ApplicationSpecificSetting> scatterChart = new ScatterChart<ApplicationSpecificSetting>(scatterChartSetting);
					if (scatterChartSetting.scatterChartType == ScatterChartTypes.BUBBLE)
					{
						plotArea.Append(scatterChart.CreateChart<C.BubbleChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
					}
					else
					{
						plotArea.Append(scatterChart.CreateChart<C.ScatterChart>(GetChartPositionData(dataCols, chartPosition, dataRange)));
					}
				}
				chartPosition++;
			});
			plotArea.Append(CreateAxis<C.CategoryAxis, XAxisOptions>(new AxisSetting<XAxisOptions>()
			{
				id = CategoryAxisId,
				crossAxisId = ValueAxisId,
				axisOptions = comboChartSetting.chartAxisOptions.xAxisOptions,
				axisPosition = comboChartSetting.chartAxisOptions.xAxisOptions.chartAxesOptions.inReverseOrder ? AxisPosition.TOP : AxisPosition.BOTTOM
			}));
			plotArea.Append(CreateAxis<C.CategoryAxis, YAxisOptions>(new AxisSetting<YAxisOptions>()
			{
				id = ValueAxisId,
				crossAxisId = CategoryAxisId,
				axisOptions = comboChartSetting.chartAxisOptions.yAxisOptions,
				axisPosition = comboChartSetting.chartAxisOptions.yAxisOptions.chartAxesOptions.inReverseOrder ? AxisPosition.RIGHT : AxisPosition.LEFT
			}));
			if (isSecondaryAxisActive)
			{
				plotArea.Append(CreateAxis<C.CategoryAxis, ZAxisOptions>(new AxisSetting<ZAxisOptions>()
				{
					id = SecondaryCategoryAxisId,
					crossAxisId = SecondaryValueAxisId,
					axisOptions = comboChartSetting.chartAxisOptions.zAxisOptions
				}));
				plotArea.Append(CreateAxis<C.CategoryAxis, ZAxisOptions>(new AxisSetting<ZAxisOptions>()
				{
					id = SecondaryValueAxisId,
					crossAxisId = SecondaryCategoryAxisId,
					axisOptions = comboChartSetting.chartAxisOptions.zAxisOptions
				}));
			}
			plotArea.Append(CreateChartShapeProperties());
			return plotArea;
		}
		private List<ChartDataGrouping> GetChartPositionData(ChartData[][] dataCols, uint chartPosition, DataRange dataRange)
		{
			List<ChartDataGrouping> chartDataGroupings = CreateDataSeries(comboChartSetting.chartDataSetting, dataCols, dataRange);
			return new List<ChartDataGrouping>() { chartDataGroupings.ElementAt((int)chartPosition) };
		}
	}
}
