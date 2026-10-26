use crate::namespaces::CONTENT_TYPES_NS;
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{
    NamespaceDeclaration, XmlAttribute, XmlDeserializer, XmlDocument, XmlSerializer,
};

#[derive(Debug)]
pub(crate) struct ContentTypesPart {
    xml_document: XmlDocument,
}

impl ContentTypesPart {
    pub(crate) fn new(xml_file_content: Vec<u8>) -> AnyResult<Self, AnyError> {
        let xml_document = XmlDeserializer::vec_to_xml_doc_tree(xml_file_content)
            .context("draviavemal-openxml_office::Decoding Content Type Failed")?;
        Ok(Self { xml_document })
    }
    /// Read every `Default` (extension, type) and `Override` (part name, type) entry.
    pub(crate) fn read_all(
        &mut self,
    ) -> AnyResult<(Vec<(String, String)>, Vec<(String, String)>), AnyError> {
        let root_id = self.xml_document.get_root_id();
        let mut defaults: Vec<(String, String)> = Vec::new();
        let mut overrides: Vec<(String, String)> = Vec::new();
        for (tag, key_attribute, collection) in [
            ("Default", "Extension", &mut defaults),
            ("Override", "PartName", &mut overrides),
        ] {
            if let Some(ids) = self
                .xml_document
                .find_all_child(root_id, tag)
                .context("draviavemal-openxml_office::Failed to find content type elements")?
            {
                for id in ids {
                    let element = self
                        .xml_document
                        .get_element(id)
                        .context("draviavemal-openxml_office::Element not Found")?;
                    if let (Some(key), Some(content_type)) = (
                        element.get_attribute(key_attribute),
                        element.get_attribute("ContentType"),
                    ) {
                        collection.push((
                            key.get_value().to_string(),
                            content_type.get_value().to_string(),
                        ));
                    }
                }
            }
        }
        Ok((defaults, overrides))
    }

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
        // Load Default Elements
        {
            for (extension, content_type) in extensions {
                let mut attributes = Vec::new();
                attributes.push(XmlAttribute::new("Extension".to_string(), extension));
                attributes.push(XmlAttribute::new("ContentType".to_string(), content_type));
                document
                    .append_child_element_mut(root_element_id, "Default", Some(attributes))
                    .context("draviavemal-openxml_office::Append child to root failed")?;
            }
        }
        // Load Override Elements
        {
            for (part_name, content_type) in overrides {
                let mut attributes = Vec::new();
                attributes.push(XmlAttribute::new("PartName".to_string(), part_name));
                attributes.push(XmlAttribute::new("ContentType".to_string(), content_type));
                document
                    .append_child_element_mut(root_element_id, "Override", Some(attributes))
                    .context("draviavemal-openxml_office::Append child to root failed")?;
            }
        }
        XmlSerializer::xml_tree_to_vec(&mut document)
    }
}
