use crate::{global_2007::parts::content_types::ContentTypesPart, namespaces::CONTENT_TYPES_NS};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlAttribute, XmlDocument, XmlSerializer};

impl ContentTypesPart {
    pub(crate) fn create_xml_file(
        extensions: Vec<(String, String)>,
        overrides: Vec<(String, String)>,
    ) -> AnyResult<Vec<u8>, AnyError> {
        let mut document = XmlDocument::new();
        let root_element_id = document
            .create_root_element_ns_mut(
                "Types",
                &NamespaceDeclaration {
                    default_alias: CONTENT_TYPES_NS.default_alias,
                    uri: CONTENT_TYPES_NS.uri,
                    alias_override: Some(""),
                },
                None,
            )
            .context("draviavemal-openxml_office::Failed to Create Root Element")?;
        for (extension, content_type) in extensions {
            let attributes = vec![
                XmlAttribute::new("Extension".to_string(), extension),
                XmlAttribute::new("ContentType".to_string(), content_type),
            ];
            document
                .append_child_element_mut(root_element_id, "Default", Some(attributes))
                .context("draviavemal-openxml_office::Append child to root failed")?;
        }
        for (part_name, content_type) in overrides {
            let attributes = vec![
                XmlAttribute::new("PartName".to_string(), part_name),
                XmlAttribute::new("ContentType".to_string(), content_type),
            ];
            document
                .append_child_element_mut(root_element_id, "Override", Some(attributes))
                .context("draviavemal-openxml_office::Append child to root failed")?;
        }
        XmlSerializer::xml_tree_to_vec(&mut document)
    }
}
