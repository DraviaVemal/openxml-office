use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    time::Instant,
};

use draviavemal_openxml_office::spreadsheet_2007::{
    CellDataType, CellProperty,
    Excel, ExcelPropertiesModel,
};
use tempfile::tempdir;

const CELL_COUNT: usize = 100_000;
const COLUMNS_PER_ROW: usize = 100;
const WORKLOAD: &str = "write_new_100k_cells_one_sheet";
const CSV_HEADER: &str =
    "language,workload,cells,rows,columns,iterations,median_ms,min_ms,max_ms,cells_per_second";

fn write_workbook(output_path: &Path) {
    let mut excel = Excel::new(None, ExcelPropertiesModel::default()).expect("create workbook");
    {
        let mut worksheet = excel
            .add_sheet_mut(Some("Benchmark".to_string()))
            .expect("add worksheet");
        for row_index in 0..CELL_COUNT / COLUMNS_PER_ROW {
            let cells = (0..COLUMNS_PER_ROW)
                .map(|column_index| {
                    let value = row_index * COLUMNS_PER_ROW + column_index;
                    CellProperty::default()
                        .set_value(Some(value.to_string()))
                        .set_data_type(CellDataType::Number)
                })
                .collect();
            worksheet
                .set_cell_index_value_mut(row_index as u32 + 1, 1, cells)
                .expect("write worksheet row");
        }
    }
    excel
        .save_as(output_path.to_str().expect("UTF-8 output path"))
        .expect("save workbook");
}

fn append_result(iterations: usize, timings: &[f64]) {
    let mut sorted = timings.to_vec();
    sorted.sort_by(f64::total_cmp);
    let median_ms = sorted[sorted.len() / 2];
    let minimum_ms = sorted[0];
    let maximum_ms = sorted[sorted.len() - 1];
    let cells_per_second = CELL_COUNT as f64 / (median_ms / 1000.0);
    let row = format!(
        "rust,{WORKLOAD},{CELL_COUNT},{},{COLUMNS_PER_ROW},{iterations},{median_ms:.3},{minimum_ms:.3},{maximum_ms:.3},{cells_per_second:.0}",
        CELL_COUNT / COLUMNS_PER_ROW,
    );

    if let Ok(output_path) = env::var("PARITY_BENCH_OUTPUT") {
        let output_path = Path::new(&output_path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("create benchmark report directory");
        }
        let needs_header = fs::metadata(output_path)
            .map(|metadata| metadata.len() == 0)
            .unwrap_or(true);
        let mut report = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_path)
            .expect("open benchmark report");
        if needs_header {
            writeln!(report, "{CSV_HEADER}").expect("write CSV header");
        }
        writeln!(report, "{row}").expect("write Rust benchmark result");
    }
    println!("{row}");
}

fn main() {
    let iterations = env::var("PARITY_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|iterations| *iterations > 0)
        .unwrap_or(5);
    let temporary_directory = tempdir().expect("create benchmark temp directory");
    let warmup_path = temporary_directory.path().join("warmup.xlsx");
    write_workbook(&warmup_path);
    fs::remove_file(warmup_path).expect("remove warmup workbook");

    let mut timings = Vec::with_capacity(iterations);
    for iteration in 0..iterations {
        let output_path = temporary_directory
            .path()
            .join(format!("rust-{iteration}.xlsx"));
        let start = Instant::now();
        write_workbook(&output_path);
        timings.push(start.elapsed().as_secs_f64() * 1000.0);
        fs::remove_file(output_path).expect("remove benchmark workbook");
    }
    append_result(iterations, &timings);
}
