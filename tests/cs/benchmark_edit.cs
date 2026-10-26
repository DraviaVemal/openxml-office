// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

using System.Diagnostics;
using System.Globalization;
using System.Text;
using CurrentSpreadsheet = draviavemal.openxml_office.spreadsheet_2007;
using LegacySpreadsheet = OpenXMLOffice.Spreadsheet_2007;

namespace openxmloffice.tests
{
    /// <summary>Open, edit and save benchmarks against the published package.</summary>
    [TestClass]
    public class OpenEditSaveBenchmark
    {
        private static readonly string resultPath = "../../../test_results";
        private static readonly string tempPath = Path.Combine(resultPath, "benchmark_edit_temp");
        private static readonly string historyPath = Path.GetFullPath("../../../../benchmark_history/cs.csv");
        private static readonly string versionFile = Path.GetFullPath("../../../../../rs/Cargo.toml");
        private const string HistoryHeader = "timestamp,version,commit,library,scenario,phase,time_ms,min_ms,max_ms,alloc_mb,alloc_count,peak_rss_mb,out_kb";
        private const string CurrentLibrary = "v4.x";
        private const string LegacyLibrary = "v2.9.0";

        public TestContext TestContext { get; set; } = null!;

        private delegate string Runner(string source, string sheet, bool edit, double[] phases);

        /// <summary>Expose the legacy protected constructor.</summary>
        private sealed class LegacyPrivacy : OpenXMLOffice.Global_2007.PrivacyProperties
        {
        }

        private sealed class Scenario
        {
            public string Name { get; set; } = string.Empty;
            public Func<string> Build { get; set; } = null!;
            public string Sheet { get; set; } = string.Empty;
            public bool Edit { get; set; }
        }

        private sealed class Sample
        {
            public double[] Phases { get; set; } = new double[3];
            public double TotalMs { get; set; }
            public double AllocMb { get; set; }
            public int Gen0 { get; set; }
            public int Gen1 { get; set; }
            public int Gen2 { get; set; }
            public double GcPauseMs { get; set; }
            public double CpuMs { get; set; }
            public double PeakWorkingSetMb { get; set; }
            public double WorkingSetDeltaMb { get; set; }
            public double OutKb { get; set; }
        }

        private sealed class Stat
        {
            public string Library { get; set; } = string.Empty;
            public string Scenario { get; set; } = string.Empty;
            public double[] PhaseMs { get; set; } = new double[3];
            public double TotalMs { get; set; }
            public double MinMs { get; set; }
            public double MaxMs { get; set; }
            public Sample Median { get; set; } = null!;
            public double InKb { get; set; }
        }

        [ClassInitialize]
        public static void ClassInitialize(TestContext context)
        {
            Directory.CreateDirectory(resultPath);
            if (Directory.Exists(tempPath))
            {
                Directory.Delete(tempPath, true);
            }
            Directory.CreateDirectory(tempPath);
        }

        [ClassCleanup]
        public static void ClassCleanup()
        {
            if (Directory.Exists(tempPath))
            {
                Directory.Delete(tempPath, true);
            }
        }

