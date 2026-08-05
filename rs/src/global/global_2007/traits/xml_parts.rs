#![allow(private_bounds)]

use crate::files::OfficeDocument;
use crate::global_2007::parts::RelationsPart;
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::XmlDocument;
use std::{cell::RefCell, rc::Weak};

pub(crate) trait XmlDocumentPartClose {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized;
}

pub trait XmlDocumentPartFlush: XmlDocumentPartClose {
    /// Save the current object state and flush to file from memory
    fn flush(mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        self.close_document()
    }
}

pub(crate) trait XmlDocumentPartInitializing:
    XmlDocumentPartClose + XmlDocumentPartFlush
{
    /// Get content of the current xml
    fn get_xml_document(
        office_document: &Weak<RefCell<OfficeDocument>>,
        file_name: &str,
    ) -> AnyResult<Weak<RefCell<XmlDocument>>, AnyError> {
        let (xml_document, content_type, file_extension, extension_type) =
            if let Some((xml_document, content_type, file_extension, extension_type)) =
                office_document
                    .upgrade()
                    .context("draviavemal-openxml_office::Document Upgrade Handled Failed")?
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to borrow handle")?
                    .get_xml_tree_mut(file_name)
                    .context(format!(
                        "draviavemal-openxml_office::XML Tree Parsing Failed for File : {}",
                        file_name
                    ))?
            {
                (xml_document, content_type, file_extension, extension_type)
            } else {
                Self::initialize_content_xml()
                    .context("draviavemal-openxml_office::Initial XML element parsing failed")?
            };
        office_document
            .upgrade()
            .context("draviavemal-openxml_office::Document Upgrade Handled Failed")?
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Getting XML Tree Handle Failed")?
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

// ######################### Train implementation of XML Part - Only accessible within crate ##############
#[warn(drop_bounds)]
pub(crate) trait XmlDocumentPart:
    XmlDocumentPartInitializing + XmlDocumentPartClose
{
    /// Create new object with file connector handle
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<Self, AnyError>
    where
        Self: Sized;
}
