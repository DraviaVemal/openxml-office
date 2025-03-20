use anyhow::Context;
use chrono::Utc;
use std::fs::{create_dir, exists};

fn get_save_file(dynamic_path: Option<&str>) -> String {
    let result_path = "test_results";
    if !exists(result_path).expect("Dir Check Failed") {
        create_dir(result_path).expect("Failed to Create")
    }
    format!(
        "{}/test-{}{}.docx",
        result_path,
        dynamic_path.unwrap_or(""),
        Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string()
    )
}

#[test]
fn blank_document() {
    let file = crate::document_2007::Word::new(None, crate::document_2007::Word::default())
        .expect("Blank File Open Failed");
    file.save_as(&get_save_file(None))
        .expect("Save Result Failed");
    assert_eq!(true, true);
}

#[test]
fn edit_document() {
    let file = crate::document_2007::Word::new(
        Some(
            "/home/draviavemal/repo/OpenXML-Office/rs/document/src/tests/TestFiles/test_file.docx"
                .to_string(),
        ),
        crate::document_2007::Word::default(),
    )
    .expect("Edit existing file failed");
    file.save_as(&get_save_file(None))
        .expect("Save Result Failed");
    assert_eq!(true, true);
}
