//go:build linux || darwin

// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

package openxml_office_go_test

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"syscall"
	"testing"
	"time"

	"github.com/xuri/excelize/v2"

	"draviavemal_openxml_office/spreadsheet/spreadsheet_2007"
)

// Standalone benchmark that compares the current package against the published
// github.com/xuri/excelize v2 package. It mirrors the C# benchmark (test/cs/benchmark.cs):
// it measures execution time, managed allocations, retained/peak heap, CPU and GC
// pressure for a range of cell counts and writes a Markdown + CSV report.
// It is fully self contained and does not touch the existing functional tests.

const (
	benchTempDirName = "benchmark_temp"
	columnsPerRow    = 100
)

var benchCellCounts = []int{10, 100, 1000, 10000, 100000}

// benchmarkResult is a single benchmark measurement row.
type benchmarkResult struct {
	Library                string
	CellCount              int
	ElapsedMs              float64
	CellsPerSecond         float64
	MicrosPerCell          float64
	AllocatedMb            float64 // total heap bytes allocated during the run (churn)
	AllocatedBytesPerCell  float64
	AllocationRateMbPerSec float64
	HeapRetainedMb         float64 // heap retained afterwards (HeapAlloc delta)
	PeakHeapMb             float64 // peak HeapInuse observed while sampling
	PeakSysMb              float64 // peak Sys (reserved from OS) observed while sampling
	AvgCpuPercent          float64
	PeakCpuPercent         float64
	NumGC                  uint32
	GcPauseMs              float64
	GcPausePercent         float64
	HeapSysMb              float64
	OutputFileKb           float64
}

// TestCompareWritePerformance runs the write benchmark for both libraries across all
// configured cell counts and writes a Markdown + CSV report to the test_results folder.
func TestCompareWritePerformance(t *testing.T) {
	tempPath := filepath.Join(resultPath, benchTempDirName)
	if err := os.RemoveAll(tempPath); err != nil {
		t.Fatalf("clean temp: %v", err)
	}
	if err := os.MkdirAll(tempPath, 0o755); err != nil {
		t.Fatalf("create temp: %v", err)
	}
	defer os.RemoveAll(tempPath)

	// Warm up so the first measured run is not penalised by one time startup costs.
	if _, err := writeCurrentPackage(tempPath, 10, "warmup"); err != nil {
		t.Fatalf("warmup current: %v", err)
	}
	if _, err := writeExcelize(tempPath, 10, "warmup"); err != nil {
		t.Fatalf("warmup excelize: %v", err)
	}

	var results []benchmarkResult
	for _, count := range benchCellCounts {
		curCount := count
		res, err := measure(t, "v4.x (module reference)", curCount, func(tag string) (string, error) {
			return writeCurrentPackage(tempPath, curCount, tag)
		})
		if err != nil {
			t.Fatalf("current package (%d cells): %v", curCount, err)
		}
		results = append(results, res)

		res, err = measure(t, "excelize v2", curCount, func(tag string) (string, error) {
			return writeExcelize(tempPath, curCount, tag)
		})
		if err != nil {
			t.Fatalf("excelize (%d cells): %v", curCount, err)
		}
		results = append(results, res)
	}

	if err := writeReport(t, results); err != nil {
		t.Fatalf("write report: %v", err)
	}
	if len(results) != len(benchCellCounts)*2 {
		t.Fatalf("expected %d results, got %d", len(benchCellCounts)*2, len(results))
	}
}

