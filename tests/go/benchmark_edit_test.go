//go:build linux || darwin

// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

package openxml_office_go_test

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/xuri/excelize/v2"

	"github.com/DraviaVemal/openxml-office/go/spreadsheet/spreadsheet_2007"
)

// Open -> edit -> save benchmark (the OfficeDocument archive path) against excelize v2.
// Each scenario runs BENCH_ITERATIONS times (default 5) and the median is reported with min/max.
// With BENCH_RECORD=1 the run is appended to tests/benchmark_history/go.csv keyed by the package
// version so the report shows the change against the previous release.
// Alloc is the Go heap only; native (Rust) memory is visible in the RSS columns.

const (
	historyHeader = "timestamp,version,commit,library,scenario,phase,time_ms,min_ms,max_ms,alloc_mb,alloc_count,peak_rss_mb,out_kb"
	libraryV4     = "v4.x"
	libraryExcel  = "excelize v2"
)

type phaseTimes [3]float64

type editScenario struct {
	name  string
	build func(dir string) (string, error)
	sheet string
	edit  bool
}

type editSample struct {
	phases    phaseTimes
	totalMs   float64
	allocMb   float64
	mallocs   uint64
	numGC     uint32
	gcPauseMs float64
	cpuMs     float64
	peakRssMb float64
	rssDelta  float64
	outKb     float64
}

type editStat struct {
	library   string
	scenario  string
	phaseMs   phaseTimes
	totalMs   float64
	minMs     float64
	maxMs     float64
	allocMb   float64
	mallocs   uint64
	numGC     uint32
	gcPauseMs float64
	cpuMs     float64
	peakRssMb float64
	rssDelta  float64
	inKb      float64
	outKb     float64
	edit      bool
}

type editRunner func(dir, source, sheet string, edit bool, phases *phaseTimes) (string, error)

func buildLargeSingleSheet(dir string) (string, error) {
	return writeCurrentPackage(dir, 100000, "edit_src_large")
}

func buildManySheets(dir string) (string, error) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		return "", err
	}
	for sheetIndex := 0; sheetIndex < 50; sheetIndex++ {
		sheet, err := excel.AddSheet(fmt.Sprintf("Sheet_%d", sheetIndex))
		if err != nil {
			return "", err
		}
		cells := make([]spreadsheet_2007.CellProperty, 100)
		for cellIndex := range cells {
			cells[cellIndex] = spreadsheet_2007.CellProperty{Value: strconv.Itoa(cellIndex), DataType: spreadsheet_2007.CellDataTypeNumber}
		}
		for row := 1; row <= 100; row++ {
			if err := sheet.SetCellRefValues(fmt.Sprintf("A%d", row), cells); err != nil {
				return "", err
			}
		}
		if err := sheet.Flush(); err != nil {
			return "", err
		}
	}
	path := filepath.Join(dir, "edit_src_many_sheets.xlsx")
	if err := excel.SaveAs(path); err != nil {
		return "", err
	}
	return path, nil
}

func ms(duration time.Duration) float64 { return float64(duration.Nanoseconds()) / 1e6 }

func runV4(dir, source, sheetName string, edit bool, phases *phaseTimes) (string, error) {
	openStarted := time.Now()
	excel, err := spreadsheet_2007.NewExcel(source)
	if err != nil {
		return "", err
	}
	phases[0] = ms(time.Since(openStarted))

	if edit {
		editStarted := time.Now()
		sheet, err := excel.GetWorksheet(sheetName)
		if err != nil {
			return "", err
		}
		if err := sheet.SetCellRefValues("A1", []spreadsheet_2007.CellProperty{
			{Value: "edited", DataType: spreadsheet_2007.CellDataTypeNumber},
		}); err != nil {
			return "", err
		}
		if err := sheet.Flush(); err != nil {
			return "", err
		}
		phases[1] = ms(time.Since(editStarted))
	}

	saveStarted := time.Now()
	out := filepath.Join(dir, "edit_out_v4x.xlsx")
	if err := excel.SaveAs(out); err != nil {
		return "", err
	}
	phases[2] = ms(time.Since(saveStarted))
	return out, nil
}

func runExcelize(dir, source, sheetName string, edit bool, phases *phaseTimes) (string, error) {
	openStarted := time.Now()
	workbook, err := excelize.OpenFile(source)
	if err != nil {
		return "", err
	}
	defer workbook.Close()
	phases[0] = ms(time.Since(openStarted))

	if edit {
		editStarted := time.Now()
		if err := workbook.SetCellValue(sheetName, "A1", "edited"); err != nil {
			return "", err
		}
		phases[1] = ms(time.Since(editStarted))
	}

	saveStarted := time.Now()
	out := filepath.Join(dir, "edit_out_excelize.xlsx")
	if err := workbook.SaveAs(out); err != nil {
		return "", err
	}
	phases[2] = ms(time.Since(saveStarted))
	return out, nil
}

