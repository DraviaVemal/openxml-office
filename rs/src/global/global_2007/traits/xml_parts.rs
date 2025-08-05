use crate::files::{OfficeDocument, XmlDocument};
use crate::global_2007::parts::RelationsPart;
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, rc::Weak};

pub trait XmlDocumentPartCommon {
    /// Save the current file state
    fn flush(mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        self.close_document()
    }

    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized;
    /// Get content of the current xml
    fn get_xml_document(
        office_document: &Weak<RefCell<OfficeDocument>>,
        file_name: &str,
    ) -> AnyResult<Weak<RefCell<XmlDocument>>, AnyError> {
        let (xml_document, content_type, file_extension, extension_type) =
            if let Some((xml_document, content_type, file_extension, extension_type)) =
                office_document
                    .upgrade()
                    .ok_or(anyhow!("Document Upgrade Handled Failed"))
                    .context("XML Document Read Failed")?
                    .try_borrow_mut()
                    .context("Failed to borrow handle")?
                    .get_xml_tree_mut(file_name)
                    .context(format!("XML Tree Parsing Failed for File : {}", file_name))?
            {
                (xml_document, content_type, file_extension, extension_type)
            } else {
                Self::initialize_content_xml().context("Initial XML element parsing failed")?
            };
        office_document
            .upgrade()
            .ok_or(anyhow!("Document Upgrade Handled Failed"))
            .context("XML Document Read Failed")?
            .try_borrow_mut()
            .context("Getting XML Tree Handle Failed")?
            .get_xml_document_ref(
                file_name,
                content_type,
                file_extension,
                extension_type,
                xml_document,
            )
    }
    /// Initialize the content if not already exist . // File Content , Content Type, File Extension, Extension Type
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>;
}

#[warn(drop_bounds)]
pub(crate) trait XmlDocumentPart: XmlDocumentPartCommon {
    /// Create new object with file connector handle
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<Self, AnyError>
    where
        Self: Sized;
}
