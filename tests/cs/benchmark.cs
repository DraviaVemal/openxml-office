// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

using System.Diagnostics;
using System.Text;
using CurrentSpreadsheet = draviavemal.openxml_office.spreadsheet_2007;
using LegacySpreadsheet = OpenXMLOffice.Spreadsheet_2007;

namespace openxmloffice.tests
{
    /// <summary>
    /// Standalone benchmark that compares the current package against the published
    /// OpenXMLOffice.Spreadsheet v2.9.0 NuGet package.
    /// Measures execution time, RAM usage, peak RAM and peak CPU for a range of cell counts.
    /// This class is fully self contained and does not touch the existing functional tests.
    /// </summary>
    [TestClass]
    public class Benchmark
    {
        private static readonly string resultPath = "../../../test_results";
        private static readonly string tempPath = Path.Combine(resultPath, "benchmark_temp");
        private static readonly int[] cellCounts = { 10, 100, 1000, 10000, 100000 };
        private const int ColumnsPerRow = 100;

        /// <summary>
        /// MSTest injected context, used to surface the report in the test output.
        /// </summary>
        public TestContext TestContext { get; set; } = null!;

        /// <summary>
        /// Prepare output folders.
        /// </summary>
        [ClassInitialize]
        public static void ClassInitialize(TestContext context)
        {
            if (!Directory.Exists(resultPath))
            {
                Directory.CreateDirectory(resultPath);
            }
            if (Directory.Exists(tempPath))
            {
                Directory.Delete(tempPath, true);
            }
            Directory.CreateDirectory(tempPath);
        }

        /// <summary>
        /// Remove generated benchmark workbooks.
        /// </summary>
        [ClassCleanup]
        public static void ClassCleanup()
        {
            if (Directory.Exists(tempPath))
            {
                Directory.Delete(tempPath, true);
            }
        }

        /// <summary>
        /// Runs the write benchmark for both libraries across all configured cell counts
        /// and writes a Markdown + CSV report to the test_results folder.
        /// </summary>
        [TestMethod]
        public void CompareWritePerformance()
        {
            List<BenchmarkResult> results = new();

            // Warm up the JIT / assembly loading for both libraries so the first
            // measured run is not penalised by one time startup costs.
            WriteCurrentPackage(10, "warmup");
            WriteLegacyPackage(10, "warmup");

            foreach (int count in cellCounts)
            {
                results.Add(Measure("v4.x (project reference)", count, tag => WriteCurrentPackage(count, tag)));
                results.Add(Measure("v2.9.0", count, tag => WriteLegacyPackage(count, tag)));
            }

            WriteReport(results);
            Assert.AreEqual(cellCounts.Length * 2, results.Count);
        }

