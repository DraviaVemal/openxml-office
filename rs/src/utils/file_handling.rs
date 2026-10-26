use std::path::Path;

pub(crate) fn clean_file_path(path: &str) -> String {
    let mut result_path = Vec::new();
    for part in Path::new(&path).components() {
        match part {
            std::path::Component::ParentDir => {
                result_path.pop();
            }
            std::path::Component::CurDir => {}
            _ => {
                result_path.push(part.as_os_str().to_string_lossy().to_string());
            }
        };
    }
    result_path.join("/")
}
