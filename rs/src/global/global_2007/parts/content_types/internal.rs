use crate::global_2007::parts::content_types::ContentTypesPart;
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::XmlDeserializer;

impl ContentTypesPart {
    pub(crate) fn new(xml_file_content: Vec<u8>) -> AnyResult<Self, AnyError> {
        let xml_document = XmlDeserializer::vec_to_xml_doc_tree(xml_file_content)
            .context("draviavemal-openxml_office::Decoding Content Type Failed")?;
        Ok(Self { xml_document })
    }
}
