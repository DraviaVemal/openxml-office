//! Per-phase time, allocation and memory measurements.
mod common;

use chrono::Utc;
use common::*;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    collections::BTreeMap,
    env, fs,
    io::Write,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Instant,
};

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static TOTAL: AtomicUsize = AtomicUsize::new(0);
static COUNT: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

fn track_growth(bytes: usize) {
    TOTAL.fetch_add(bytes, Relaxed);
    COUNT.fetch_add(1, Relaxed);
    let current = CURRENT.fetch_add(bytes, Relaxed) + bytes;
    PEAK.fetch_max(current, Relaxed);
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = System.alloc(layout);
        if !pointer.is_null() {
            track_growth(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        System.dealloc(pointer, layout);
        CURRENT.fetch_sub(layout.size(), Relaxed);
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let moved = System.realloc(pointer, layout, new_size);
        if !moved.is_null() {
            if new_size >= layout.size() {
                track_growth(new_size - layout.size());
            } else {
                CURRENT.fetch_sub(layout.size() - new_size, Relaxed);
            }
        }
        moved
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

const MB: f64 = 1024.0 * 1024.0;

#[derive(Clone, Copy, Default)]
struct Sample {
    time_ms: f64,
    alloc_mb: f64,
    alloc_count: usize,
    peak_live_mb: f64,
    peak_rss_mb: f64,
}

fn rss_field(name: &str) -> f64 {
    fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|status| {
            status
                .lines()
                .find(|line| line.starts_with(name))
                .and_then(|line| line.split_whitespace().nth(1)?.parse::<f64>().ok())
        })
        .map(|kb| kb / 1024.0)
        .unwrap_or(0.0)
}

fn measure<T>(work: impl FnOnce() -> T) -> (T, Sample) {
    // Reset the Linux RSS high-water mark.
    let _ = fs::write("/proc/self/clear_refs", "5");
    let base_live = CURRENT.load(Relaxed);
    PEAK.store(base_live, Relaxed);
    let base_total = TOTAL.load(Relaxed);
    let base_count = COUNT.load(Relaxed);
    let start = Instant::now();
    let result = work();
    let elapsed = start.elapsed();
    let sample = Sample {
        time_ms: elapsed.as_secs_f64() * 1000.0,
        alloc_mb: (TOTAL.load(Relaxed) - base_total) as f64 / MB,
        alloc_count: COUNT.load(Relaxed) - base_count,
        peak_live_mb: PEAK.load(Relaxed).saturating_sub(base_live) as f64 / MB,
        peak_rss_mb: rss_field("VmHWM:"),
    };
    (result, sample)
}

fn median_sample(samples: &[Sample]) -> Sample {
    let mut sorted = samples.to_vec();
    sorted.sort_by(|left, right| left.time_ms.total_cmp(&right.time_ms));
    sorted[sorted.len() / 2]
}

struct Scenario {
    name: &'static str,
    phases: &'static [&'static str],
    run: fn(&Fixtures) -> Vec<Sample>,
}

struct Fixtures {
    large: String,
    many: String,
    out: String,
}

fn write_100k(fixtures: &Fixtures) -> Vec<Sample> {
    let (mut excel, build) = measure(|| {
        let mut excel = new_excel();
        populate(&mut excel, 1, 100_000);
        excel
    });
    let (_, save) = measure(|| excel.save_as(&fixtures.out).expect("save"));
    vec![build, save]
}

fn open_edit_save(path: &str, out: &str) -> Vec<Sample> {
    let (mut excel, open) = measure(|| open_excel(path));
    let (_, edit) = measure(|| edit_first_cell(&mut excel, "Sheet_0"));
    let (_, save) = measure(|| excel.save_as(out).expect("save"));
    vec![open, edit, save]
}

fn open_save_untouched(path: &str, out: &str) -> Vec<Sample> {
    let (excel, open) = measure(|| open_excel(path));
    let (_, save) = measure(|| excel.save_as(out).expect("save"));
    vec![open, save]
}

fn git_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|commit| commit.trim().to_string())
        .filter(|commit| !commit.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn history_path() -> PathBuf {
    let directory = env::var("BENCH_HISTORY_DIR")
        .unwrap_or_else(|_| format!("{}/../tests/benchmark_history", env!("CARGO_MANIFEST_DIR")));
    PathBuf::from(directory).join("rs.csv")
}

const HISTORY_HEADER: &str = "timestamp,version,commit,library,scenario,phase,time_ms,min_ms,max_ms,alloc_mb,alloc_count,peak_rss_mb,out_kb";

/// Read the latest prior-release result for each scenario phase.
fn previous_release(version: &str) -> BTreeMap<String, (String, f64, f64)> {
    let mut previous = BTreeMap::new();
    let Ok(content) = fs::read_to_string(history_path()) else {
        return previous;
    };
    for line in content.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 13 || fields[1] == version {
            continue;
        }
        if let (Ok(time), Ok(alloc)) = (fields[6].parse::<f64>(), fields[9].parse::<f64>()) {
            previous.insert(
                format!("{}/{}", fields[4], fields[5]),
                (fields[1].to_string(), time, alloc),
            );
        }
    }
    previous
}

fn change(current: f64, before: f64) -> String {
    if before <= 0.0 {
        return "n/a".to_string();
    }
    format!("{:+.1}%", (current - before) / before * 100.0)
}

