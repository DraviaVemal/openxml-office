use crate::{
    files::{XmlDocument, XmlElement},
    log_elapsed,
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use quick_xml::escape::escape;

pub struct XmlSerializer {}

impl XmlSerializer {
    pub(crate) fn xml_tree_to_vec(
        xml_document: &mut XmlDocument,
        file_name: &str,
    ) -> AnyResult<Vec<u8>, AnyError> {
        let mut xml_content = String::new();
        #[cfg(debug_assertions)]
        {
            xml_content.push_str(r#"<?xml version="1.0" encoding="utf-8"?>"#);
        }
        #[cfg(not(debug_assertions))]
        {
            use chrono::Utc;
            xml_content.push_str(r#"<?xml version="1.0" encoding="utf-8"?>"#);
            xml_content.push_str(
                format!(r#"<!--<dvmo:office><dvmo:appName>{}</dvmo:appName><dvmo:repo>{}</dvmo:repo><dvmo:version>{}</dvmo:version><dvmo:modified>{}</dvmo:modified></dvmo:office>-->"#,
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_REPOSITORY"),
                    env!("CARGO_PKG_VERSION"),
                    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                )
                .as_str(),);
        }
        log_elapsed!(
            || {
                Self::build_xml_tree(xml_document, &mut xml_content)
                    .context("Create XML Contact String Failed")
            },
            format!("Deserialize File : {}", file_name)
        )?;
        Ok(xml_content.as_bytes().to_vec())
    }

    fn build_xml_tree(
        xml_document: &mut XmlDocument,
        master_string: &mut String,
    ) -> AnyResult<(), AnyError> {
        let max_count = xml_document.get_element_count() * 2;
        let mut current_count = 0;
        if let Some(xml_root) = xml_document.get_root() {
            if xml_root.is_empty_tag() {
                master_string.push_str(Self::generate_xml_element(xml_root, true, true).as_str());
            } else {
                master_string.push_str(Self::generate_xml_element(xml_root, false, true).as_str());
                if let Some(value) = xml_root.get_value() {
                    master_string.push_str(&Self::generate_xml_value_close(value, xml_root));
                } else {
                    let mut parent_id = 0;
                    loop {
                        current_count += 1;
                        if current_count > max_count {
                            return Err(anyhow!("Loop Safety Error Triggered at XML Deserialized"));
                        }
                        if let Some(current_element) = xml_document.get_element(&parent_id) {
                            if let Some((current_id, element_tag)) = current_element.pop_child_mut()
                            {
                                // Pop Next Valid Child From the tree
                                if let Some(element) = xml_document.get_element(&current_id) {
                                    if element.is_empty_tag() {
                                        master_string.push_str(
                                            Self::generate_xml_element(element, true, false)
                                                .as_str(),
                                        );
                                    } else {
                                        master_string.push_str(
                                            Self::generate_xml_element(element, false, false)
                                                .as_str(),
                                        );
                                        if let Some(value) = element.get_value() {
                                            master_string.push_str(
                                                &Self::generate_xml_value_close(value, element),
                                            );
                                        } else {
                                            parent_id = current_id
                                        }
                                    }
                                } else {
                                    return Err(anyhow!(
                                        "Un know element ID is Pull failed : {}",
                                        element_tag
                                    ));
                                }
                            } else {
                                if parent_id == 0 {
                                    break;
                                } else {
                                    // Travel Up as there is no active child to continue
                                    if let Some(element) = xml_document.get_element(&parent_id) {
                                        master_string
                                            .push_str(&Self::generate_xml_value_close("", element));
                                        parent_id = element.get_parent_id()
                                    }
                                }
                            }
                        } else {
                            return Err(anyhow!("Un know Element pull failed"));
                        }
                    }
                    master_string.push_str(&Self::generate_xml_value_close("", xml_root));
                }
            }
        }
        Ok(())
    }

    fn generate_xml_element(xml_element: &XmlElement, close: bool, root_element: bool) -> String {
        let mut element_tag = format!("<{}", xml_element.get_tag());
        if root_element {
            #[allow(unused_mut)]
            if let Some(mut namespace) = xml_element.get_namespace() {
                #[cfg(not(debug_assertions))]
                {
                    namespace.insert(
                        "dvmo".to_string(),
                        "http://schemas.draviavemal.com/openxml-office".to_string(),
                    );
                }
                element_tag.push_str(
                    format!(
                        " {}",
                        namespace
                            .iter()
                            .map(|(key, value)| {
                                if key == "<Default>" {
                                    format!("xmlns=\"{}\"", value)
                                } else {
                                    format!("xmlns:{}=\"{}\"", key, value)
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" ")
                    )
                    .as_str(),
                );
            }
        }
        if let Some(attributes) = xml_element.get_attribute() {
            let mut keys = attributes.keys().cloned().collect::<Vec<String>>();
            keys.sort();
            element_tag.push_str(
                format!(
                    " {}",
                    keys.iter()
                        .map(|key| format!("{}=\"{}\"", key, escape(attributes.get(key).unwrap())))
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
        format!("{}</{}>", escape(value), xml_element.get_tag())
    }
}