        [TestMethod]
        public void CompareOpenEditSavePerformance()
        {
            int iterations = int.TryParse(Environment.GetEnvironmentVariable("BENCH_ITERATIONS"), out int parsed) && parsed > 0 ? parsed : 5;
            bool record = Environment.GetEnvironmentVariable("BENCH_RECORD") == "1";
            string version = PackageVersion();
            string commit = GitCommit();
            string timestamp = DateTime.UtcNow.ToString("o", CultureInfo.InvariantCulture);
            Dictionary<string, (string Version, double TimeMs, double AllocMb)> previous = PreviousRelease(version);

            List<Scenario> scenarios = new()
            {
                new Scenario { Name = "open_edit_save_100k_cells_1_sheet", Build = BuildLargeSingleSheet, Sheet = "Benchmark", Edit = true },
                new Scenario { Name = "open_edit_save_10k_cells_50_sheets", Build = BuildManySheets, Sheet = "Sheet_0", Edit = true },
                new Scenario { Name = "open_save_untouched_50_sheets", Build = BuildManySheets, Sheet = "Sheet_0", Edit = false },
            };
            List<(string Name, Runner Run)> libraries = new()
            {
                (CurrentLibrary, RunCurrent),
                (LegacyLibrary, RunLegacy),
            };

            // Warm each library once.
            string warmSource = scenarios[1].Build();
            foreach ((string name, Runner run) in libraries)
            {
                SampleRun(warmSource, "Sheet_0", true, run);
            }

            StringBuilder markdown = new();
            markdown.AppendLine("# Spreadsheet Open/Edit/Save Benchmark (C#)");
            markdown.AppendLine();
            markdown.AppendLine($"- Version: {version} ({commit})");
            markdown.AppendLine($"- Machine: {Environment.MachineName} ({Environment.ProcessorCount} logical CPUs), {Environment.Version}");
            markdown.AppendLine($"- Iterations: {iterations} (median reported)");
            markdown.AppendLine();
            markdown.AppendLine("| Scenario | Library | Total (ms) | Min | Max | Open | Edit | Save | Open MB/s | Alloc managed (MB) | GC 0/1/2 | GC pause (ms) | CPU (ms) | Peak WS (MB) | WS Δ (MB) | In (KB) | Out (KB) |");
            markdown.AppendLine("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :---: | ---: | ---: | ---: | ---: | ---: | ---: |");

            StringBuilder progress = new();
            progress.AppendLine();
            progress.AppendLine("## Change against previous release");
            progress.AppendLine();
            progress.AppendLine("| Scenario | Library | Phase | Time (ms) | vs previous | Alloc (MB) | vs previous | Previous version |");
            progress.AppendLine("| --- | --- | --- | ---: | ---: | ---: | ---: | --- |");
            StringBuilder versus = new();
            versus.AppendLine();
            versus.AppendLine($"## Head-to-head: {CurrentLibrary} vs {LegacyLibrary}");
            versus.AppendLine();
            versus.AppendLine("Multiplier of the legacy package over v4.x. Higher is better for v4.x; `1.00×` is equal.");
            versus.AppendLine();
            versus.AppendLine("| Scenario | Total | Open | Save | Peak WS |");
            versus.AppendLine("| --- | ---: | ---: | ---: | ---: |");
            StringBuilder rows = new();

            foreach (Scenario scenario in scenarios)
            {
                string source = scenario.Build();
                double inKb = new FileInfo(source).Length / 1024.0;
                Dictionary<string, Stat> stats = new();
                foreach ((string name, Runner run) in libraries)
                {
                    List<Sample> samples = new();
                    for (int iteration = 0; iteration < iterations; iteration++)
                    {
                        samples.Add(SampleRun(source, scenario.Sheet, scenario.Edit, run));
                    }
                    Stat stat = Summarize(name, scenario.Name, samples, inKb);
                    stats[name] = stat;
                    Sample medianSample = stat.Median;
                    double openMbPerSec = stat.PhaseMs[0] > 0 ? inKb / 1024.0 / (stat.PhaseMs[0] / 1000.0) : 0;
                    markdown.AppendLine(string.Create(CultureInfo.InvariantCulture,
                        $"| {scenario.Name} | {name} | {stat.TotalMs:0.00} | {stat.MinMs:0.00} | {stat.MaxMs:0.00} | {stat.PhaseMs[0]:0.00} | {stat.PhaseMs[1]:0.00} | {stat.PhaseMs[2]:0.00} | {openMbPerSec:0.0} | {medianSample.AllocMb:0.00} | {medianSample.Gen0}/{medianSample.Gen1}/{medianSample.Gen2} | {medianSample.GcPauseMs:0.00} | {medianSample.CpuMs:0.0} | {medianSample.PeakWorkingSetMb:0.0} | {medianSample.WorkingSetDeltaMb:0.0} | {inKb:0.0} | {medianSample.OutKb:0.0} |"));

                    List<(string Phase, double Time)> phaseValues = new()
                    {
                        ("open", stat.PhaseMs[0]), ("edit", stat.PhaseMs[1]), ("save", stat.PhaseMs[2]), ("total", stat.TotalMs),
                    };
                    foreach ((string phase, double time) in phaseValues)
                    {
                        if (phase == "edit" && !scenario.Edit)
                        {
                            continue;
                        }
                        double minimum = phase == "total" ? stat.MinMs : time;
                        double maximum = phase == "total" ? stat.MaxMs : time;
                        double allocationMb = phase == "total" ? medianSample.AllocMb : 0;
                        rows.AppendLine(string.Create(CultureInfo.InvariantCulture,
                            $"{timestamp},{version},{commit},{name},{scenario.Name},{phase},{time:0.000},{minimum:0.000},{maximum:0.000},{allocationMb:0.000},{medianSample.Gen0 + medianSample.Gen1 + medianSample.Gen2},{medianSample.PeakWorkingSetMb:0.00},{medianSample.OutKb:0.00}"));
                        if (name == CurrentLibrary && previous.TryGetValue($"{name}|{scenario.Name}|{phase}", out var previousSample))
                        {
                            progress.AppendLine(string.Create(CultureInfo.InvariantCulture,
                                $"| {scenario.Name} | {name} | {phase} | {time:0.00} | {ChangePercent(time, previousSample.TimeMs)} | {allocationMb:0.00} | {ChangePercent(allocationMb, previousSample.AllocMb)} | {previousSample.Version} |"));
                        }
                    }
                }
                Stat current = stats[CurrentLibrary];
                Stat legacy = stats[LegacyLibrary];
                versus.AppendLine($"| {scenario.Name} | {Times(legacy.TotalMs, current.TotalMs)} | {Times(legacy.PhaseMs[0], current.PhaseMs[0])} | {Times(legacy.PhaseMs[2], current.PhaseMs[2])} | {Times(legacy.Median.PeakWorkingSetMb, current.Median.PeakWorkingSetMb)} |");
            }

            markdown.Append(progress);
            markdown.Append(versus);

            string stamp = DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss");
            File.WriteAllText(Path.Combine(resultPath, $"benchmark-edit-report-{stamp}.md"), markdown.ToString());
            File.WriteAllText(Path.Combine(resultPath, $"benchmark-edit-report-{stamp}.csv"), HistoryHeader + Environment.NewLine + rows);
            if (record)
            {
                Directory.CreateDirectory(Path.GetDirectoryName(historyPath)!);
                bool isNew = !File.Exists(historyPath);
                File.AppendAllText(historyPath, (isNew ? HistoryHeader + Environment.NewLine : string.Empty) + rows);
                TestContext.WriteLine($"History appended to {historyPath}");
            }
            TestContext.WriteLine(markdown.ToString());
            Console.WriteLine(markdown.ToString());
        }

