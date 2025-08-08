use crate::files::XmlDocument;
use crate::log_elapsed;
use anyhow::{anyhow, Context, Error as AnyError, Ok, Result as AnyResult};
use quick_xml::events::BytesStart;
use quick_xml::{
    events::{
        attributes::{AttrError, Attribute},
        Event,
    },
    NsReader,
};
use std::{collections::HashMap, io::Cursor};

pub struct XmlDeSerializer {}

impl XmlDeSerializer {
    pub(crate) fn vec_to_xml_doc_tree(
        xml_str: Vec<u8>,
        file_name: &str,
    ) -> AnyResult<XmlDocument, AnyError> {
        let mut reader: NsReader<Cursor<Vec<u8>>> = NsReader::from_reader(Cursor::new(xml_str));
        let mut xml_document = XmlDocument::new();
        reader.config_mut().trim_text(true);
        log_elapsed!(
            || {
                Self::xml_element_parser(&mut reader, &mut xml_document)
                    .context("Xml Element Parser Failed")
            },
            format!("Serializing : {}", file_name)
        )?;
        Ok(xml_document)
    }

    fn xml_element_parser(
        reader: &mut NsReader<Cursor<Vec<u8>>>,
        xml_document: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        let mut temp_buffer: Vec<u8> = Vec::new();
        let mut root_loaded = false;
        let mut active_xml_element_id: usize = 0;
        loop {
            match reader.read_event_into(&mut temp_buffer) {
                // Handle errors
                Err(e) => break Err(e.into()),
                // Read XML declaration
                Result::Ok(Event::Decl(_)) => {}
                // Read start tag and attributes
                Result::Ok(Event::Start(element)) => {
                    let tag: String =
                        String::from_utf8_lossy(element.name().into_inner()).to_string();
                    let attributes = Self::get_attributes_string(element)?;
                    // Update tree structure
                    if root_loaded {
                        // Add child to parent
                        active_xml_element_id = xml_document
                            .append_child_mut(&tag, Some(&active_xml_element_id))
                            .context("Insert XML Child Failed.")?
                            .set_attribute_mut(attributes)
                            .context("Attribute Update Error")?
                            .get_id();
                    } else {
                        // Create the root element
                        active_xml_element_id = xml_document
                            .create_root_mut(&tag)
                            .context("Create XML Root Element Failed")?
                            .set_attribute_mut(attributes)
                            .context("Attribute Update Error")?
                            .get_id();
                        root_loaded = true
                    }
                }

                // Handle empty elements (self-closing tags)
                Result::Ok(Event::Empty(element)) => {
                    let tag: String =
                        String::from_utf8_lossy(element.name().into_inner()).to_string();
                    let attributes = Self::get_attributes_string(element)?;
                    xml_document
                        .append_child_mut(&tag, Some(&active_xml_element_id))
                        .context("Insert XML Child Failed.")?
                        .set_attribute_mut(attributes)
                        .context("Parser Attribute Validation Failed")?;
                }

                // Read text content
                Result::Ok(Event::Text(byte_text)) => {
                    let text = byte_text.unescape().context("XML Text parsing error")?;
                    xml_document
                        .get_element_mut(&active_xml_element_id)
                        .ok_or(anyhow!("Converting Option to Result Failed"))
                        .context("Getting Target Element Failed")?
                        .set_value_mut(text.to_string());
                }

                // Handle end tag
                Result::Ok(Event::End(element)) => {
                    // Pop the active element back to the parent
                    let tag: String =
                        String::from_utf8_lossy(element.name().into_inner()).to_string();
                    let element = xml_document
                        .get_element_mut(&active_xml_element_id)
                        .ok_or(anyhow!("Converting Option to Result Failed"))
                        .context("Getting Target Element Failed")?;
                    if element.get_tag() == tag {
                        active_xml_element_id = element.get_parent_id()
                    } else {
                        break Err(anyhow!("Invalid XML Tree Parsing Failed."));
                    }
                }

                // End of file
                Result::Ok(Event::Eof) => break Ok(()),

                // Default case for other events
                _ => {}
            }
            temp_buffer.clear();
        }
    }

    fn get_attributes_string(element: BytesStart) -> AnyResult<HashMap<String, String>, AnyError> {
        element
            .attributes()
            .map(|attribute_result: Result<Attribute<'_>, AttrError>| {
                let attribute: Attribute<'_> =
                    attribute_result.context("Failed to parse attribute")?;
                let key: String = String::from_utf8_lossy(attribute.key.into_inner()).to_string();
                let value: String = String::from_utf8_lossy(&attribute.value).to_string();
                Ok((key, value))
            })
            .collect::<AnyResult<HashMap<String, String>>>()
    }
}
