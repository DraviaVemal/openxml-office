use anyhow::Context;
use chrono::Utc;
use std::fs::{create_dir, exists};

fn get_save_file(dynamic_path: Option<&str>) -> String {
    let result_path = "test_results";
    if !exists(result_path).expect("Dir Check Failed") {
        create_dir(result_path).expect("Failed to Create")
    }
    format!(
        "{}/test-{}{}.pptx",
        result_path,
        dynamic_path.unwrap_or(""),
        Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string()
    )
}

#[test]
fn blank_power_point() {
    let file = crate::presentation_2007::PowerPoint::new(
        None,
        crate::presentation_2007::PowerPoint::default(),
    )
    .expect("Create New File Failed");
    file.save_as(&get_save_file(None))
        .expect("Failed to save Empty Power Point");
    assert_eq!(true, true);
}

#[test]
fn edit_power_point() {
    let file = crate::presentation_2007::PowerPoint::new(
        Some(
            "src/tests/TestFiles/basic_test.pptx"
                .to_string(),
        ),
        crate::presentation_2007::PowerPoint::default(),
    )
    .expect("Open Existing file failed");
    file.save_as(&get_save_file(None))
        .expect("Failed to save Edit Power Point");
    assert_eq!(true, true);
}