        /// <summary>
        /// Measures a broad set of metrics while the supplied workload runs: wall clock
        /// time, throughput, per-cell latency, total managed allocations, retained managed
        /// RAM, working set / private memory growth and peaks, average and peak CPU, GC
        /// collection counts per generation and the produced workbook size. RAM/CPU peaks
        /// are captured by a background sampler polling the v4.x process.
        /// The workload returns the path of the file it produced so its size can be recorded.
        /// </summary>
        private BenchmarkResult Measure(string library, int cellCount, Func<string, string> workload)
        {
            // Establish a clean baseline.
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();

            Process process = Process.GetCurrentProcess();
            long managedBefore = GC.GetTotalMemory(true);
            long allocatedBefore = GC.GetTotalAllocatedBytes(true);
            int gen0Before = GC.CollectionCount(0);
            int gen1Before = GC.CollectionCount(1);
            int gen2Before = GC.CollectionCount(2);
            TimeSpan pauseBefore = GC.GetTotalPauseDuration();
            process.Refresh();
            long workingSetBefore = process.WorkingSet64;
            long privateBefore = process.PrivateMemorySize64;

            long peakWorkingSet = workingSetBefore;
            long peakPrivate = privateBefore;
            double peakCpuPercent = 0;
            bool monitoring = true;

            TimeSpan cpuAtStart = process.TotalProcessorTime;
            Stopwatch stopwatch = Stopwatch.StartNew();
            long lastWallTicks = stopwatch.ElapsedTicks;
            TimeSpan lastCpuTime = cpuAtStart;

            Thread sampler = new(() =>
            {
                while (Volatile.Read(ref monitoring))
                {
                    Thread.Sleep(15);
                    process.Refresh();

                    long workingSet = process.WorkingSet64;
                    if (workingSet > peakWorkingSet)
                    {
                        peakWorkingSet = workingSet;
                    }

                    long privateMemory = process.PrivateMemorySize64;
                    if (privateMemory > peakPrivate)
                    {
                        peakPrivate = privateMemory;
                    }

                    long nowWallTicks = stopwatch.ElapsedTicks;
                    TimeSpan nowCpuTime = process.TotalProcessorTime;
                    double wallMs = (nowWallTicks - lastWallTicks) * 1000.0 / Stopwatch.Frequency;
                    double cpuMs = (nowCpuTime - lastCpuTime).TotalMilliseconds;
                    if (wallMs > 0)
                    {
                        double cpuPercent = cpuMs / (wallMs * Environment.ProcessorCount) * 100.0;
                        if (cpuPercent > peakCpuPercent)
                        {
                            peakCpuPercent = cpuPercent;
                        }
                    }
                    lastWallTicks = nowWallTicks;
                    lastCpuTime = nowCpuTime;
                }
            })
            {
                IsBackground = true,
                Name = "benchmark-sampler"
            };
            sampler.Start();

            string outputPath = workload("run");

            stopwatch.Stop();
            Volatile.Write(ref monitoring, false);
            sampler.Join();

            process.Refresh();
            TimeSpan cpuAtEnd = process.TotalProcessorTime;
            long workingSetAfter = process.WorkingSet64;
            long managedAfter = GC.GetTotalMemory(false);
            long allocatedAfter = GC.GetTotalAllocatedBytes(true);
            TimeSpan pauseAfter = GC.GetTotalPauseDuration();
            GCMemoryInfo gcInfo = GC.GetGCMemoryInfo();

            const double toMb = 1024.0 * 1024.0;
            double elapsedMs = stopwatch.Elapsed.TotalMilliseconds;
            double elapsedSeconds = elapsedMs / 1000.0;
            double cpuTotalMs = (cpuAtEnd - cpuAtStart).TotalMilliseconds;
            long allocatedBytes = allocatedAfter - allocatedBefore;
            double allocatedMb = allocatedBytes / toMb;
            double gcPauseMs = (pauseAfter - pauseBefore).TotalMilliseconds;
            long fileSize = File.Exists(outputPath) ? new FileInfo(outputPath).Length : 0;

            return new BenchmarkResult
            {
                Library = library,
                CellCount = cellCount,
                ElapsedMs = elapsedMs,
                CellsPerSecond = elapsedSeconds > 0 ? cellCount / elapsedSeconds : 0,
                MicrosPerCell = cellCount > 0 ? elapsedMs * 1000.0 / cellCount : 0,
                AllocatedMb = allocatedMb,
                AllocatedBytesPerCell = cellCount > 0 ? (double)allocatedBytes / cellCount : 0,
                AllocationRateMbPerSec = elapsedSeconds > 0 ? allocatedMb / elapsedSeconds : 0,
                ManagedRamMb = (managedAfter - managedBefore) / toMb,
                WorkingSetDeltaMb = (workingSetAfter - workingSetBefore) / toMb,
                PeakWorkingSetMb = peakWorkingSet / toMb,
                PeakPrivateMemoryMb = peakPrivate / toMb,
                AvgCpuPercent = elapsedMs > 0 ? cpuTotalMs / (elapsedMs * Environment.ProcessorCount) * 100.0 : 0,
                PeakCpuPercent = peakCpuPercent,
                Gen0Collections = GC.CollectionCount(0) - gen0Before,
                Gen1Collections = GC.CollectionCount(1) - gen1Before,
                Gen2Collections = GC.CollectionCount(2) - gen2Before,
                GcPauseMs = gcPauseMs,
                GcPausePercent = elapsedMs > 0 ? gcPauseMs / elapsedMs * 100.0 : 0,
                HeapSizeMb = gcInfo.HeapSizeBytes / toMb,
                FragmentedMb = gcInfo.FragmentedBytes / toMb,
                CommittedMb = gcInfo.TotalCommittedBytes / toMb,
                OutputFileKb = fileSize / 1024.0
            };
        }