// measure captures wall clock time, throughput, per-cell latency, heap allocations
// (churn), retained/peak heap, average and peak CPU, GC collection counts and pause
// time and the produced workbook size while the supplied workload runs. RAM/CPU peaks
// are captured by a background sampler. The workload returns the path of the produced
// file so its size can be recorded.
func measure(t *testing.T, library string, cellCount int, workload func(tag string) (string, error)) (benchmarkResult, error) {
	t.Helper()

	// Establish a clean baseline.
	runtime.GC()

	var msBefore runtime.MemStats
	runtime.ReadMemStats(&msBefore)
	cpuBefore := processCPUTime()

	peakHeap := msBefore.HeapInuse
	peakSys := msBefore.Sys
	var peakCPUPercent float64
	stop := make(chan struct{})
	done := make(chan struct{})

	start := time.Now()
	lastWall := start
	lastCPU := cpuBefore

	go func() {
		ticker := time.NewTicker(15 * time.Millisecond)
		defer ticker.Stop()
		defer close(done)
		var ms runtime.MemStats
		for {
			select {
			case <-stop:
				return
			case now := <-ticker.C:
				runtime.ReadMemStats(&ms)
				if ms.HeapInuse > peakHeap {
					peakHeap = ms.HeapInuse
				}
				if ms.Sys > peakSys {
					peakSys = ms.Sys
				}
				nowCPU := processCPUTime()
				wallMs := now.Sub(lastWall).Seconds() * 1000.0
				cpuMs := float64(nowCPU-lastCPU) / 1e6
				if wallMs > 0 {
					cpuPercent := cpuMs / (wallMs * float64(runtime.NumCPU())) * 100.0
					if cpuPercent > peakCPUPercent {
						peakCPUPercent = cpuPercent
					}
				}
				lastWall = now
				lastCPU = nowCPU
			}
		}
	}()

	outputPath, err := workload("run")

	elapsed := time.Since(start)
	close(stop)
	<-done
	if err != nil {
		return benchmarkResult{}, err
	}

	cpuAfter := processCPUTime()
	var msAfter runtime.MemStats
	runtime.ReadMemStats(&msAfter)

	const toMb = 1024.0 * 1024.0
	elapsedMs := float64(elapsed.Nanoseconds()) / 1e6
	elapsedSeconds := elapsed.Seconds()
	cpuTotalMs := float64(cpuAfter-cpuBefore) / 1e6
	allocatedBytes := msAfter.TotalAlloc - msBefore.TotalAlloc
	allocatedMb := float64(allocatedBytes) / toMb
	gcPauseMs := float64(msAfter.PauseTotalNs-msBefore.PauseTotalNs) / 1e6

	var fileSize int64
	if info, statErr := os.Stat(outputPath); statErr == nil {
		fileSize = info.Size()
	}

	res := benchmarkResult{
		Library:        library,
		CellCount:      cellCount,
		ElapsedMs:      elapsedMs,
		HeapRetainedMb: float64(int64(msAfter.HeapAlloc)-int64(msBefore.HeapAlloc)) / toMb,
		PeakHeapMb:     float64(peakHeap) / toMb,
		PeakSysMb:      float64(peakSys) / toMb,
		NumGC:          msAfter.NumGC - msBefore.NumGC,
		GcPauseMs:      gcPauseMs,
		HeapSysMb:      float64(msAfter.HeapSys) / toMb,
		AllocatedMb:    allocatedMb,
		OutputFileKb:   float64(fileSize) / 1024.0,
	}
	if elapsedSeconds > 0 {
		res.CellsPerSecond = float64(cellCount) / elapsedSeconds
		res.AllocationRateMbPerSec = allocatedMb / elapsedSeconds
	}
	if cellCount > 0 {
		res.MicrosPerCell = elapsedMs * 1000.0 / float64(cellCount)
		res.AllocatedBytesPerCell = float64(allocatedBytes) / float64(cellCount)
	}
	if elapsedMs > 0 {
		res.AvgCpuPercent = cpuTotalMs / (elapsedMs * float64(runtime.NumCPU())) * 100.0
		res.GcPausePercent = gcPauseMs / elapsedMs * 100.0
	}
	res.PeakCpuPercent = peakCPUPercent
	return res, nil
}

// processCPUTime returns the accumulated user + system CPU time of the current
// process in nanoseconds.
func processCPUTime() int64 {
	var usage syscall.Rusage
	if err := syscall.Getrusage(syscall.RUSAGE_SELF, &usage); err != nil {
		return 0
	}
	toNs := func(tv syscall.Timeval) int64 {
		return int64(tv.Sec)*1e9 + int64(tv.Usec)*1e3
	}
	return toNs(usage.Utime) + toNs(usage.Stime)
}

