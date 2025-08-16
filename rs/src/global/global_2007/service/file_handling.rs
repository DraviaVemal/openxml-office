use anyhow::Error as AnyError;

#[derive(Debug)]
pub(crate) struct MediaFiles {
    file_names: Vec<String>,
}

impl MediaFiles {
    pub(crate) fn new() -> Result<MediaFiles, AnyError> {
        Ok(Self {
            file_names: Vec::new(),
        })
    }

    pub(crate) fn add_new_file() {}

    pub(crate) fn check_file_exist() {}

    pub(crate) fn remove_file() {}
}
