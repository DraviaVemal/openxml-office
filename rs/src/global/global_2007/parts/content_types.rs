use crate::element_dictionary::COMMON_TYPE_COLLECTION;
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlAttribute, XmlDeserializer, XmlDocument, XmlSerializer};

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
    pub(crate) fn get_extensions(&mut self) -> AnyResult<Option<Vec<(String, String)>>, AnyError> {
        let mut elements: Vec<(String, String)> = Vec::new();
        let root_id = self.xml_document.get_root_id();
        if let Some(default_ids) = self
            .xml_document
            .find_all_child(root_id, "Default")
            .context("draviavemal-openxml_office::Failed to find Default elements")?
        {
            for default_id in default_ids {
                let default_element = self
                    .xml_document
                    .get_element(default_id)
                    .context("draviavemal-openxml_office::Element not Found")?;
                elements.push((
                    default_element
                        .get_attribute("Extension")
                        .context(
                            "draviavemal-openxml_office::content type default attribute missing",
                        )?
                        .get_value()
                        .to_string(),
                    default_element
                        .get_attribute("ContentType")
                        .context(
                            "draviavemal-openxml_office::content type default attribute missing",
                        )?
                        .get_value()
                        .to_string(),
                ));
                self.xml_document
                    .remove_element_mut(default_id)
                    .context("draviavemal-openxml_office::Falied to remove element from tree")?;
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
    ) -> AnyResult<Option<String>, AnyError> {
        let root_id = self.xml_document.get_root_id();
        if let Some(mut find_ids) = self
            .xml_document
            .find_all_by_attribute(root_id, "PartName", &format!("/{}", file_name))
            .context("draviavemal-openxml_office::Failed to find override by attribute")?
        {
            if let Some(id) = find_ids.pop() {
                let element = self
                    .xml_document
                    .get_element(id)
                    .context("draviavemal-openxml_office::Failed to pull element")?;
                if let Some(attribute) = element.get_attribute("ContentType") {
                    return Ok(Some(attribute.get_value().to_string()));
                }
                self.xml_document
                    .remove_element_mut(id)
                    .context("draviavemal-openxml_office::Falied to remove element from tree")?;
            }
        }
        Ok(None)
    }

    pub(crate) fn create_xml_file(
        extensions: Vec<(String, String)>,
        overrides: Vec<(String, String)>,
    ) -> AnyResult<Vec<u8>, AnyError> {
        let mut document = XmlDocument::new();
        let mut attributes = Vec::new();
        attributes.push(XmlAttribute::new(
            "xmlns".to_string(),
            COMMON_TYPE_COLLECTION
                .get("content_type")
                .context("Failed to load common type collection")?
                .schemas_namespace
                .to_string(),
        ));
        let root_element_id = document
            .create_root_element_mut("Types", Some(attributes))
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
