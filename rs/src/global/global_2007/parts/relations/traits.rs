use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    global_2007::{
        parts::relations::RelationsPart,
        traits::{XmlDocumentPartClose, XmlDocumentPartInitializing},
    },
    namespaces::RELATIONSHIP_PKG_NS,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlDocument};

impl_xml_part_lifecycle!(RelationsPart);

impl XmlDocumentPartClose for RelationsPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        if let Some(office_document) = self.office_document.upgrade() {
            let has_relationships = self.save_relationship_to_doc().context(
                "draviavemal-openxml_office::Failed to Insert relationship to Relationships",
            )?;
            let mut office_document_mut = office_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Pull Open XML Relations Handle")?;
            if has_relationships {
                office_document_mut.close_xml_document(&self.file_path)?;
            } else {
                office_document_mut.delete_document_mut(&self.file_path);
            }
        }
        Ok(())
    }
}

impl XmlDocumentPartInitializing for RelationsPart {
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let relationship_content = COMMON_TYPE_COLLECTION
            .get("rels")
            .context("Failed to read Common type collection")?;
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_element_ns_mut(
                "Relationships",
                &NamespaceDeclaration {
                    default_alias: RELATIONSHIP_PKG_NS.default_alias,
                    uri: RELATIONSHIP_PKG_NS.uri,
                    alias_override: Some(""),
                },
                None,
            )
            .context("draviavemal-openxml_office::Create XML Root Element Failed")?;
        Ok((
            xml_document,
            None,
            relationship_content.extension.to_string(),
            relationship_content.extension_type.to_string(),
        ))
    }
}
