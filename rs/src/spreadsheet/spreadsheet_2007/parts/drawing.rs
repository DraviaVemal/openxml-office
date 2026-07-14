use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlDeserializer, XmlDocument, XmlElementContentType};
use phf::Map;

use crate::{
    element_dictionary::{Content, EXCEL_TYPE_COLLECTION},
    files::OfficeDocument,
    global_2007::{
        parts::{DrawingPartGlobal, RelationsPart},
        traits::{XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing},
    },
    log_elapsed,
    spreadsheet_2007::{models::DrawingAnchor, services::CommonServices},
};

#[derive(Debug)]
pub(crate) struct DrawingPart {
    drawing_global: DrawingPartGlobal,
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    common_service: Weak<RefCell<CommonServices>>,
    worksheet_relationship_part: Weak<RefCell<RelationsPart>>,
    drawing_relationship_part: Rc<RefCell<RelationsPart>>,
    anchor_collection: Option<VecDeque<DrawingAnchor>>,
    is_new: bool,
    file_path: String,
}

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
                            .context("Failed to pull worksheet relationship handle")?
                            .delete_relationship_mut(&self.file_path);
                    }
                    if let Some(office_document) = self.office_document.upgrade() {
                        office_document
                            .try_borrow_mut()
                            .context("Failed to pull office document")?
                            .delete_document_mut(&self.file_path);
                    }
                    self.drawing_relationship_part
                        .try_borrow_mut()
                        .context("Failed to pull relationship handle")?
                        .close_document()
                        .context("Failed to Close relationship part")?;
                    return Ok(());
                }
                if let Some(office_document) = self.office_document.upgrade() {
                    let mut office_doc_mut = office_document
                        .try_borrow_mut()
                        .context("Failed to pull office document")?;
                    log_elapsed!(
                        || {
                            office_doc_mut
                                .close_xml_document(&self.file_path)
                                .context("Failed to close the current tree document")
                        },
                        "Close Drawing document"
                    )?;
                }
                log_elapsed!(
                    || {
                        self.drawing_relationship_part
                            .try_borrow_mut()
                            .context("Failed to pull relationship handle")?
                            .close_document()
                            .context("Failed to Close relationship part")
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
        let content = EXCEL_TYPE_COLLECTION.get("drawing").unwrap();
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <xdr:wsDr xmlns:xdr="http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing">
            </xdr:wsDr>"#;
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(template_core_properties.as_bytes().to_vec())
                .context("Initializing Drawing part Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

impl DrawingPart {
    pub(crate) fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        worksheet_relationship_part: Weak<RefCell<RelationsPart>>,
        common_service: Weak<RefCell<CommonServices>>,
        type_collection: &Map<&'static str, &'static Content>,
    ) -> AnyResult<DrawingPart, AnyError> {
        let file_path = Self::get_drawing_file_name(&worksheet_relationship_part, type_collection)
            .context("Failed to pull worksheet file name")?;
        // Detect whether this drawing already exists (loaded) before the xml document
        // handle is created, since fetching the handle moves it out of the archive.
        let is_new = if let Some(office_document) = office_document.upgrade() {
            !office_document
                .try_borrow()
                .context("Failed to borrow office document")?
                .check_file_exist(file_path.clone())
        } else {
            true
        };
        let xml_document = Self::get_xml_document(&office_document, &file_path)?;
        let drawing_relationship_part = Rc::new(RefCell::new(
            RelationsPart::new(
                office_document.clone(),
                &format!(
                    "{}/_rels/{}.rels",
                    &file_path[..file_path.rfind('/').unwrap()],
                    file_path.rsplit('/').next().unwrap()
                ),
            )
            .context("Creating Relation ship part for workbook failed.")?,
        ));
        Ok(Self {
            drawing_global: DrawingPartGlobal::new(),
            office_document,
            xml_document,
            common_service,
            worksheet_relationship_part,
            drawing_relationship_part,
            anchor_collection: None,
            is_new,
            file_path: file_path.to_string(),
        })
    }

    /// A drawing is considered empty when it was newly created this session and has
    /// not received any anchor/content. Loaded drawings are never treated as empty so
    /// they always round-trip.
    fn is_empty(&self) -> bool {
        self.is_new
            && self
                .anchor_collection
                .as_ref()
                .map(|anchors| anchors.is_empty())
                .unwrap_or(true)
    }

    fn get_drawing_file_name(
        worksheet_relationship_part: &Weak<RefCell<RelationsPart>>,
        type_collection: &Map<&'static str, &'static Content>,
    ) -> AnyResult<String, AnyError> {
        let drawing_content = type_collection.get("drawing").unwrap();
        if let Some(worksheet_relationship_part) = worksheet_relationship_part.upgrade() {
            Ok(worksheet_relationship_part
                .try_borrow_mut()
                .context("Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &drawing_content.schemas_type,
                    drawing_content,
                    Some(format!("xl/{}", drawing_content.default_path)),
                    None,
                )
                .context("Pull Path From Existing File Failed")?)
        } else {
            Err(anyhow!("Failed to upgrade relation part"))
        }
    }

    fn deserialize_drawing(
        xml_document: &Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Option<VecDeque<DrawingAnchor>>, AnyError> {
        if let Some(xml_document) = xml_document.upgrade() {
            let xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("Failed to get XML doc handle")?;
            if let Ok(root_element) = xml_doc_mut.get_element(xml_doc_mut.get_root_id()) {
                let anchor_collection = VecDeque::new();
                let child_list: Vec<(NodeId, String)> = root_element
                    .get_child_contents()
                    .as_ref()
                    .map(|contents| {
                        contents
                            .iter()
                            .filter_map(|content| match content {
                                XmlElementContentType::Element((id, tag, _)) => {
                                    Some((*id, tag.clone()))
                                }
                                _ => None,
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                for (anchor_element_id, anchor_element_tag) in child_list {
                    // FIXME: Complete the feature to deserialize the drawing anchor elements into DrawingAnchor structs
                    let _anchor_element = xml_doc_mut
                        .get_element(anchor_element_id)
                        .context("Failed to locate Child element")?;
                    match anchor_element_tag.as_str() {
                        "absoluteAnchor" => {}
                        "oneCellAnchor" => {}
                        "twoCellAnchor" => {}
                        _ => {
                            return Err(anyhow!(
                                "Unhandled Drawing Component Detected. '{}'",
                                anchor_element_tag
                            ));
                        }
                    }
                }
                Ok(Some(anchor_collection))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
