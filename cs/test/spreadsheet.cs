// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using draviavemal.openxml_office.global_2007;
using draviavemal.openxml_office.spreadsheet_2007;

namespace openxmloffice.tests
{/// <summary>
 /// Excel Test
 /// </summary>
    [TestClass]
    public class Spreadsheet
    {
        private static readonly string resultPath = "../../test_results";
        private static readonly Excel excel = new(new ExcelProperties
        {
            coreProperties = new()
            {
                title = "Test File",
                creator = "OpenXML-Office",
                subject = "Test Subject",
                tags = "Test",
                category = "Test Category",
                description = "Describe the test file"
            }
        });
        /// <summary>
        /// Initialize excel Test
        /// </summary>
        /// <param name="context">
        /// </param>
        [ClassInitialize]
        public static void ClassInitialize(TestContext context)
        {
            if (!Directory.Exists(resultPath))
            {
                Directory.CreateDirectory(resultPath);
            }
            PrivacyProperties.ShareComponentRelatedDetails = false;
            PrivacyProperties.ShareIpGeoLocation = false;
            PrivacyProperties.ShareOsDetails = false;
            PrivacyProperties.SharePackageRelatedDetails = false;
            PrivacyProperties.ShareUsageCounterDetails = false;
            excel.AddSheet();
        }
        /// <summary>
        /// Save the Test File After execution
        /// </summary>
        [ClassCleanup]
        public static void ClassCleanup()
        {
            excel.SaveAs(string.Format("{1}/test-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        }
        /// <summary>
        /// 
        /// </summary>
        [TestMethod]
        public void BlankFile()
        {
            Excel excel2 = new();
            excel2.SaveAs(string.Format("{1}/Blank-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
            Assert.IsNotNull(excel2);
        }
        /// <summary>
        /// Add Sheet Test
        /// </summary>
        [TestMethod]
        public void AddSheet()
        {
            Worksheet worksheet = excel.AddSheet("TestSheet1");
            Assert.IsNotNull(worksheet);
        }
        /// <summary>
        /// Add Sheet Test
        /// </summary>
        [TestMethod]
        public void AddSecondSheet()
        {
            Worksheet worksheet = excel.AddSheet("TestSheet2");
            Assert.IsNotNull(worksheet);
        }
        /// <summary>
        /// Rename Sheet Based on Index Test
        /// </summary>
        [TestMethod]
        public void RenameBySheetId()
        {
            Worksheet worksheet = excel.AddSheet("TestSheet3");
            Assert.IsNotNull(worksheet);
            Assert.IsTrue(excel.RenameSheet("TestSheet3", "RenameTestSheet3"));
        }
        /// <summary>
        /// Rename Sheet Based on Index Test
        /// </summary>
        [TestMethod]
        public void RenameSheet()
        {
            Worksheet worksheet = excel.AddSheet("Sheet11");
            Assert.IsNotNull(worksheet);
            Assert.IsTrue(excel.RenameSheet("Sheet11", "RenameSheet11"));
        }
        // /// <summary>
        // /// Set Cell Test
        // /// </summary>
        // [TestMethod]
        // public void SetColumn()
        // {
        //     Worksheet worksheet = excel.AddSheet("Data3");
        //     Assert.IsNotNull(worksheet);
        //     worksheet.SetColumn("A1", new ColumnProperties()
        //     {
        //         width = 30
        //     });
        //     worksheet.SetColumn("C4", new ColumnProperties()
        //     {
        //         width = 30,
        //         bestFit = true
        //     });
        //     worksheet.SetColumn("G7", new ColumnProperties()
        //     {
        //         hidden = true
        //     });
        //     Assert.IsTrue(true);
        // }
        // /// <summary>
        // /// Set Row Test
        // /// </summary>
        // [TestMethod]
        // public void SetRow()
        // {
        //     Worksheet worksheet = excel.AddSheet("Data2");
        //     uint styleId = excel.GetStyleId(new CellStyleSetting()
        //     {
        //         numberFormat = "00.000",
        //     });
        //     Assert.IsNotNull(worksheet);
        //     worksheet.SetRow("A1", new ColumnCell[6]{
        //         new(){
        //             cellValue = "test1",
        //             dataType = CellDataType.STRING
        //         },
        //          new(){
        //             cellValue = "test2",
        //             dataType = CellDataType.STRING
        //         },
        //          new(){
        //             cellValue = "test3",
        //             dataType = CellDataType.STRING
        //         },
        //          new(){
        //             cellValue = "test4",
        //             dataType = CellDataType.STRING,
        //             styleSetting = new(){
        //                 fontSize = 20
        //             }
        //         },
        //          new(){
        //             cellValue = "2.51",
        //             dataType = CellDataType.NUMBER,
        //             styleId=styleId
        //         },new(){
        //             cellValue = "5.51",
        //             dataType = CellDataType.NUMBER,
        //             styleSetting = new(){
        //                 numberFormat = "₹ #,##0.00;₹ -#,##0.00",
        //             }
        //         }
        //     }, new RowProperties()
        //     {
        //         height = 20
        //     });
        //     worksheet.SetRow("C1", new ColumnCell[1]{
        //         new(){
        //             cellValue = "Re Update",
        //             dataType = CellDataType.STRING
        //         }
        //     }, new RowProperties()
        //     {
        //         height = 30
        //     });
        //     Assert.IsTrue(true);
        // }
        // /// <summary>
        // /// 
        // /// </summary>
        // [TestMethod]
        // public void AddMergeCell()
        // {
        //     Excel excel1 = new("./TestFiles/basic_test.xlsx", true);
        //     Worksheet worksheet = excel1.GetWorksheet("Style");
        //     List<MergeCellRange> mergedCellRange = worksheet.GetMergeCellList();
        //     Assert.AreEqual(1, mergedCellRange.Count);
        //     Assert.IsTrue(worksheet.SetMergeCell(new MergeCellRange()
        //     {
        //         topLeftCell = "D30",
        //         bottomRightCell = "F33"
        //     }));
        //     Assert.IsTrue(worksheet.SetMergeCell(new MergeCellRange()
        //     {
        //         topLeftCell = "G30",
        //         bottomRightCell = "J33"
        //     }));
        //     Assert.IsTrue(worksheet.RemoveMergeCell(new MergeCellRange()
        //     {
        //         topLeftCell = "G30",
        //         bottomRightCell = "J33"
        //     }));
        //     Assert.IsFalse(worksheet.SetMergeCell(new MergeCellRange()
        //     {
        //         topLeftCell = "F26",
        //         bottomRightCell = "J30",
        //     }));
        //     Assert.IsFalse(worksheet.RemoveMergeCell(new MergeCellRange()
        //     {
        //         topLeftCell = "A1",
        //         bottomRightCell = "C5",
        //     }));
        //     excel1.SaveAs(string.Format("{1}/ReadEdit-MergeCell-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        // }

        // /// <summary>
        // /// 
        // /// </summary>
        // [TestMethod]
        // public void FormulaCell()
        // {
        //     Excel excel1 = new("./TestFiles/basic_test.xlsx", true);
        //     Worksheet worksheet = excel1.GetWorksheet("formula");
        //     worksheet.SetRow("G1", new ColumnCell[2]{
        //         new(){
        //             dataType= CellDataType.FORMULA,
        //             cellValue="=B2+A2"
        //         },
        //         new(){
        //         dataType= CellDataType.FORMULA,
        //         cellValue="=SUM(B2,A2)"
        //         }
        //     });
        //     Worksheet worksheet1 = excel.AddSheet("formula");
        //     worksheet1.SetRow("A1", new ColumnCell[3]{
        //         new(){
        //             dataType= CellDataType.NUMBER,
        //             cellValue="2.524"
        //         },
        //         new(){
        //         dataType= CellDataType.NUMBER,
        //         cellValue="10"
        //         },
        //         new(){
        //         dataType= CellDataType.NUMBER,
        //         cellValue="29.75894855"
        //         }
        //     });
        //     worksheet1.SetRow("A3", new ColumnCell[3]{
        //         new(){
        //             dataType= CellDataType.FORMULA,
        //             cellValue="=A1+B1"
        //         },
        //         new(){
        //             dataType= CellDataType.FORMULA,
        //             cellValue="=SUM(A1:C1)"
        //         },
        //         new(){
        //         dataType= CellDataType.FORMULA,
        //         cellValue="=SUM(A1,C1)"
        //         }
        //     });
        //     excel1.SaveAs(string.Format("{1}/ReadEdit-Formula-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        // }

        // /// <summary>
        // /// 
        // /// </summary>
        // [TestMethod]
        // public void TestSheetView()
        // {
        //     Worksheet sheet = excel.AddSheet("Activated Cell");
        //     sheet.SetActiveCell("Z99");
        //     excel.SetActiveSheet("Activated Cell");
        //     Worksheet sheet2 = excel.AddSheet("View Applied");
        //     sheet2.SetSheetViewOptions(new WorkSheetViewOption()
        //     {
        //         showFormula = false,
        //         showGridLine = false,
        //         showGridLines = false,
        //         showRowColHeaders = false,
        //         showRuler = false,
        //         workSheetViewsValue = WorkSheetViewsValues.PAGE_LAYOUT,
        //         ZoomScale = 110
        //     });
        //     excel.AddSheet("Zoom 400").SetSheetViewOptions(new()
        //     {
        //         ZoomScale = 500 // Should auto correct to 400
        //     });
        //     excel.AddSheet("Zoom 10").SetSheetViewOptions(new()
        //     {
        //         ZoomScale = 0 // Should auto correct to 10
        //     });
        //     excel.AddSheet("Page Break").SetSheetViewOptions(new()
        //     {
        //         workSheetViewsValue = WorkSheetViewsValues.PAGE_BREAK_PREVIEW,
        //     });
        // }

        // /// <summary>
        // ///
        // /// </summary>
        // [TestMethod]
        // public void AddPicture()
        // {
        //     Worksheet worksheet = excel.AddSheet("Add Picture");
        //     Assert.IsNotNull(worksheet);
        //     worksheet.SetRow("D3", new ColumnCell[1]{
        //         new(){
        //             cellValue = "Re Update",
        //             dataType = CellDataType.STRING
        //         }
        //     }, new RowProperties()
        //     {
        //         height = 30
        //     });
        //     worksheet.AddPicture("./TestFiles/tom_and_jerry.jpg", new()
        //     {
        //         imageType = ImageType.JPEG,
        //         from = new()
        //         {
        //             column = 6,
        //             row = 6
        //         },
        //         to = new()
        //         {
        //             column = 8,
        //             row = 8
        //         }
        //     });
        //     Assert.IsTrue(true);
        // }
        // /// <summary>
        // ///
        // /// </summary>
        // [TestMethod]
        // public void AddPictureHyperlink()
        // {
        //     Worksheet worksheet = excel.AddSheet("hyperLink pic");
        //     Assert.IsNotNull(worksheet);
        //     worksheet.SetRow("D3", new ColumnCell[1]{
        //         new(){
        //             cellValue = "Re Update",
        //             dataType = CellDataType.STRING
        //         }
        //     }, new RowProperties()
        //     {
        //         height = 30
        //     });
        //     worksheet.AddPicture("./TestFiles/tom_and_jerry.jpg", new()
        //     {
        //         imageType = ImageType.JPEG,
        //         from = new()
        //         {
        //             column = 6,
        //             row = 6
        //         },
        //         to = new()
        //         {
        //             column = 8,
        //             row = 8
        //         },
        //         hyperlinkProperties = new()
        //         {
        //             value = "https://openxml-office.draviavemal.com/"
        //         }
        //     });
        //     Assert.IsTrue(true);
        // }
        // /// <summary>
        // /// Test All Chart Implementation
        // /// </summary>
        // [TestMethod]
        // public void AddAllCharts()
        // {
        //     Worksheet worksheet = excel.AddSheet("Area Chart");
        //     int row = 0;
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         areaChartSeriesSettings = new(){
        //             new(){
        //                 trendLines = new(){
        //                     new(){
        //                         trendLineType = TrendLineTypes.LINEAR,
        //                         trendLineName = "Dravia",
        //                         hexColor = "FF0000",
        //                         lineStye = DrawingPresetLineDashValues.LARGE_DASH
        //                     }
        //                 }
        //             },
        //             new(){
        //                 trendLines = new(){
        //                     new(){
        //                         trendLineType = TrendLineTypes.EXPONENTIAL,
        //                         trendLineName = "vemal",
        //                         hexColor = "FFFF00",
        //                         lineStye = DrawingPresetLineDashValues.DASH_DOT
        //                     }
        //                 }
        //             }
        //         },
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         areaChartDataLabel = new()
        //         {
        //             dataLabelPosition = AreaChartDataLabel.DataLabelPositionValues.SHOW,
        //             isBold = true
        //         },
        //         areaChartType = AreaChartTypes.STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         areaChartType = AreaChartTypes.PERCENT_STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 42,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 62,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         areaChartType = AreaChartTypes.CLUSTERED_3D,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 25
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 40
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         areaChartType = AreaChartTypes.STACKED_3D,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 25
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 40
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         areaChartType = AreaChartTypes.PERCENT_STACKED_3D,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 42,
        //                 column = 25
        //             },
        //             to = new()
        //             {
        //                 row = 62,
        //                 column = 40
        //             }
        //         }
        //     });
        //     row = 0;
        //     worksheet = excel.AddSheet("Bar Chart");
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new BarChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new BarChartSetting<ExcelSetting>()
        //     {
        //         barChartType = BarChartTypes.STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new BarChartSetting<ExcelSetting>()
        //     {
        //         barChartType = BarChartTypes.PERCENT_STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 42,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 62,
        //                 column = 20
        //             }
        //         }
        //     });
        //     row = 0;
        //     worksheet = excel.AddSheet("Column Chart");
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new ColumnChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new ColumnChartSetting<ExcelSetting>()
        //     {
        //         columnChartType = ColumnChartTypes.STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new ColumnChartSetting<ExcelSetting>()
        //     {
        //         columnChartType = ColumnChartTypes.PERCENT_STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 42,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 62,
        //                 column = 20
        //             }
        //         }
        //     });
        //     row = 0;
        //     worksheet = excel.AddSheet("Line Chart");
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         lineChartType = LineChartTypes.STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         lineChartType = LineChartTypes.PERCENT_STACKED,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 42,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 62,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         lineChartType = LineChartTypes.CLUSTERED_MARKER,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 21
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 36
        //             }
        //         }
        //     });
        //     excel.RemoveSheet("Sheet1");
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         lineChartType = LineChartTypes.STACKED_MARKER,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 21
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 36
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         lineChartType = LineChartTypes.PERCENT_STACKED_MARKER,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 42,
        //                 column = 21
        //             },
        //             to = new()
        //             {
        //                 row = 62,
        //                 column = 36
        //             }
        //         }
        //     });
        //     row = 0;
        //     worksheet = excel.AddSheet("Pie Chart");
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new PieChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new PieChartSetting<ExcelSetting>()
        //     {
        //         pieChartType = PieChartTypes.DOUGHNUT,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 20
        //             }
        //         }
        //     });
        //     row = 0;
        //     worksheet = excel.AddSheet("Scatter Chart");
        //     CommonMethod.CreateDataCellPayload(6, 6, true).ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartType = ScatterChartTypes.SCATTER_SMOOTH,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 35,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartType = ScatterChartTypes.SCATTER_SMOOTH_MARKER,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 36,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 50,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartType = ScatterChartTypes.SCATTER_STRAIGHT,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 22
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 37
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartType = ScatterChartTypes.SCATTER_STRAIGHT_MARKER,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 22
        //             },
        //             to = new()
        //             {
        //                 row = 35,
        //                 column = 37
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartSeriesSettings = new(){
        //             new(){
        //                 trendLines = new(){
        //                     new(){
        //                         trendLineType = TrendLineTypes.LINEAR,
        //                         trendLineName = "Dravia",
        //                         hexColor = "FF0000",
        //                         lineStye = DrawingPresetLineDashValues.LARGE_DASH
        //                     }
        //                 }
        //             },
        //             new(){
        //                 trendLines = new(){
        //                     new(){
        //                         trendLineType = TrendLineTypes.EXPONENTIAL,
        //                         trendLineName = "vemal",
        //                         hexColor = "FFFF00",
        //                         lineStye = DrawingPresetLineDashValues.DASH_DOT
        //                     }
        //                 }
        //             }
        //         },
        //         scatterChartType = ScatterChartTypes.BUBBLE,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 36,
        //                 column = 22
        //             },
        //             to = new()
        //             {
        //                 row = 50,
        //                 column = 37
        //             }
        //         }
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartType = ScatterChartTypes.BUBBLE_3D,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 40
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 55
        //             }
        //         }
        //     });
        //     row = 0;
        //     worksheet = excel.AddSheet("Combo Chart");
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     ComboChartSetting<ExcelSetting, CategoryAxis, ValueAxis, ValueAxis> comboChartSetting = new()
        //     {
        //         secondaryAxisPosition = AxisPosition.TOP,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 21,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 41,
        //                 column = 20
        //             }
        //         }
        //     };
        //     comboChartSetting.AddComboChartsSetting(new LineChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //     });
        //     comboChartSetting.AddComboChartsSetting(new BarChartSetting<ExcelSetting>()
        //     {
        //         isSecondaryAxis = true,
        //         applicationSpecificSetting = new()
        //     });
        //     comboChartSetting.AddComboChartsSetting(new ColumnChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, comboChartSetting);
        //     Assert.IsTrue(true);
        // }

        // /// <summary>
        // /// Test All Chart Implementation
        // /// </summary>
        // [TestMethod]
        // public void AddScatterCharts()
        // {
        //     Worksheet worksheet = excel.AddSheet("Only Scatter Chart");
        //     excel.RemoveSheet("Sheet1");
        //     int row = 0;
        //     CommonMethod.CreateDataCellPayload(6, 6, true).ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "F4"
        //     }, new ScatterChartSetting<ExcelSetting>()
        //     {
        //         scatterChartType = ScatterChartTypes.SCATTER,
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 6,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     Assert.IsTrue(true);
        // }

        // /// <summary>
        // /// Test existing file
        // /// </summary>
        // [TestMethod]
        // public void OpenExistingExcel()
        // {
        //     Excel excel1 = new("./TestFiles/basic_test.xlsx", true);
        //     Worksheet worksheet = excel1.AddSheet("AreaChart");
        //     int row = 0;
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new AreaChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     worksheet = excel1.AddSheet("LineChart");
        //     row = 0;
        //     CommonMethod.CreateDataCellPayload().ToList().ForEach(rowData =>
        //     {
        //         worksheet.SetRow(ConverterUtils.ConvertToExcelCellReference(++row, 1), rowData, new());
        //     });
        //     worksheet.AddChart(new()
        //     {
        //         cellIdStart = "A1",
        //         cellIdEnd = "D4"
        //     }, new LineChartSetting<ExcelSetting>()
        //     {
        //         applicationSpecificSetting = new()
        //         {
        //             from = new()
        //             {
        //                 row = 5,
        //                 column = 5
        //             },
        //             to = new()
        //             {
        //                 row = 20,
        //                 column = 20
        //             }
        //         }
        //     });
        //     excel1.SaveAs(string.Format("{1}/Edit-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        //     Assert.IsTrue(true);
        // }

        /// <summary>
        /// Test existing file
        /// </summary>
        [TestMethod]
        public void OpenExistingExcelStyleString()
        {
            Excel excel1 = new("./test_files/basic_test.xlsx");
            excel1.SaveAs(string.Format("{1}/EditStyle-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
            Assert.IsTrue(true);
        }
    }
}