// peakRssBytes returns the Linux resident-set high-water mark.
func peakRssBytes() uint64 {
	data, err := os.ReadFile("/proc/self/status")
	if err != nil {
		return processRssBytes()
	}
	for _, line := range strings.Split(string(data), "\n") {
		if strings.HasPrefix(line, "VmHWM:") {
			fields := strings.Fields(line)
			if len(fields) >= 2 {
				kb, _ := strconv.ParseUint(fields[1], 10, 64)
				return kb * 1024
			}
		}
	}
	return processRssBytes()
}

// resetPeakRss resets the Linux high-water mark when permitted.
func resetPeakRss() {
	_ = os.WriteFile("/proc/self/clear_refs", []byte("5"), 0o200)
}

func sampleRun(dir, source, sheet string, edit bool, run editRunner) (editSample, error) {
	runtime.GC()
	resetPeakRss()
	var before, after runtime.MemStats
	runtime.ReadMemStats(&before)
	cpuBefore := processCPUTime()
	rssBefore := processRssBytes()

	var phases phaseTimes
	start := time.Now()
	out, err := run(dir, source, sheet, edit, &phases)
	elapsed := time.Since(start)
	if err != nil {
		return editSample{}, err
	}
	runtime.ReadMemStats(&after)
	cpuAfter := processCPUTime()
	peak := peakRssBytes()

	const mb = 1024.0 * 1024.0
	var outKb float64
	if info, statErr := os.Stat(out); statErr == nil {
		outKb = float64(info.Size()) / 1024.0
	}
	return editSample{
		phases:    phases,
		totalMs:   ms(elapsed),
		allocMb:   float64(after.TotalAlloc-before.TotalAlloc) / mb,
		mallocs:   after.Mallocs - before.Mallocs,
		numGC:     after.NumGC - before.NumGC,
		gcPauseMs: float64(after.PauseTotalNs-before.PauseTotalNs) / 1e6,
		cpuMs:     float64(cpuAfter-cpuBefore) / 1e6,
		peakRssMb: float64(peak) / mb,
		rssDelta:  (float64(peak) - float64(rssBefore)) / mb,
		outKb:     outKb,
	}, nil
}

func median(values []float64) float64 {
	sorted := append([]float64(nil), values...)
	sort.Float64s(sorted)
	return sorted[len(sorted)/2]
}

func summarize(library, scenario string, samples []editSample, inKb float64, edit bool) editStat {
	sorted := append([]editSample(nil), samples...)
	sort.Slice(sorted, func(leftIndex, rightIndex int) bool {
		return sorted[leftIndex].totalMs < sorted[rightIndex].totalMs
	})
	medianSample := sorted[len(sorted)/2]
	stat := editStat{
		library: library, scenario: scenario, edit: edit,
		totalMs: medianSample.totalMs, minMs: sorted[0].totalMs, maxMs: sorted[len(sorted)-1].totalMs,
		allocMb: medianSample.allocMb, mallocs: medianSample.mallocs, numGC: medianSample.numGC, gcPauseMs: medianSample.gcPauseMs,
		cpuMs: medianSample.cpuMs, peakRssMb: medianSample.peakRssMb, rssDelta: medianSample.rssDelta, inKb: inKb, outKb: medianSample.outKb,
	}
	for phase := 0; phase < 3; phase++ {
		values := make([]float64, len(samples))
		for sampleIndex, sample := range samples {
			values[sampleIndex] = sample.phases[phase]
		}
		stat.phaseMs[phase] = median(values)
	}
	return stat
}

func packageVersion() string {
	data, err := os.ReadFile(filepath.Join("..", "..", "rs", "Cargo.toml"))
	if err != nil {
		return "unknown"
	}
	match := regexp.MustCompile(`(?m)^version\s*=\s*"([^"]+)"`).FindSubmatch(data)
	if match == nil {
		return "unknown"
	}
	return string(match[1])
}

func gitCommit() string {
	out, err := exec.Command("git", "rev-parse", "--short", "HEAD").Output()
	if err != nil {
		return "unknown"
	}
	return strings.TrimSpace(string(out))
}

type historyRow struct {
	version string
	timeMs  float64
	allocMb float64
}

