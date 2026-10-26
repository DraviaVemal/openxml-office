use std::{cell::RefCell, rc::Weak};

use crate::global_2007::{
    files::OfficeDocument,
    parts::relations::RelationsPart,
    traits::{GlobalRelationshipRules, RelationshipRules, XmlDocumentPartInitializing},
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};

impl RelationsPart {
    pub(crate) fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        file_name: &str,
    ) -> AnyResult<RelationsPart, AnyError> {
        RelationsPart::with_rules(
            office_document,
            file_name,
            Box::new(GlobalRelationshipRules),
        )
    }

    /// Open a relationship part with document-specific overrides of the shared rules.
    pub(crate) fn with_rules(
        office_document: Weak<RefCell<OfficeDocument>>,
        file_name: &str,
        rules: Box<dyn RelationshipRules>,
    ) -> AnyResult<RelationsPart, AnyError> {
        let mut xml_document = RelationsPart::get_xml_document(&office_document, file_name)?;
        let relationships = RelationsPart::load_relations(&mut xml_document)
            .context("draviavemal-openxml_office::Failed to decode Relations")?;
        Ok(RelationsPart {
            office_document,
            xml_document,
            relationships,
            file_path: file_name.to_string(),
            rules,
        })
    }

    pub(super) fn get_next_relationship_id(&self) -> String {
        let mut candidate_number = self.relationships.len() + 1;
        while self
            .relationships
            .iter()
            .any(|relationship| relationship.0 == format!("rId{}", candidate_number))
        {
            candidate_number += 1;
        }
        format!("rId{}", candidate_number)
    }
}
