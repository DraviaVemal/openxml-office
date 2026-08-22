// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

using draviavemal.openxml_office.global_2007;
using draviavemal.openxml_office.spreadsheet_2007;

namespace openxmloffice.tests
{/// <summary>
 /// Excel Test
 /// </summary>
    [TestClass]
    public class Spreadsheet
    {
        private static readonly string resultPath = "../../../test_results";
        private static readonly Excel excel = new(new ExcelProperties
        {
            isEditable = true,
            // coreProperties = new()
            // {
            //     title = "Test File",
            //     creator = "OpenXML-Office",
            //     subject = "Test Subject",
            //     tags = "Test",
            //     category = "Test Category",
            //     description = "Describe the test file"
            // }
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
            using Worksheet _ = excel.AddSheet();
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
            using Worksheet worksheet = excel.AddSheet("TestSheet1");
            Assert.IsNotNull(worksheet);
        }

        /// <summary>
        /// Add Sheet Test
        /// </summary>
        [TestMethod]
        public void AddSecondSheet()
        {
            using Worksheet worksheet = excel.AddSheet("TestSheet2");
            Assert.IsNotNull(worksheet);
        }

        /// <summary>
        /// Rename Sheet Based on Index Test
        /// </summary>
        [TestMethod]
        public void RenameBySheetId()
        {
            using Worksheet worksheet = excel.AddSheet("TestSheet3");
            Assert.IsNotNull(worksheet);
            Assert.IsTrue(excel.RenameSheet("TestSheet3", "RenameTestSheet3"));
        }

        /// <summary>
        /// Rename Sheet Based on Index Test
        /// </summary>
        [TestMethod]
        public void RenameSheet()
        {
            using Worksheet worksheet = excel.AddSheet("RenamedSheet11");
            Assert.IsNotNull(worksheet);
            Assert.IsTrue(excel.RenameSheet("RenamedSheet11", "RenameSheet11"));
        }

        /// <summary>
        /// Set Cell Test
        /// </summary>
        [TestMethod]
        public void SetColumn()
        {
            using Worksheet worksheet = excel.AddSheet("Data3 Column properties");
            Assert.IsNotNull(worksheet);
            worksheet.SetColumnRefProperties("A1", new ColumnProperties()
            {
                Width = 30
            });
            worksheet.SetColumnRefProperties("C4", new ColumnProperties()
            {
                Width = 30,
                BestFit = true
            });
            worksheet.SetColumnRefProperties("G7", new ColumnProperties()
            {
                Hidden = true
            });
            Assert.IsTrue(true);
        }

        /// <summary>
        /// Set Row Test
        /// </summary>
        [TestMethod]
        public void SetRow()
        {
            using Worksheet worksheet = excel.AddSheet("Data2");
            StyleId styleId = excel.GetStyleId(new CellStyleSetting()
            {
                CustomNumberFormat = "00.000",
            });
            Assert.IsNotNull(worksheet);
            worksheet.SetCellRefValues("A1", new CellProperty[6]{
                new(){
                    Value = "test1",
                    DataType = CellDataType.String
                },
                 new(){
                    Value = "test2",
                    DataType = CellDataType.String
                },
                 new(){
                    Value = "test3",
                    DataType = CellDataType.String
                },
                 new(){
                    Value = "test4",
                    DataType = CellDataType.String,
                    StyleId = excel.GetStyleId(new CellStyleSetting()
                    {
                        FontSize = 20
                    })
                },
                 new(){
                    Value = "2.51",
                    DataType = CellDataType.Number,
                    StyleId=styleId
                },new(){
                    Value = "5.51",
                    DataType = CellDataType.Number,
                    StyleId = excel.GetStyleId(new CellStyleSetting()
                    {
                        CustomNumberFormat = "₹ #,##0.00;₹ -#,##0.00",
                    })
                }
            });
            worksheet.SetCellRefValues("C1", new CellProperty[1]{
                new(){
                    Value = "Re Update",
                    DataType = CellDataType.String
                }
            });
            Assert.IsTrue(true);
        }

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
        //             value = "https://docs.draviavemal.com/openxml-office/"
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
            Excel excel1 = new("./edit_test_files/basic_test.xlsx");
            excel1.SaveAs(string.Format("{1}/EditStyle-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
            Assert.IsTrue(true);
        }

        // ── Excel workbook-level methods ──────────────────────────────────────

        [TestMethod]
        public void SetActiveSheet()
        {
            using Worksheet ws = excel.AddSheet("ActiveSheetTarget");
            excel.SetActiveSheet("ActiveSheetTarget");
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void SetVisibility()
        {
            excel.SetVisibility(true);
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void MinimizeWorkbook()
        {
            excel.MinimizeWorkbook(true);
            excel.MinimizeWorkbook(false);
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void HideSheetTabs()
        {
            excel.HideSheetTabs(true);
            excel.HideSheetTabs(false);
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void HideVerticalScroll()
        {
            excel.HideVerticalScroll(true);
            excel.HideVerticalScroll(false);
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void HideHorizontalScroll()
        {
            excel.HideHorizontalScroll(true);
            excel.HideHorizontalScroll(false);
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void HideSheet()
        {
            using Worksheet ws = excel.AddSheet("HiddenSheet");
            excel.HideSheet("HiddenSheet");
            Assert.IsTrue(true);
        }

        // ── Worksheet-level methods ───────────────────────────────────────────

        [TestMethod]
        public void SetAndListMergeCell()
        {
            using Worksheet ws = excel.AddSheet("MergeCell");
            ws.SetMergeCell(new ReferenceRange { ColumnStart = 1, ColumnEnd = 3, RowStart = 1, RowEnd = 2 });
            ws.SetMergeCell(new ReferenceRange { ColumnStart = 5, ColumnEnd = 7, RowStart = 5, RowEnd = 6 });
            ReferenceRange[] merged = ws.ListMergeCell();
            Assert.IsNotNull(merged);
            Assert.IsTrue(merged.Length >= 2);
        }

        [TestMethod]
        public void RemoveMergeCell()
        {
            using Worksheet ws = excel.AddSheet("RemoveMerge");
            ReferenceRange range = new() { ColumnStart = 1, ColumnEnd = 3, RowStart = 1, RowEnd = 2 };
            ws.SetMergeCell(range);
            ws.RemoveMergeCell(range);
            ReferenceRange[] merged = ws.ListMergeCell();
            Assert.IsNotNull(merged);
            Assert.AreEqual(0, merged.Length);
        }

        [TestMethod]
        public void SetAndListHyperlinks()
        {
            using Worksheet ws = excel.AddSheet("Hyperlinks");
            ws.SetHyperlink("https://docs.draviavemal.com/openxml-office/", new ReferenceRange { ColumnStart = 1, ColumnEnd = 1, RowStart = 1, RowEnd = 1 }, "OpenXML-Office");
            ws.SetHyperlink("https://github.com/DraviaVemal/openxml-office", new ReferenceRange { ColumnStart = 2, ColumnEnd = 2, RowStart = 1, RowEnd = 1 });
            HyperlinkInfo[] links = ws.ListHyperlinks();
            Assert.IsNotNull(links);
            Assert.IsTrue(links.Length >= 2);
            Assert.AreEqual("OpenXML-Office", links[0].Display);
        }

        [TestMethod]
        public void RemoveHyperlink()
        {
            using Worksheet ws = excel.AddSheet("RemoveHyperlink");
            ReferenceRange range = new() { ColumnStart = 1, ColumnEnd = 1, RowStart = 1, RowEnd = 1 };
            ws.SetHyperlink("https://docs.draviavemal.com/openxml-office/", range, "Test");
            ws.RemoveHyperlink(range);
            HyperlinkInfo[] links = ws.ListHyperlinks();
            Assert.IsNotNull(links);
            Assert.AreEqual(0, links.Length);
        }

        [TestMethod]
        public void GetRangeCellProperties()
        {
            using Worksheet ws = excel.AddSheet("ReadCells");
            ws.SetCellRefValues("A1", new CellProperty[]
            {
                new() { Value = "Alpha", DataType = CellDataType.String },
                new() { Value = "Beta",  DataType = CellDataType.String },
                new() { Value = "42",    DataType = CellDataType.Number },
            });
            CellPackage[] packages = ws.GetRangeCellProperties(new ReferenceRange
            {
                ColumnStart = 1, ColumnEnd = 3,
                RowStart    = 1, RowEnd    = 1,
            });
            Assert.IsNotNull(packages);
            Assert.IsTrue(packages.Length >= 3);
        }

        [TestMethod]
        public void AddPicture()
        {
            using Worksheet ws = excel.AddSheet("PictureSheet");
            ws.AddPicture("./edit_test_files/tom_and_jerry.jpg", new ExcelPictureSetting
            {
                ImageType = ImageType.JPEG,
                From = new AnchorPosition { Column = 2, Row = 2 },
                To   = new AnchorPosition { Column = 5, Row = 8 },
            });
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void DeleteSheet()
        {
            Worksheet ws = excel.AddSheet("ToDelete");
            ws.DeleteSheet();
            Assert.IsTrue(true);
        }

        [TestMethod]
        public void SetRowProperties()
        {
            using Worksheet worksheet = excel.AddSheet("RowProperties");
            worksheet.SetRowindexProperties(1, new RowProperties { Height = 100 });
            worksheet.SetRowindexProperties(3, new RowProperties { Hidden = true });
            worksheet.SetRowindexProperties(5, new RowProperties { TickTop = true });
            worksheet.SetRowindexProperties(7, new RowProperties { ThickBottom = true });
            Assert.IsNotNull(worksheet);
        }

        [TestMethod]
        public void SetCellStyles()
        {
            using Worksheet worksheet = excel.AddSheet("CellStyles");
            StyleId boldId = excel.GetStyleId(new CellStyleSetting { IsBold = true });
            StyleId italicId = excel.GetStyleId(new CellStyleSetting { IsItalic = true });
            StyleId underlineId = excel.GetStyleId(new CellStyleSetting { IsUnderline = true });
            StyleId doubleUnderlineId = excel.GetStyleId(new CellStyleSetting { IsDoubleUnderline = true });
            StyleId wrapTextId = excel.GetStyleId(new CellStyleSetting { IsWrapText = true });
            worksheet.SetCellRefValues("A1", new CellProperty[]
            {
                new() { Value = "Bold", DataType = CellDataType.String, StyleId = boldId },
                new() { Value = "Italic", DataType = CellDataType.String, StyleId = italicId },
                new() { Value = "Underline", DataType = CellDataType.String, StyleId = underlineId },
                new() { Value = "Double Underline", DataType = CellDataType.String, StyleId = doubleUnderlineId },
                new() { Value = "This is a very long line to wrap the column. Test the wrap string", DataType = CellDataType.String, StyleId = wrapTextId },
            });
            Assert.IsNotNull(worksheet);
        }

        [TestMethod]
        public void EditExistingMergeAndHyperlinks()
        {
            Excel editExcel = new("./edit_test_files/merge_links.xlsx", new ExcelProperties { isEditable = true });
            Worksheet worksheet = editExcel.GetWorksheet("edit");
            ReferenceRange[] merges = worksheet.ListMergeCell();
            Assert.IsNotNull(merges);
            Assert.IsTrue(merges.Length > 0);
            HyperlinkInfo[] links = worksheet.ListHyperlinks();
            Assert.IsNotNull(links);
            Assert.IsTrue(links.Length > 0);
            worksheet.RemoveMergeCell(merges[0]);
            worksheet.Dispose();
            editExcel.SaveAs(string.Format("{1}/EditMergeLinks-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        }
    }
}