        private Sample SampleRun(string source, string sheet, bool edit, Runner run)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
            Process process = Process.GetCurrentProcess();
            process.Refresh();
            long allocBefore = GC.GetTotalAllocatedBytes(true);
            int gen0 = GC.CollectionCount(0), gen1 = GC.CollectionCount(1), gen2 = GC.CollectionCount(2);
            TimeSpan pauseBefore = GC.GetTotalPauseDuration();
            long workingSetBefore = process.WorkingSet64;
            long peakWorkingSet = workingSetBefore;
            TimeSpan cpuBefore = process.TotalProcessorTime;
            bool monitoring = true;
            Thread sampler = new(() =>
            {
                while (Volatile.Read(ref monitoring))
                {
                    Thread.Sleep(10);
                    process.Refresh();
                    long workingSet = process.WorkingSet64;
                    if (workingSet > peakWorkingSet)
                    {
                        peakWorkingSet = workingSet;
                    }
                }
            })
            { IsBackground = true, Name = "benchmark-sampler" };
            sampler.Start();

            double[] phases = new double[3];
            Stopwatch stopwatch = Stopwatch.StartNew();
            string output = run(source, sheet, edit, phases);
            stopwatch.Stop();
            Volatile.Write(ref monitoring, false);
            sampler.Join();

