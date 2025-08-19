use crate::element_dictionary::COMMON_TYPE_COLLECTION;
use anyhow::{Context, Error as AnyError};
use draviavemal_xml_rs::{XmlAttribute, XmlDeserializer, XmlDocument, XmlSerializer};

#[derive(Debug)]
pub(crate) struct ContentTypesPart {
    xml_document: XmlDocument,
}

impl ContentTypesPart {
    pub(crate) fn new(xml_file_content: Vec<u8>) -> Result<Self, AnyError> {
        let xml_document = XmlDeserializer::vec_to_xml_doc_tree(xml_file_content)
            .context("Decoding Content Type Failed")?;
        Ok(Self { xml_document })
    }
    pub(crate) fn get_extensions(&mut self) -> Result<Option<Vec<(String, String)>>, AnyError> {
        let mut elements: Vec<(String, String)> = Vec::new();
        let root_id = self.xml_document.get_root_id();
        let default_element_ids = self
            .xml_document
            .find_all_child(root_id, "Default")
            .context("Failed to get Default element collection")?;
        if let Some(default_element_ids) = default_element_ids {
            for default_element_id in default_element_ids {
                let default_element = self
                    .xml_document
                    .get_element(default_element_id)
                    .context("Element Attribute not Found")?;
                elements.push((
                    default_element
                        .get_attribute("Extension")
                        .context("content type default attribute missing")?
                        .get_value()
                        .to_string(),
                    default_element
                        .get_attribute("ContentType")
                        .context("content type default attribute missing")?
                        .get_value()
                        .to_string(),
                ));
            }
            if elements.len() > 0 {
                return Ok(Some(elements));
            }
        }
        Ok(None)
    }

    pub(crate) fn get_override_content_type(
        &mut self,
        file_name: &str,
    ) -> Result<Option<String>, AnyError> {
        let root_id = self.xml_document.get_root_id();
        if let Some(element_id) = self
            .xml_document
            .find_first_by_attribute(root_id, "PartName", &format!("/{}", file_name))
            .context("Failed to find all attribute childs")?
        {
            let element = self
                .xml_document
                .get_element(element_id)
                .context("Failed to get override type element")?;
            Ok(Some(
                element
                    .get_attribute("ContentType")
                    .context("Failed to get ContentType attribute")?
                    .get_value()
                    .to_string(),
            ))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn create_xml_file(
        extensions: Vec<(String, String)>,
        overrides: Vec<(String, String)>,
    ) -> Result<Vec<u8>, AnyError> {
        let mut document = XmlDocument::new();
        document
            .create_root_element_mut(
                "Types",
                Some(vec![XmlAttribute::new(
                    "xmlns".to_string(),
                    COMMON_TYPE_COLLECTION
                        .get("content_type")
                        .unwrap()
                        .schemas_namespace
                        .to_string(),
                )]),
            )
            .context("Failed to Create Root Element")?;
        // Load Default Elements
        {
            for (extension, content_type) in extensions {
                let root_id = document.get_root_id();
                document
                    .append_child_element_mut(
                        root_id,
                        "Default",
                        Some(vec![
                            XmlAttribute::new("Extension".to_string(), extension),
                            XmlAttribute::new("ContentType".to_string(), content_type),
                        ]),
                    )
                    .context("Append child to root failed")?;
            }
        }
        // Load Override Elements
        {
            for (part_name, content_type) in overrides {
                let root_id = document.get_root_id();
                document
                    .append_child_element_mut(
                        root_id,
                        "Override",
                        Some(vec![
                            XmlAttribute::new("PartName".to_string(), part_name),
                            XmlAttribute::new("ContentType".to_string(), content_type),
                        ]),
                    )
                    .context("Append child to root failed")?;
            }
        }
        XmlSerializer::xml_tree_to_vec(&mut document)
    }
}
