#[derive(Debug)]
pub(crate) struct MediaGlobal {
    hash: String,
}

impl MediaGlobal {
    pub(crate) fn new(hash: &str) -> MediaGlobal {
        MediaGlobal {
            hash: hash.to_string(),
        }
    }
}
