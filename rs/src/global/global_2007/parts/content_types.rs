use std::collections::HashMap;

use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    files::{XmlSerializer, XmlDocument, XmlDeSerializer},
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};

#[derive(Debug)]
pub(crate) struct ContentTypesPart {
    xml_document: XmlDocument,
}

impl ContentTypesPart {
    pub(crate) fn new(xml_file_content: Vec<u8>) -> AnyResult<Self, AnyError> {
        let xml_document =
            XmlDeSerializer::vec_to_xml_doc_tree(xml_file_content, "Default Content Type")
                .context("Decoding Content Type Failed")?;
        Ok(Self { xml_document })
    }
    pub(crate) fn get_extensions(&mut self) -> AnyResult<Option<Vec<(String, String)>>, AnyError> {
        let mut elements: Vec<(String, String)> = Vec::new();
        if let Some(default_elements) = self.xml_document.pop_elements_by_tag_mut("Default", None) {
            for default_element in default_elements {
                let attributes = default_element
                    .get_attribute()
                    .ok_or(anyhow!("Element Attribute not Found"))?;
                elements.push((
                    attributes
                        .get("Extension")
                        .ok_or(anyhow!("content type default attribute missing"))?
                        .to_string(),
                    attributes
                        .get("ContentType")
                        .ok_or(anyhow!("content type default attribute missing"))?
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
    ) -> AnyResult<Option<String>, AnyError> {
        if let Some(mut find_ids) = self.xml_document.get_element_ids_by_attribute(
            "PartName",
            &format!("/{}", file_name),
            None,
        ) {
            if let Some(id) = find_ids.pop() {
                if let Some(element) = self.xml_document.pop_element_mut(&id) {
                    if let Some(attributes) = element.get_attribute() {
                        let res = attributes.get("ContentType").unwrap().to_string();
                        return Ok(Some(res));
                    }
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn create_xml_file(
        extensions: Vec<(String, String)>,
        overrides: Vec<(String, String)>,
    ) -> AnyResult<Vec<u8>, AnyError> {
        let mut document = XmlDocument::new();
        let root_element = document
            .create_root_mut("Types")
            .context("Failed to Create Root Element")?;
        let mut attributes = HashMap::new();
        attributes.insert(
            "xmlns".to_string(),
            COMMON_TYPE_COLLECTION
                .get("content_type")
                .unwrap()
                .schemas_namespace
                .to_string(),
        );
        root_element
            .set_attribute_mut(attributes)
            .context("Set Attributes on root element failed")?;
        // Load Default Elements
        {
            for (extension, content_type) in extensions {
                let element = document
                    .append_child_mut("Default", None)
                    .context("Append child to root failed")?;
                let mut attributes = HashMap::new();
                attributes.insert("Extension".to_string(), extension);
                attributes.insert("ContentType".to_string(), content_type);
                element
                    .set_attribute_mut(attributes)
                    .context("Adding attributes to Default element Failed")?;
            }
        }
        // Load Override Elements
        {
            for (part_name, content_type) in overrides {
                let element = document
                    .append_child_mut("Override", None)
                    .context("Append child to root failed")?;
                let mut attributes = HashMap::new();
                attributes.insert("PartName".to_string(), part_name);
                attributes.insert("ContentType".to_string(), content_type);
                element
                    .set_attribute_mut(attributes)
                    .context("Adding attributes to Default element Failed")?;
            }
        }
        XmlSerializer::xml_tree_to_vec(&mut document, "Create Content Type")
    }
}
