#[cfg(test)]
mod presentation_test {
    use chrono::Utc;
    use draviavemal_openxml_office::presentation_2007::{PowerPoint, PowerPointPropertiesModel};
    use std::fs::{create_dir_all, exists};

    fn get_save_file(dynamic_path: Option<&str>) -> String {
        let result_path = "test_results";
        if !exists(result_path).expect("Dir Check Failed") {
            create_dir_all(result_path).expect("Failed to Create")
        }
        format!(
            "{}/test-{}{}.pptx",
            result_path,
            dynamic_path.unwrap_or(""),
            Utc::now().format("%Y-%m-%d-%H-%M-%S-%3f").to_string()
        )
    }

    #[test]
    fn blank_power_point() {
        let file = PowerPoint::new(None, PowerPointPropertiesModel::default())
            .expect("Create New File Failed");
        file.save_as(&get_save_file(None))
            .expect("Failed to save Empty Power Point");
        assert_eq!(true, true);
    }

    #[test]
    fn edit_power_point() {
        let file = PowerPoint::new(
            Some("src/TestFiles/basic_test.pptx".to_string()),
            PowerPointPropertiesModel::default(),
        )
        .expect("Open Existing file failed");
        file.save_as(&get_save_file(None))
            .expect("Failed to save Edit Power Point");
        assert_eq!(true, true);
    }
}