            process.Refresh();
            const double toMb = 1024.0 * 1024.0;
            return new Sample
            {
                Phases = phases,
                TotalMs = stopwatch.Elapsed.TotalMilliseconds,
                AllocMb = (GC.GetTotalAllocatedBytes(true) - allocBefore) / toMb,
                Gen0 = GC.CollectionCount(0) - gen0,
                Gen1 = GC.CollectionCount(1) - gen1,
                Gen2 = GC.CollectionCount(2) - gen2,
                GcPauseMs = (GC.GetTotalPauseDuration() - pauseBefore).TotalMilliseconds,
                CpuMs = (process.TotalProcessorTime - cpuBefore).TotalMilliseconds,
                PeakWorkingSetMb = peakWorkingSet / toMb,
                WorkingSetDeltaMb = (Math.Max(peakWorkingSet, process.WorkingSet64) - workingSetBefore) / toMb,
                OutKb = File.Exists(output) ? new FileInfo(output).Length / 1024.0 : 0,
            };
        }

        private static Stat Summarize(string library, string scenario, List<Sample> samples, double inKb)
        {
            List<Sample> sorted = samples.OrderBy(sample => sample.TotalMs).ToList();
            Stat stat = new()
            {
                Library = library,
                Scenario = scenario,
                TotalMs = sorted[sorted.Count / 2].TotalMs,
                MinMs = sorted[0].TotalMs,
                MaxMs = sorted[^1].TotalMs,
                Median = sorted[sorted.Count / 2],
                InKb = inKb,
            };
            for (int phase = 0; phase < 3; phase++)
            {
                List<double> values = samples.Select(sample => sample.Phases[phase]).OrderBy(value => value).ToList();
                stat.PhaseMs[phase] = values[values.Count / 2];
            }
            return stat;
        }

        private string RunCurrent(string source, string sheet, bool edit, double[] phases)
        {
            Stopwatch stopwatch = Stopwatch.StartNew();
            CurrentSpreadsheet.Excel excel = new(source);
            phases[0] = stopwatch.Elapsed.TotalMilliseconds;
            if (edit)
            {
                stopwatch.Restart();
                CurrentSpreadsheet.Worksheet worksheet = excel.GetWorksheet(sheet);
                worksheet.SetCellRefValues("A1", new[]
                {
                    new CurrentSpreadsheet.CellProperty { Value = "edited", DataType = CurrentSpreadsheet.CellDataType.Number },
                });
                worksheet.Dispose();
                phases[1] = stopwatch.Elapsed.TotalMilliseconds;
            }
            stopwatch.Restart();
            string output = Path.Combine(tempPath, "edit_out_v4x.xlsx");
            excel.SaveAs(output);
            phases[2] = stopwatch.Elapsed.TotalMilliseconds;
            return output;
        }

        private string RunLegacy(string source, string sheet, bool edit, double[] phases)
        {
            Stopwatch stopwatch = Stopwatch.StartNew();
            LegacySpreadsheet.Excel excel = new(source, true, new LegacySpreadsheet.ExcelProperties(), new LegacyPrivacy());
            phases[0] = stopwatch.Elapsed.TotalMilliseconds;
            if (edit)
            {
                stopwatch.Restart();
                LegacySpreadsheet.Worksheet worksheet = excel.GetWorksheet(sheet);
                worksheet.SetRow("A1", new[]
                {
                    new LegacySpreadsheet.ColumnCell { cellValue = "edited", dataType = LegacySpreadsheet.CellDataType.NUMBER },
                });
                phases[1] = stopwatch.Elapsed.TotalMilliseconds;
            }
            stopwatch.Restart();
            string output = Path.Combine(tempPath, "edit_out_legacy.xlsx");
            excel.SaveAs(output);
            phases[2] = stopwatch.Elapsed.TotalMilliseconds;
            return output;
        }