        /// <summary>
        /// Writes the requested number of cells using the v4.x (project reference) package.
        /// Returns the path of the saved workbook.
        /// </summary>
        private string WriteCurrentPackage(int cellCount, string tag)
        {
            CurrentSpreadsheet.Excel excel = new(new CurrentSpreadsheet.ExcelProperties
            {
                isEditable = true
            });
            CurrentSpreadsheet.Worksheet worksheet = excel.AddSheet("Benchmark");

            int columns = Math.Min(cellCount, ColumnsPerRow);
            int written = 0;
            int row = 1;
            while (written < cellCount)
            {
                int rowCount = Math.Min(columns, cellCount - written);
                CurrentSpreadsheet.CellProperty[] cells = new CurrentSpreadsheet.CellProperty[rowCount];
                for (int column = 0; column < rowCount; column++)
                {
                    cells[column] = new CurrentSpreadsheet.CellProperty
                    {
                        Value = (written + column).ToString(),
                        DataType = CurrentSpreadsheet.CellDataType.Number
                    };
                }
                worksheet.SetCellRefValues($"A{row}", cells);
                written += rowCount;
                row++;
            }

            // The v4.x package requires open worksheets to be released before saving.
            worksheet.Dispose();
            string path = Path.Combine(tempPath, $"v4x_{cellCount}_{tag}.xlsx");
            excel.SaveAs(path);
            return path;
        }

        /// <summary>
        /// Writes the requested number of cells using the legacy OpenXMLOffice.Spreadsheet 2.9.0 package.
        /// Returns the path of the saved workbook.
        /// </summary>
        private string WriteLegacyPackage(int cellCount, string tag)
        {
            LegacySpreadsheet.Excel excel = new(new LegacySpreadsheet.ExcelProperties());
            LegacySpreadsheet.Worksheet worksheet = excel.AddSheet("Benchmark");

            int columns = Math.Min(cellCount, ColumnsPerRow);
            int written = 0;
            int row = 1;
            while (written < cellCount)
            {
                int rowCount = Math.Min(columns, cellCount - written);
                LegacySpreadsheet.ColumnCell[] cells = new LegacySpreadsheet.ColumnCell[rowCount];
                for (int column = 0; column < rowCount; column++)
                {
                    cells[column] = new LegacySpreadsheet.ColumnCell
                    {
                        cellValue = (written + column).ToString(),
                        dataType = LegacySpreadsheet.CellDataType.NUMBER
                    };
                }
                worksheet.SetRow($"A{row}", cells);
                written += rowCount;
                row++;
            }

            string path = Path.Combine(tempPath, $"legacy_{cellCount}_{tag}.xlsx");
            excel.SaveAs(path);
            return path;
        }