func historyFile() string { return filepath.Join("..", "benchmark_history", "go.csv") }

// previousRelease returns the latest rows from earlier versions.
func previousRelease(version string) map[string]historyRow {
	previous := map[string]historyRow{}
	data, err := os.ReadFile(historyFile())
	if err != nil {
		return previous
	}
	for lineIndex, line := range strings.Split(string(data), "\n") {
		fields := strings.Split(strings.TrimSpace(line), ",")
		if lineIndex == 0 || len(fields) < 13 || fields[1] == version {
			continue
		}
		timeMs, errTime := strconv.ParseFloat(fields[6], 64)
		allocMb, errAlloc := strconv.ParseFloat(fields[9], 64)
		if errTime != nil || errAlloc != nil {
			continue
		}
		previous[fields[3]+"|"+fields[4]+"|"+fields[5]] = historyRow{fields[1], timeMs, allocMb}
	}
	return previous
}

func changePercent(current, before float64) string {
	if before <= 0 {
		return "n/a"
	}
	return fmt.Sprintf("%+.1f%%", (current-before)/before*100.0)
}

func appendHistory(rows string) error {
	path := historyFile()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	_, statErr := os.Stat(path)
	file, err := os.OpenFile(path, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	defer file.Close()
	if statErr != nil {
		if _, err := file.WriteString(historyHeader + "\n"); err != nil {
			return err
		}
	}
	_, err = file.WriteString(rows)
	return err
}

func TestOpenEditSavePerformance(t *testing.T) {
	iterations := 5
	if value, err := strconv.Atoi(os.Getenv("BENCH_ITERATIONS")); err == nil && value > 0 {
		iterations = value
	}
	record := os.Getenv("BENCH_RECORD") == "1"

	tempPath := filepath.Join(resultPath, benchTempDirName+"_edit")
	if err := os.RemoveAll(tempPath); err != nil {
		t.Fatalf("clean temp: %v", err)
	}
	if err := os.MkdirAll(tempPath, 0o755); err != nil {
		t.Fatalf("create temp: %v", err)
	}
	defer os.RemoveAll(tempPath)

	scenarios := []editScenario{
		{"open_edit_save_100k_cells_1_sheet", buildLargeSingleSheet, "Benchmark", true},
		{"open_edit_save_10k_cells_50_sheets", buildManySheets, "Sheet_0", true},
		{"open_save_untouched_50_sheets", buildManySheets, "Sheet_0", false},
	}
	libraries := []struct {
		name string
		run  editRunner
	}{{libraryV4, runV4}, {libraryExcel, runExcelize}}

	version := packageVersion()
	commit := gitCommit()
	previous := previousRelease(version)
	timestamp := time.Now().UTC().Format(time.RFC3339)

	var md strings.Builder
	fmt.Fprintf(&md, "# Spreadsheet Open/Edit/Save Benchmark (Go)\n\n- Version: %s (%s)\n", version, commit)
	hostname, _ := os.Hostname()
	fmt.Fprintf(&md, "- Machine: %s (%d logical CPUs), %s on %s/%s\n", hostname, runtime.NumCPU(), runtime.Version(), runtime.GOOS, runtime.GOARCH)
	fmt.Fprintf(&md, "- Iterations: %d (median reported)\n\n", iterations)
	md.WriteString("| Scenario | Library | Total (ms) | Min | Max | Open | Edit | Save | Open MB/s | Alloc Go (MB) | Allocs | GC | GC pause (ms) | CPU (ms) | Peak RSS (MB) | RSS Δ (MB) | In (KB) | Out (KB) |\n")
	md.WriteString("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")

	var progress strings.Builder
	progress.WriteString("\n## Change against previous release\n\n| Scenario | Library | Phase | Time (ms) | vs previous | Alloc (MB) | vs previous | Previous version |\n| --- | --- | --- | ---: | ---: | ---: | ---: | --- |\n")
	var versus strings.Builder
	versus.WriteString("\n## Head-to-head: v4.x vs excelize v2\n\nMultiplier of excelize over v4.x. Higher is better for v4.x; `1.00×` is equal.\n\n| Scenario | Total | Open | Save | Peak RSS |\n| --- | ---: | ---: | ---: | ---: |\n")
	var rows strings.Builder
	// Warm each library once.
	warmSource, err := scenarios[1].build(tempPath)
	if err != nil {
		t.Fatalf("warmup build: %v", err)
	}
	for _, library := range libraries {
		if _, err := sampleRun(tempPath, warmSource, "Sheet_0", true, library.run); err != nil {
			t.Fatalf("%s warmup: %v", library.name, err)
		}
	}

	for _, scenario := range scenarios {
		source, err := scenario.build(tempPath)
		if err != nil {
			t.Fatalf("%s build: %v", scenario.name, err)
		}
		inInfo, err := os.Stat(source)
		if err != nil {
			t.Fatalf("%s stat: %v", scenario.name, err)
		}
		inKb := float64(inInfo.Size()) / 1024.0

		stats := map[string]editStat{}
		for _, library := range libraries {
			samples := make([]editSample, 0, iterations)
			for iteration := 0; iteration < iterations; iteration++ {
				sample, err := sampleRun(tempPath, source, scenario.sheet, scenario.edit, library.run)
				if err != nil {
					t.Fatalf("%s %s: %v", scenario.name, library.name, err)
				}
				samples = append(samples, sample)
			}
			stat := summarize(library.name, scenario.name, samples, inKb, scenario.edit)
			stats[library.name] = stat

			openMbPerSec := 0.0
			if stat.phaseMs[0] > 0 {
				openMbPerSec = inKb / 1024.0 / (stat.phaseMs[0] / 1000.0)
			}
			fmt.Fprintf(&md, "| %s | %s | %.2f | %.2f | %.2f | %.2f | %.2f | %.2f | %.1f | %.2f | %d | %d | %.2f | %.1f | %.1f | %.1f | %.1f | %.1f |\n",
				scenario.name, library.name, stat.totalMs, stat.minMs, stat.maxMs,
				stat.phaseMs[0], stat.phaseMs[1], stat.phaseMs[2], openMbPerSec,
				stat.allocMb, stat.mallocs, stat.numGC, stat.gcPauseMs, stat.cpuMs,
				stat.peakRssMb, stat.rssDelta, stat.inKb, stat.outKb)

			phaseValues := []struct {
				name string
				time float64
			}{{"open", stat.phaseMs[0]}, {"edit", stat.phaseMs[1]}, {"save", stat.phaseMs[2]}, {"total", stat.totalMs}}
			for _, phase := range phaseValues {
				if phase.name == "edit" && !scenario.edit {
					continue
				}
				minMs, maxMs, alloc := phase.time, phase.time, 0.0
				if phase.name == "total" {
					minMs, maxMs, alloc = stat.minMs, stat.maxMs, stat.allocMb
				}
				fmt.Fprintf(&rows, "%s,%s,%s,%s,%s,%s,%.3f,%.3f,%.3f,%.3f,%d,%.2f,%.2f\n",
					timestamp, version, commit, library.name, scenario.name, phase.name,
					phase.time, minMs, maxMs, alloc, stat.mallocs, stat.peakRssMb, stat.outKb)
				if library.name == libraryV4 {
					if old, ok := previous[library.name+"|"+scenario.name+"|"+phase.name]; ok {
						fmt.Fprintf(&progress, "| %s | %s | %s | %.2f | %s | %.2f | %s | %s |\n",
							scenario.name, library.name, phase.name, phase.time, changePercent(phase.time, old.timeMs),
							alloc, changePercent(alloc, old.allocMb), old.version)
					}
				}
			}
		}
		currentStat, comparisonStat := stats[libraryV4], stats[libraryExcel]
		fmt.Fprintf(&versus, "| %s | %s | %s | %s | %s |\n", scenario.name,
			times(comparisonStat.totalMs, currentStat.totalMs), times(comparisonStat.phaseMs[0], currentStat.phaseMs[0]),
			times(comparisonStat.phaseMs[2], currentStat.phaseMs[2]), times(comparisonStat.peakRssMb, currentStat.peakRssMb))
	}

	md.WriteString(progress.String())
	md.WriteString(versus.String())

	stamp := time.Now().Format("2006-01-02-15-04-05")
	mdPath := filepath.Join(resultPath, "benchmark-edit-report-"+stamp+".md")
	csvPath := filepath.Join(resultPath, "benchmark-edit-report-"+stamp+".csv")
	if err := os.WriteFile(mdPath, []byte(md.String()), 0o644); err != nil {
		t.Fatalf("write report: %v", err)
	}
	csvContent := historyHeader + "\n" + rows.String()
	if err := os.WriteFile(csvPath, []byte(csvContent), 0o644); err != nil {
		t.Fatalf("write csv: %v", err)
	}
	if record {
		if err := appendHistory(rows.String()); err != nil {
			t.Fatalf("append history: %v", err)
		}
		t.Logf("History appended to %s", historyFile())
	}
	t.Log("\n" + md.String())
}
