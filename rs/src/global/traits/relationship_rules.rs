use std::fmt::Debug;

use crate::element_dictionary::IMAGE_RELATIONSHIP_TYPE;

/// Document families override these defaults to change how shared relationship parts behave.
pub(crate) trait RelationshipRules: Debug {
    fn image_relationship_type(&self) -> &'static str {
        IMAGE_RELATIONSHIP_TYPE
    }

    /// Top-level package directory that owns the relationship file, e.g. `xl`.
    fn package_root_directory(&self, relative_path: &str) -> Option<String> {
        relative_path
            .split('/')
            .next()
            .filter(|directory| !directory.is_empty())
            .map(str::to_string)
    }

    fn media_directory(&self, relative_path: &str) -> Option<String> {
        self.package_root_directory(relative_path)
            .map(|root_directory| format!("{}/media", root_directory))
    }
}

#[derive(Debug, Default)]
pub(crate) struct GlobalRelationshipRules;

impl RelationshipRules for GlobalRelationshipRules {}