        /// <summary>
        /// Renders the collected results as a detailed Markdown report (raw metrics table +
        /// head-to-head comparison) plus a CSV, writes both to the test_results folder and
        /// echoes the report to the test output.
        /// </summary>
        private void WriteReport(List<BenchmarkResult> results)
        {
            string timestamp = DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss");

            StringBuilder markdown = new();
            markdown.AppendLine("# Spreadsheet Write Benchmark");
            markdown.AppendLine();
            markdown.AppendLine($"- Generated: {DateTime.Now:u}");
            markdown.AppendLine($"- Machine: {Environment.MachineName} ({Environment.ProcessorCount} logical CPUs)");
            markdown.AppendLine($"- Runtime: {Environment.Version} on {System.Runtime.InteropServices.RuntimeInformation.OSDescription.Trim()}");
            markdown.AppendLine($"- Layout: cells written in rows of up to {ColumnsPerRow} columns.");
            markdown.AppendLine();
            markdown.AppendLine("Notes:");
            markdown.AppendLine("- **Alloc** = total managed bytes allocated during the run (churn), **Managed Δ** = managed heap retained afterwards.");
            markdown.AppendLine("- **WS** = OS working set, **Priv** = private memory. CPU % is normalised across all logical processors (100% = one fully busy core-equivalent).");
            markdown.AppendLine("- **µs/cell** = wall time per cell, **Cells/s** = throughput.");
            markdown.AppendLine();

            // Executive summary: aggregate the per-size head-to-head into a few takeaways.
            List<double> timeSpeedups = new();
            List<double> allocRatios = new();
            double currentTotalAlloc = 0;
            double legacyTotalAlloc = 0;
            int currentTotalGcPauses = 0;
            int legacyTotalGcPauses = 0;
            double currentTotalGcPauseMs = 0;
            double legacyTotalGcPauseMs = 0;
            foreach (int count in cellCounts)
            {
                BenchmarkResult? cur = results.Find(x => x.CellCount == count && x.Library.StartsWith("v4.x", StringComparison.Ordinal));
                BenchmarkResult? leg = results.Find(x => x.CellCount == count && !x.Library.StartsWith("v4.x", StringComparison.Ordinal));
                if (cur == null || leg == null)
                {
                    continue;
                }
                if (cur.ElapsedMs > 0)
                {
                    timeSpeedups.Add(leg.ElapsedMs / cur.ElapsedMs);
                }
                if (cur.AllocatedMb > 0)
                {
                    allocRatios.Add(leg.AllocatedMb / cur.AllocatedMb);
                }
                currentTotalAlloc += cur.AllocatedMb;
                legacyTotalAlloc += leg.AllocatedMb;
                currentTotalGcPauses += cur.Gen0Collections + cur.Gen1Collections + cur.Gen2Collections;
                legacyTotalGcPauses += leg.Gen0Collections + leg.Gen1Collections + leg.Gen2Collections;
                currentTotalGcPauseMs += cur.GcPauseMs;
                legacyTotalGcPauseMs += leg.GcPauseMs;
            }

            markdown.AppendLine("## Summary");
            markdown.AppendLine();
            markdown.AppendLine("| Metric (across all sizes) | v4.x | v2.9.0 |");
            markdown.AppendLine("| --- | ---: | ---: |");
            markdown.AppendLine($"| Total managed allocation churn (MB) | {currentTotalAlloc:0.00} | {legacyTotalAlloc:0.00} |");
            markdown.AppendLine($"| Total GC collections (Gen0+1+2) | {currentTotalGcPauses} | {legacyTotalGcPauses} |");
            markdown.AppendLine($"| Total GC pause time (ms) | {currentTotalGcPauseMs:0.00} | {legacyTotalGcPauseMs:0.00} |");
            markdown.AppendLine();
            markdown.AppendLine($"- v4.x is **{Average(timeSpeedups):0.0}× faster** on average (range {Min(timeSpeedups):0.0}×–{Max(timeSpeedups):0.0}×).");
            markdown.AppendLine($"- v4.x allocates **{Average(allocRatios):0.0}× less** managed memory on average (range {Min(allocRatios):0.0}×–{Max(allocRatios):0.0}×).");
            markdown.AppendLine($"- v4.x triggered **{currentTotalGcPauses}** GC collections vs **{legacyTotalGcPauses}** for the legacy package.");
            markdown.AppendLine();

            markdown.AppendLine("## Raw metrics");
            markdown.AppendLine();
            markdown.AppendLine("| Library | Cells | Time (ms) | µs/cell | Cells/s | Alloc (MB) | Managed Δ (MB) | WS Δ (MB) | Peak WS (MB) | Peak Priv (MB) | Avg CPU (%) | Peak CPU (%) | GC 0/1/2 | File (KB) |");
            markdown.AppendLine("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :---: | ---: |");

            StringBuilder csv = new();
            csv.AppendLine("Library,Cells,TimeMs,MicrosPerCell,CellsPerSecond,AllocatedMb,AllocBytesPerCell,AllocRateMbPerSec,ManagedRamDeltaMb,WorkingSetDeltaMb,PeakWorkingSetMb,PeakPrivateMemoryMb,AvgCpuPercent,PeakCpuPercent,Gen0,Gen1,Gen2,GcPauseMs,GcPausePercent,HeapSizeMb,FragmentedMb,CommittedMb,OutputFileKb");

            foreach (BenchmarkResult r in results)
            {
                markdown.AppendLine(
                    $"| {r.Library} | {r.CellCount} | {r.ElapsedMs:0.00} | {r.MicrosPerCell:0.00} | {r.CellsPerSecond:0} | {r.AllocatedMb:0.00} | {r.ManagedRamMb:0.00} | {r.WorkingSetDeltaMb:0.00} | {r.PeakWorkingSetMb:0.00} | {r.PeakPrivateMemoryMb:0.00} | {r.AvgCpuPercent:0.0} | {r.PeakCpuPercent:0.0} | {r.Gen0Collections}/{r.Gen1Collections}/{r.Gen2Collections} | {r.OutputFileKb:0.00} |");
                csv.AppendLine(
                    $"{r.Library},{r.CellCount},{r.ElapsedMs:0.00},{r.MicrosPerCell:0.00},{r.CellsPerSecond:0},{r.AllocatedMb:0.00},{r.AllocatedBytesPerCell:0},{r.AllocationRateMbPerSec:0.00},{r.ManagedRamMb:0.00},{r.WorkingSetDeltaMb:0.00},{r.PeakWorkingSetMb:0.00},{r.PeakPrivateMemoryMb:0.00},{r.AvgCpuPercent:0.0},{r.PeakCpuPercent:0.0},{r.Gen0Collections},{r.Gen1Collections},{r.Gen2Collections},{r.GcPauseMs:0.00},{r.GcPausePercent:0.00},{r.HeapSizeMb:0.00},{r.FragmentedMb:0.00},{r.CommittedMb:0.00},{r.OutputFileKb:0.00}");
            }

            // GC / memory stress focus: allocation churn and collector pressure are the
            // clearest indicators of managed memory stress for a workload.
            markdown.AppendLine();
            markdown.AppendLine("## GC / memory stress");
            markdown.AppendLine();
            markdown.AppendLine("Highlights managed allocation churn and garbage collector pressure. High Gen0/1/2 counts, GC pause time and allocation rate indicate heavier memory stress.");
            markdown.AppendLine();
            markdown.AppendLine("| Library | Cells | Alloc (MB) | Alloc/cell (bytes) | Alloc rate (MB/s) | Gen0 | Gen1 | Gen2 | GC pause (ms) | GC pause (%) | Heap (MB) | Fragmented (MB) | Committed (MB) |");
            markdown.AppendLine("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");

            foreach (BenchmarkResult r in results)
            {
                markdown.AppendLine(
                    $"| {r.Library} | {r.CellCount} | {r.AllocatedMb:0.00} | {r.AllocatedBytesPerCell:0} | {r.AllocationRateMbPerSec:0.00} | {r.Gen0Collections} | {r.Gen1Collections} | {r.Gen2Collections} | {r.GcPauseMs:0.00} | {r.GcPausePercent:0.00} | {r.HeapSizeMb:0.00} | {r.FragmentedMb:0.00} | {r.CommittedMb:0.00} |");
            }

            // Head-to-head: how many times better the v4.x package is than the legacy one.
            markdown.AppendLine();
            markdown.AppendLine("## Head-to-head: v4.x vs v2.9.0");
            markdown.AppendLine();
            markdown.AppendLine("Each cell is a multiplier comparing the two libraries at the same cell count. **Higher is always better for v4.x**, and `1.00×` means they are equal.");
            markdown.AppendLine();
            markdown.AppendLine("For example, `500.00×` under *Faster* means v4.x finished the work 500 times faster than v2.9.0; `120.00×` under *Less RAM churn* means v4.x allocated 120 times less managed memory.");
            markdown.AppendLine();
            markdown.AppendLine("| Cells | Faster (time) | Higher throughput | Less RAM churn (alloc) | Less GC pause | Lower peak RAM | Lower peak private RAM | Smaller file |");
            markdown.AppendLine("| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");

            foreach (int count in cellCounts)
            {
                BenchmarkResult? v4x = results.Find(x => x.CellCount == count && x.Library.StartsWith("v4.x", StringComparison.Ordinal));
                BenchmarkResult? legacy = results.Find(x => x.CellCount == count && !x.Library.StartsWith("v4.x", StringComparison.Ordinal));
                if (v4x == null || legacy == null)
                {
                    continue;
                }
                markdown.AppendLine(
                    $"| {count} | {Times(legacy.ElapsedMs, v4x.ElapsedMs)} | {Times(v4x.CellsPerSecond, legacy.CellsPerSecond)} | {Times(legacy.AllocatedMb, v4x.AllocatedMb)} | {Times(legacy.GcPauseMs, v4x.GcPauseMs)} | {Times(legacy.PeakWorkingSetMb, v4x.PeakWorkingSetMb)} | {Times(legacy.PeakPrivateMemoryMb, v4x.PeakPrivateMemoryMb)} | {Times(legacy.OutputFileKb, v4x.OutputFileKb)} |");
            }

            string markdownPath = Path.Combine(resultPath, $"benchmark-report-{timestamp}.md");
            string csvPath = Path.Combine(resultPath, $"benchmark-report-{timestamp}.csv");
            File.WriteAllText(markdownPath, markdown.ToString());
            File.WriteAllText(csvPath, csv.ToString());

            TestContext.WriteLine(markdown.ToString());
            TestContext.WriteLine($"Full report written to: {Path.GetFullPath(markdownPath)}");
            TestContext.WriteLine($"CSV data: {Path.GetFullPath(csvPath)}");
            Console.WriteLine(markdown.ToString());
        }