        private static string BuildLargeSingleSheet()
        {
            CurrentSpreadsheet.Excel excel = new(new CurrentSpreadsheet.ExcelProperties { isEditable = true });
            CurrentSpreadsheet.Worksheet worksheet = excel.AddSheet("Benchmark");
            FillSheet(worksheet, 100000);
            worksheet.Dispose();
            string path = Path.Combine(tempPath, "edit_src_large.xlsx");
            excel.SaveAs(path);
            return path;
        }

        private static string BuildManySheets()
        {
            CurrentSpreadsheet.Excel excel = new(new CurrentSpreadsheet.ExcelProperties { isEditable = true });
            for (int sheetIndex = 0; sheetIndex < 50; sheetIndex++)
            {
                CurrentSpreadsheet.Worksheet worksheet = excel.AddSheet($"Sheet_{sheetIndex}");
                FillSheet(worksheet, 10000);
                worksheet.Dispose();
            }
            string path = Path.Combine(tempPath, "edit_src_many_sheets.xlsx");
            excel.SaveAs(path);
            return path;
        }

        private static void FillSheet(CurrentSpreadsheet.Worksheet worksheet, int cellCount)
        {
            const int columnsPerRow = 100;
            int written = 0;
            int row = 1;
            while (written < cellCount)
            {
                int rowCount = Math.Min(columnsPerRow, cellCount - written);
                CurrentSpreadsheet.CellProperty[] cells = new CurrentSpreadsheet.CellProperty[rowCount];
                for (int column = 0; column < rowCount; column++)
                {
                    cells[column] = new CurrentSpreadsheet.CellProperty
                    {
                        Value = (written + column).ToString(),
                        DataType = CurrentSpreadsheet.CellDataType.Number,
                    };
                }
                worksheet.SetCellRefValues($"A{row}", cells);
                written += rowCount;
                row++;
            }
        }

        private static string PackageVersion()
        {
            if (!File.Exists(versionFile))
            {
                return "unknown";
            }
            foreach (string line in File.ReadLines(versionFile))
            {
                if (line.StartsWith("version", StringComparison.Ordinal) && line.Contains('"'))
                {
                    return line.Split('"')[1];
                }
            }
            return "unknown";
        }

        private static string GitCommit()
        {
            try
            {
                using Process process = Process.Start(new ProcessStartInfo("git", "rev-parse --short HEAD")
                {
                    RedirectStandardOutput = true,
                    UseShellExecute = false,
                })!;
                string commit = process.StandardOutput.ReadToEnd().Trim();
                process.WaitForExit();
                return commit.Length > 0 ? commit : "unknown";
            }
            catch (Exception)
            {
                return "unknown";
            }
        }

        /// <summary>Latest row per library|scenario|phase from a version other than the current one.</summary>
        private static Dictionary<string, (string Version, double TimeMs, double AllocMb)> PreviousRelease(string version)
        {
            Dictionary<string, (string, double, double)> previous = new();
            if (!File.Exists(historyPath))
            {
                return previous;
            }
            foreach (string line in File.ReadLines(historyPath).Skip(1))
            {
                string[] fields = line.Split(',');
                if (fields.Length < 13 || fields[1] == version)
                {
                    continue;
                }
                if (double.TryParse(fields[6], NumberStyles.Float, CultureInfo.InvariantCulture, out double time)
                    && double.TryParse(fields[9], NumberStyles.Float, CultureInfo.InvariantCulture, out double alloc))
                {
                    previous[$"{fields[3]}|{fields[4]}|{fields[5]}"] = (fields[1], time, alloc);
                }
            }
            return previous;
        }

        private static string ChangePercent(double current, double before)
        {
            return before <= 0 ? "n/a" : string.Create(CultureInfo.InvariantCulture, $"{(current - before) / before * 100.0:+0.0;-0.0}%");
        }

        private static string Times(double numerator, double denominator)
        {
            return denominator <= 0 ? "n/a" : string.Create(CultureInfo.InvariantCulture, $"{numerator / denominator:0.00}×");
        }
    }
}
