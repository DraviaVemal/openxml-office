#[cfg(test)]
mod document_test {
    use chrono::Utc;
    use draviavemal_openxml_office::document_2007::{Word, WordPropertiesModel};
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
        let file = Word::new(None, WordPropertiesModel::default()).expect("Blank File Open Failed");
        file.save_as(&get_save_file(None))
            .expect("Save Result Failed");
        assert_eq!(true, true);
    }

    #[test]
    fn edit_document() {
        let file = Word::new(
            Some("src/tests/TestFiles/basic_test.docx".to_string()),
            WordPropertiesModel::default(),
        )
        .expect("Edit existing file failed");
        file.save_as(&get_save_file(None))
            .expect("Save Result Failed");
        assert_eq!(true, true);
    }
}