        /// <summary>
        /// Formats a comparison as an "N.NN×" multiplier, or "n/a" when it cannot be computed.
        /// </summary>
        private static string Times(double numerator, double denominator)
        {
            if (denominator <= 0)
            {
                return "n/a";
            }
            return $"{numerator / denominator:0.00}×";
        }

        private static double Average(List<double> values) => values.Count == 0 ? 0 : values.Sum() / values.Count;

        private static double Min(List<double> values) => values.Count == 0 ? 0 : values.Min();

        private static double Max(List<double> values) => values.Count == 0 ? 0 : values.Max();

        /// <summary>
        /// Single benchmark measurement row.
        /// </summary>
        private sealed class BenchmarkResult
        {
            public string Library { get; set; } = string.Empty;
            public int CellCount { get; set; }
            public double ElapsedMs { get; set; }
            public double CellsPerSecond { get; set; }
            public double MicrosPerCell { get; set; }
            public double AllocatedMb { get; set; }
            public double AllocatedBytesPerCell { get; set; }
            public double AllocationRateMbPerSec { get; set; }
            public double ManagedRamMb { get; set; }
            public double WorkingSetDeltaMb { get; set; }
            public double PeakWorkingSetMb { get; set; }
            public double PeakPrivateMemoryMb { get; set; }
            public double AvgCpuPercent { get; set; }
            public double PeakCpuPercent { get; set; }
            public int Gen0Collections { get; set; }
            public int Gen1Collections { get; set; }
            public int Gen2Collections { get; set; }
            public double GcPauseMs { get; set; }
            public double GcPausePercent { get; set; }
            public double HeapSizeMb { get; set; }
            public double FragmentedMb { get; set; }
            public double CommittedMb { get; set; }
            public double OutputFileKb { get; set; }
        }
    }
}
