use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    rc::{Rc, Weak},
};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{
    NodeId, Tag, XmlAttribute, XmlDocument, XmlElement, XmlElementContentType,
};
use phf::Map;

use crate::{
    element_dictionary::{Content, EXCEL_TYPE_COLLECTION},
    files::OfficeDocument,
    global_2007::{
        models::{
            AnchorPosition, BlipFillMode, GraphPosition, Picture, RelativeRect, ShapeProperties,
            Transform,
        },
        parts::{DrawingPartGlobal, RelationsPart},
        traits::{Enum, XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing},
    },
    log_elapsed,
    namespaces::{CHART_NS, DRAWINGML_NS, RELATIONSHIP_OFFICE_DOC_NS, SPREADSHEET_DRAWING_NS},
    spreadsheet_2007::{
        models::{
            AbsoluteAnchor, AnchorContent, ConnectorShape, ContentPart, DrawingAnchor,
            EditAsValues, ExcelPictureSetting, GraphicFrame, GroupShape, OneCellAnchor, Shape,
            TwoCellAnchor,
        },
        services::CommonServices,
    },
};

mod deserialize;
mod edit;
mod internal;
mod model;
mod serialize;

pub(crate) use model::DrawingPart;

impl XmlDocumentPartFlush for DrawingPart {}

impl XmlDocumentPartClose for DrawingPart {
    fn close_document(&mut self) -> anyhow::Result<(), anyhow::Error>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if self.is_empty() {
                    if let Some(worksheet_relationship_part) =
                        self.worksheet_relationship_part.upgrade()
                    {
                        worksheet_relationship_part
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to pull worksheet relationship handle")?
                            .delete_relationship_mut(&self.file_path);
                    }
                    if let Some(office_document) = self.office_document.upgrade() {
                        office_document
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to pull office document")?
                            .delete_document_mut(&self.file_path);
                    }
                    self.drawing_relationship_part
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::Failed to pull relationship handle")?
                        .close_document()
                        .context("draviavemal-openxml_office::Failed to Close relationship part")?;
                    return Ok(());
                }
                if let Some(office_document) = self.office_document.upgrade() {
                    let mut office_doc_mut = office_document
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::Failed to pull office document")?;
                    if let Some(xml_document) = self.xml_document.upgrade() {
                        let mut xml_doc_mut = xml_document
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to pull XML handle")?;
                        log_elapsed!(self.serialize_drawing(&mut xml_doc_mut))?;
                    }
                    log_elapsed!(
                        || {
                            office_doc_mut
                                .close_xml_document(&self.file_path)
                                .context("draviavemal-openxml_office::Failed to close the current tree document")
                        },
                        "Close Drawing document"
                    )?;
                }
                log_elapsed!(
                    || {
                        self.drawing_relationship_part
                            .try_borrow_mut()
                            .context(
                                "draviavemal-openxml_office::Failed to pull relationship handle",
                            )?
                            .close_document()
                            .context(
                                "draviavemal-openxml_office::Failed to Close relationship part",
                            )
                    },
                    "Drawing relation part closed"
                )?;
                Ok(())
            },
            "Close Worksheet"
        )
    }
}

impl XmlDocumentPartInitializing for DrawingPart {
    fn initialize_content_xml(
    ) -> anyhow::Result<(XmlDocument, Option<String>, String, String), anyhow::Error> {
        let content = EXCEL_TYPE_COLLECTION
            .get("drawing")
            .context("Failed to read excel type collection")?;
        let mut template_core_properties = XmlDocument::new();
        template_core_properties
            .create_root_element_ns_mut("wsDr", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to create drawing root element")?;
        Ok((
            template_core_properties,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}
