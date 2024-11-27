use crate::files::{XmlDocument, XmlElement};
use anyhow::{Context, Error as AnyError, Result as AnyResult};

pub struct XmlDeSerializer {}

impl XmlDeSerializer {
    pub fn xml_tree_to_vec(xml_document: &XmlDocument) -> AnyResult<Vec<u8>, AnyError> {
        let mut xml_content = String::new();
        xml_content.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        Self::build_xml_tree(xml_document, &mut xml_content)
            .context("Create XML Contact String Failed")?;
        Ok(xml_content.as_bytes().to_vec())
    }

    fn build_xml_tree(
        xml_document: &XmlDocument,
        master_string: &mut String,
    ) -> AnyResult<(), AnyError> {
        if let Some(xml_root) = xml_document.get_root() {
            if xml_root.is_empty_tag() {
                master_string.push_str(Self::generate_xml_element(xml_root, true).as_str());
            } else {
                master_string.push_str(Self::generate_xml_element(xml_root, false).as_str());
                if let Some(value) = xml_root.get_value() {
                    master_string.push_str(&Self::generate_xml_value_close(value, xml_root));
                } else {
                    let mut parent_id = 0;
                    loop {
                        if let Some(current_element) = xml_document.get_element(&parent_id) {
                            if let Some(current_id) = current_element.pop_child_id_mut() {
                                // Pop Next Valid Child From the tree
                                if let Some(element) = xml_document.get_element(&current_id) {
                                    if element.is_empty_tag() {
                                        master_string.push_str(
                                            Self::generate_xml_element(element, true).as_str(),
                                            
                                        );
                                    } else {
                                        master_string.push_str(
                                            Self::generate_xml_element(element, false).as_str(),
                                        );
                                        if let Some(value) = element.get_value() {
                                            master_string.push_str(
                                                &Self::generate_xml_value_close(value, element),
                                            );
                                        } else {
                                            parent_id = current_id
                                        }
                                    }
                                }
                            } else {
                                if parent_id == 0 {
                                    break;
                                }
                            }
                        } else {
                            if parent_id == 0 {
                                break;
                            }
                            // Travel Up as there is no active child to continue
                            if let Some(element) = xml_document.get_element(&parent_id) {
                                master_string
                                    .push_str(&Self::generate_xml_value_close("", element));
                                parent_id = element.get_parent_id()
                            }
                        }
                    }
                    master_string.push_str(&Self::generate_xml_value_close("", xml_root));
                }
            }
        }
        Ok(())
    }

    fn generate_xml_element(xml_element: &XmlElement, close: bool) -> String {
        let mut element_tag = format!("<{}", xml_element.get_tag());
        if let Some(attributes) = xml_element.get_attribute() {
            element_tag.push_str(
                format!(
                    " {}",
                    attributes
                        .iter()
                        .map(|(key, value)| format!("{}=\"{}\"", key, value))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_str(),
            );
        }
        if close {
            element_tag.push_str(" />");
        } else {
            element_tag.push_str(" >");
        }
        element_tag
    }

    fn generate_xml_value_close(value: &str, xml_element: &XmlElement) -> String {
        format!("{}</{}>", value, xml_element.get_tag())
    }
}
