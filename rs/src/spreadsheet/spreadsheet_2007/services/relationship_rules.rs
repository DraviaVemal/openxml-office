use crate::global::traits::RelationshipRules;

/// Spreadsheet packages keep every relationship target under `xl`.
#[derive(Debug, Default)]
pub(crate) struct SpreadsheetRelationshipRules;

impl RelationshipRules for SpreadsheetRelationshipRules {
    fn package_root_directory(&self, _relative_path: &str) -> Option<String> {
        Some("xl".to_string())
    }
}