// writeCurrentPackage writes the requested number of cells using the v4.x package and
// returns the path of the saved workbook.
func writeCurrentPackage(dir string, cellCount int, tag string) (string, error) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		return "", err
	}
	worksheet, err := excel.AddSheet("Benchmark")
	if err != nil {
		return "", err
	}

	columns := cellCount
	if columns > columnsPerRow {
		columns = columnsPerRow
	}
	written := 0
	row := 1
	for written < cellCount {
		rowCount := columns
		if remaining := cellCount - written; remaining < rowCount {
			rowCount = remaining
		}
		cells := make([]spreadsheet_2007.CellProperty, rowCount)
		for column := 0; column < rowCount; column++ {
			cells[column] = spreadsheet_2007.CellProperty{
				Value:    strconv.Itoa(written + column),
				DataType: spreadsheet_2007.CellDataTypeNumber,
			}
		}
		if err := worksheet.SetCellRefValues(fmt.Sprintf("A%d", row), cells); err != nil {
			return "", err
		}
		written += rowCount
		row++
	}

	// The v4.x package requires open worksheets to be released before saving.
	if err := worksheet.Flush(); err != nil {
		return "", err
	}
	path := filepath.Join(dir, fmt.Sprintf("v4x_%d_%s.xlsx", cellCount, tag))
	if err := excel.SaveAs(path); err != nil {
		return "", err
	}
	return path, nil
}

// writeExcelize writes the requested number of cells using the excelize v2 package and
// returns the path of the saved workbook.
func writeExcelize(dir string, cellCount int, tag string) (string, error) {
	f := excelize.NewFile()
	defer f.Close()
	const sheet = "Benchmark"
	if err := f.SetSheetName("Sheet1", sheet); err != nil {
		return "", err
	}

	columns := cellCount
	if columns > columnsPerRow {
		columns = columnsPerRow
	}
	written := 0
	row := 1
	for written < cellCount {
		rowCount := columns
		if remaining := cellCount - written; remaining < rowCount {
			rowCount = remaining
		}
		values := make([]interface{}, rowCount)
		for column := 0; column < rowCount; column++ {
			values[column] = written + column
		}
		if err := f.SetSheetRow(sheet, fmt.Sprintf("A%d", row), &values); err != nil {
			return "", err
		}
		written += rowCount
		row++
	}

	path := filepath.Join(dir, fmt.Sprintf("excelize_%d_%s.xlsx", cellCount, tag))
	if err := f.SaveAs(path); err != nil {
		return "", err
	}
	return path, nil
}