fn main() {
    let iterations: usize = env::var("BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(5);
    let version = env!("CARGO_PKG_VERSION");
    let record = env::var("BENCH_RECORD")
        .map(|setting| setting == "1")
        .unwrap_or(false);

    let dir = tempfile::tempdir().expect("temp dir");
    let path = |name: &str| dir.path().join(name).to_string_lossy().to_string();
    let fixtures = Fixtures {
        large: path("large.xlsx"),
        many: path("many.xlsx"),
        out: path("out.xlsx"),
    };
    build_fixture(&fixtures.large, 1, 100_000);
    build_fixture(&fixtures.many, 50, 10_000);

    let scenarios = [
        Scenario {
            name: "write_new_100k_cells",
            phases: &["build", "save"],
            run: write_100k,
        },
        Scenario {
            name: "open_edit_save_100k_cells_1_sheet",
            phases: &["open", "edit", "save"],
            run: |fixtures| open_edit_save(&fixtures.large, &fixtures.out),
        },
        Scenario {
            name: "open_edit_save_10k_cells_50_sheets",
            phases: &["open", "edit", "save"],
            run: |fixtures| open_edit_save(&fixtures.many, &fixtures.out),
        },
        Scenario {
            name: "open_save_untouched_50_sheets",
            phases: &["open", "save"],
            run: |fixtures| open_save_untouched(&fixtures.many, &fixtures.out),
        },
    ];

    // Warm each scenario once.
    for scenario in &scenarios {
        (scenario.run)(&fixtures);
    }

    let previous = previous_release(version);
    let timestamp = Utc::now().to_rfc3339();
    let commit = git_commit();
    let mut report = String::new();
    let mut rows = String::new();
    report.push_str(&format!(
        "# Rust footprint benchmark\n\n- Version: {version} ({commit})\n- Iterations: {iterations} (median reported)\n- Alloc and live memory are measured by a counting allocator (all threads); RSS is the kernel peak for the phase.\n\n"
    ));
    report.push_str("| Scenario | Phase | Time (ms) | Min | Max | Alloc (MB) | Allocs | Peak live (MB) | Peak RSS (MB) | Out (KB) | Time vs previous | Alloc vs previous |\n");
    report.push_str(
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n",
    );

    for scenario in &scenarios {
        let mut per_phase: Vec<Vec<Sample>> = vec![Vec::new(); scenario.phases.len()];
        let mut totals: Vec<f64> = Vec::new();
        for _ in 0..iterations {
            let samples = (scenario.run)(&fixtures);
            totals.push(samples.iter().map(|sample| sample.time_ms).sum());
            for (phase_index, sample) in samples.into_iter().enumerate() {
                per_phase[phase_index].push(sample);
            }
        }
        let out_kb = fs::metadata(&fixtures.out)
            .map(|meta| meta.len() as f64 / 1024.0)
            .unwrap_or(0.0);
        totals.sort_by(f64::total_cmp);
        let phase_medians: Vec<Sample> = per_phase
            .iter()
            .map(|phase_samples| median_sample(phase_samples))
            .collect();
        let total = Sample {
            time_ms: totals[totals.len() / 2],
            alloc_mb: phase_medians.iter().map(|s| s.alloc_mb).sum(),
            alloc_count: phase_medians.iter().map(|s| s.alloc_count).sum(),
            peak_live_mb: phase_medians
                .iter()
                .fold(0.0, |peak, sample| sample.peak_live_mb.max(peak)),
            peak_rss_mb: phase_medians
                .iter()
                .fold(0.0, |peak, sample| sample.peak_rss_mb.max(peak)),
        };
        let min_total = totals[0];
        let max_total = totals[totals.len() - 1];

        let mut lines: Vec<(String, Sample, f64, f64)> = scenario
            .phases
            .iter()
            .zip(&phase_medians)
            .zip(&per_phase)
            .map(|((phase, median), samples)| {
                let minimum = samples
                    .iter()
                    .map(|sample| sample.time_ms)
                    .fold(f64::MAX, f64::min);
                let maximum = samples
                    .iter()
                    .map(|sample| sample.time_ms)
                    .fold(0.0, f64::max);
                (phase.to_string(), *median, minimum, maximum)
            })
            .collect();
        lines.push(("total".to_string(), total, min_total, max_total));

        for (phase, sample, min, max) in lines {
            let key = format!("{}/{}", scenario.name, phase);
            let (time_change, alloc_change) = match previous.get(&key) {
                Some((old_version, old_time, old_alloc)) => (
                    format!("{} (v{})", change(sample.time_ms, *old_time), old_version),
                    change(sample.alloc_mb, *old_alloc),
                ),
                None => ("-".to_string(), "-".to_string()),
            };
            report.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {} | {:.2} | {:.1} | {:.1} | {} | {} |\n",
                scenario.name, phase, sample.time_ms, min, max, sample.alloc_mb, sample.alloc_count,
                sample.peak_live_mb, sample.peak_rss_mb, out_kb, time_change, alloc_change
            ));
            rows.push_str(&format!(
                "{},{},{},rust,{},{},{:.3},{:.3},{:.3},{:.3},{},{:.2},{:.2}\n",
                timestamp,
                version,
                commit,
                scenario.name,
                phase,
                sample.time_ms,
                min,
                max,
                sample.alloc_mb,
                sample.alloc_count,
                sample.peak_rss_mb,
                out_kb
            ));
        }
    }

    println!("{report}");
    if record {
        let path = history_path();
        fs::create_dir_all(path.parent().expect("history dir")).expect("create history dir");
        let is_new = !path.exists();
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open history");
        if is_new {
            writeln!(file, "{HISTORY_HEADER}").expect("write header");
        }
        file.write_all(rows.as_bytes()).expect("write history");
        println!("History appended to {}", path.display());
    }
}
