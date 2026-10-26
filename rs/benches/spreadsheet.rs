//! Criterion benchmarks for workbook operations.
mod common;

use common::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::{hint::black_box, time::Duration};

const WRITE_SIZES: [usize; 3] = [1_000, 10_000, 100_000];

fn bench_spreadsheet(c: &mut Criterion) {
    let dir = tempfile::tempdir().expect("temp dir");
    let large = dir.path().join("large.xlsx").to_string_lossy().to_string();
    let many = dir.path().join("many.xlsx").to_string_lossy().to_string();
    let out = dir.path().join("out.xlsx").to_string_lossy().to_string();
    build_fixture(&large, 1, 100_000);
    build_fixture(&many, 50, 10_000);

    let mut write = c.benchmark_group("write_new_workbook");
    for cells in WRITE_SIZES {
        write.throughput(Throughput::Elements(cells as u64));
        write.bench_with_input(
            BenchmarkId::from_parameter(cells),
            &cells,
            |bencher, &cell_count| {
                bencher.iter(|| {
                    let mut excel = new_excel();
                    populate(&mut excel, 1, cell_count);
                    black_box(excel.save_as(&out).expect("save"));
                })
            },
        );
    }
    write.finish();

    let mut open = c.benchmark_group("open_only");
    for (name, path) in [
        ("100k_cells_1_sheet", &large),
        ("10k_cells_50_sheets", &many),
    ] {
        open.bench_function(name, |bencher| bencher.iter(|| black_box(open_excel(path))));
    }
    open.finish();

    let mut untouched = c.benchmark_group("open_save_untouched");
    for (name, path) in [
        ("100k_cells_1_sheet", &large),
        ("10k_cells_50_sheets", &many),
    ] {
        untouched.bench_function(name, |bencher| {
            bencher.iter(|| black_box(open_excel(path).save_as(&out).expect("save")))
        });
    }
    untouched.finish();

    let mut edit = c.benchmark_group("open_edit_save");
    for (name, path, sheet) in [
        ("100k_cells_1_sheet", &large, "Sheet_0"),
        ("10k_cells_50_sheets", &many, "Sheet_0"),
    ] {
        edit.bench_function(name, |bencher| {
            bencher.iter(|| {
                let mut excel = open_excel(path);
                edit_first_cell(&mut excel, sheet);
                black_box(excel.save_as(&out).expect("save"));
            })
        });
    }
    edit.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(10));
    targets = bench_spreadsheet
}
criterion_main!(benches);