// writeReport renders the collected results as a detailed Markdown report (raw metrics
// table + head-to-head comparison) plus a CSV, writes both to the test_results folder and
// echoes the report to the test output.
func writeReport(t *testing.T, results []benchmarkResult) error {
	t.Helper()
	now := time.Now()
	stamp := now.Format("2006-01-02-15-04-05")

	find := func(count int, current bool) *benchmarkResult {
		for i := range results {
			isCurrent := strings.HasPrefix(results[i].Library, "v4.x")
			if results[i].CellCount == count && isCurrent == current {
				return &results[i]
			}
		}
		return nil
	}

	var md strings.Builder
	md.WriteString("# Spreadsheet Write Benchmark (Go)\n\n")
	fmt.Fprintf(&md, "- Generated: %s\n", now.Format(time.RFC3339))
	hostname, _ := os.Hostname()
	fmt.Fprintf(&md, "- Machine: %s (%d logical CPUs)\n", hostname, runtime.NumCPU())
	fmt.Fprintf(&md, "- Runtime: %s on %s/%s\n", runtime.Version(), runtime.GOOS, runtime.GOARCH)
	fmt.Fprintf(&md, "- Layout: cells written in rows of up to %d columns.\n\n", columnsPerRow)
	md.WriteString("Notes:\n")
	md.WriteString("- **Alloc** = total heap bytes allocated during the run (churn), **Heap Δ** = heap retained afterwards.\n")
	md.WriteString("- **Peak Heap** = peak in-use heap, **Peak Sys** = peak memory reserved from the OS. CPU % is normalised across all logical processors (100% = one fully busy core-equivalent).\n")
	md.WriteString("- **µs/cell** = wall time per cell, **Cells/s** = throughput.\n\n")

	// Executive summary.
	var timeSpeedups, allocRatios []float64
	var curTotalAlloc, exlTotalAlloc float64
	var curTotalGC, exlTotalGC uint32
	var curTotalGcPauseMs, exlTotalGcPauseMs float64
	for _, count := range benchCellCounts {
		cur := find(count, true)
		exl := find(count, false)
		if cur == nil || exl == nil {
			continue
		}
		if cur.ElapsedMs > 0 {
			timeSpeedups = append(timeSpeedups, exl.ElapsedMs/cur.ElapsedMs)
		}
		if cur.AllocatedMb > 0 {
			allocRatios = append(allocRatios, exl.AllocatedMb/cur.AllocatedMb)
		}
		curTotalAlloc += cur.AllocatedMb
		exlTotalAlloc += exl.AllocatedMb
		curTotalGC += cur.NumGC
		exlTotalGC += exl.NumGC
		curTotalGcPauseMs += cur.GcPauseMs
		exlTotalGcPauseMs += exl.GcPauseMs
	}

	md.WriteString("## Summary\n\n")
	md.WriteString("| Metric (across all sizes) | v4.x | excelize v2 |\n")
	md.WriteString("| --- | ---: | ---: |\n")
	fmt.Fprintf(&md, "| Total heap allocation churn (MB) | %.2f | %.2f |\n", curTotalAlloc, exlTotalAlloc)
	fmt.Fprintf(&md, "| Total GC collections | %d | %d |\n", curTotalGC, exlTotalGC)
	fmt.Fprintf(&md, "| Total GC pause time (ms) | %.2f | %.2f |\n\n", curTotalGcPauseMs, exlTotalGcPauseMs)
	fmt.Fprintf(&md, "- v4.x is **%.1f× faster** on average (range %.1f×–%.1f×).\n", avg(timeSpeedups), min(timeSpeedups), max(timeSpeedups))
	fmt.Fprintf(&md, "- v4.x allocates **%.1f× less** heap memory on average (range %.1f×–%.1f×).\n", avg(allocRatios), min(allocRatios), max(allocRatios))
	fmt.Fprintf(&md, "- v4.x triggered **%d** GC collections vs **%d** for excelize.\n\n", curTotalGC, exlTotalGC)

	// Raw metrics.
	md.WriteString("## Raw metrics\n\n")
	md.WriteString("| Library | Cells | Time (ms) | µs/cell | Cells/s | Alloc (MB) | Heap Δ (MB) | Peak Heap (MB) | Peak Sys (MB) | Avg CPU (%) | Peak CPU (%) | GC | File (KB) |\n")
	md.WriteString("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")

	var csv strings.Builder
	csv.WriteString("Library,Cells,TimeMs,MicrosPerCell,CellsPerSecond,AllocatedMb,AllocBytesPerCell,AllocRateMbPerSec,HeapRetainedMb,PeakHeapMb,PeakSysMb,AvgCpuPercent,PeakCpuPercent,NumGC,GcPauseMs,GcPausePercent,HeapSysMb,OutputFileKb\n")

	for _, r := range results {
		fmt.Fprintf(&md, "| %s | %d | %.2f | %.2f | %.0f | %.2f | %.2f | %.2f | %.2f | %.1f | %.1f | %d | %.2f |\n",
			r.Library, r.CellCount, r.ElapsedMs, r.MicrosPerCell, r.CellsPerSecond, r.AllocatedMb, r.HeapRetainedMb, r.PeakHeapMb, r.PeakSysMb, r.AvgCpuPercent, r.PeakCpuPercent, r.NumGC, r.OutputFileKb)
		fmt.Fprintf(&csv, "%s,%d,%.2f,%.2f,%.0f,%.2f,%.0f,%.2f,%.2f,%.2f,%.2f,%.1f,%.1f,%d,%.2f,%.2f,%.2f,%.2f\n",
			r.Library, r.CellCount, r.ElapsedMs, r.MicrosPerCell, r.CellsPerSecond, r.AllocatedMb, r.AllocatedBytesPerCell, r.AllocationRateMbPerSec, r.HeapRetainedMb, r.PeakHeapMb, r.PeakSysMb, r.AvgCpuPercent, r.PeakCpuPercent, r.NumGC, r.GcPauseMs, r.GcPausePercent, r.HeapSysMb, r.OutputFileKb)
	}

	// GC / memory stress focus.
	md.WriteString("\n## GC / memory stress\n\n")
	md.WriteString("Highlights heap allocation churn and garbage collector pressure. High GC counts, GC pause time and allocation rate indicate heavier memory stress.\n\n")
	md.WriteString("| Library | Cells | Alloc (MB) | Alloc/cell (bytes) | Alloc rate (MB/s) | GC | GC pause (ms) | GC pause (%) | Heap Sys (MB) |\n")
	md.WriteString("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")
	for _, r := range results {
		fmt.Fprintf(&md, "| %s | %d | %.2f | %.0f | %.2f | %d | %.2f | %.2f | %.2f |\n",
			r.Library, r.CellCount, r.AllocatedMb, r.AllocatedBytesPerCell, r.AllocationRateMbPerSec, r.NumGC, r.GcPauseMs, r.GcPausePercent, r.HeapSysMb)
	}

	// Head-to-head.
	md.WriteString("\n## Head-to-head: v4.x vs excelize v2\n\n")
	md.WriteString("Each cell is a multiplier comparing the two libraries at the same cell count. **Higher is always better for v4.x**, and `1.00×` means they are equal.\n\n")
	md.WriteString("| Cells | Faster (time) | Higher throughput | Less RAM churn (alloc) | Less GC pause | Lower peak heap | Lower peak sys RAM | Smaller file |\n")
	md.WriteString("| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")
	for _, count := range benchCellCounts {
		cur := find(count, true)
		exl := find(count, false)
		if cur == nil || exl == nil {
			continue
		}
		fmt.Fprintf(&md, "| %d | %s | %s | %s | %s | %s | %s | %s |\n",
			count,
			times(exl.ElapsedMs, cur.ElapsedMs),
			times(cur.CellsPerSecond, exl.CellsPerSecond),
			times(exl.AllocatedMb, cur.AllocatedMb),
			times(exl.GcPauseMs, cur.GcPauseMs),
			times(exl.PeakHeapMb, cur.PeakHeapMb),
			times(exl.PeakSysMb, cur.PeakSysMb),
			times(exl.OutputFileKb, cur.OutputFileKb))
	}

	mdPath := filepath.Join(resultPath, "benchmark-report-"+stamp+".md")
	csvPath := filepath.Join(resultPath, "benchmark-report-"+stamp+".csv")
	if err := os.WriteFile(mdPath, []byte(md.String()), 0o644); err != nil {
		return err
	}
	if err := os.WriteFile(csvPath, []byte(csv.String()), 0o644); err != nil {
		return err
	}

	t.Log("\n" + md.String())
	absMd, _ := filepath.Abs(mdPath)
	absCsv, _ := filepath.Abs(csvPath)
	t.Logf("Full report written to: %s", absMd)
	t.Logf("CSV data: %s", absCsv)
	return nil
}

// times formats a comparison as an "N.NN×" multiplier, or "n/a" when it cannot be computed.
func times(numerator, denominator float64) string {
	if denominator <= 0 {
		return "n/a"
	}
	return fmt.Sprintf("%.2f×", numerator/denominator)
}

func avg(values []float64) float64 {
	if len(values) == 0 {
		return 0
	}
	var sum float64
	for _, v := range values {
		sum += v
	}
	return sum / float64(len(values))
}

func min(values []float64) float64 {
	if len(values) == 0 {
		return 0
	}
	m := values[0]
	for _, v := range values[1:] {
		if v < m {
			m = v
		}
	}
	return m
}

func max(values []float64) float64 {
	if len(values) == 0 {
		return 0
	}
	m := values[0]
	for _, v := range values[1:] {
		if v > m {
			m = v
		}
	}
	return m
}
