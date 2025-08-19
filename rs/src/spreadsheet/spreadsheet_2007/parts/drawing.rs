use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument};

use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
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
        let content = EXCEL_TYPE_COLLECTION.get("worksheet").unwrap();
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
    pub(crate) fn new_worksheet(
        office_document: Weak<RefCell<OfficeDocument>>,
        worksheet_relationship_part: Weak<RefCell<RelationsPart>>,
        common_service: Weak<RefCell<CommonServices>>,
    ) -> AnyResult<DrawingPart, AnyError> {
        let file_path = Self::get_drawing_file_name(&worksheet_relationship_part)
            .context("Failed to pull worksheet file name")?;
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
        // let anchor_collection = log_elapsed!(
        //     || { Self::deserialize_drawing(&xml_document).context("Failed to open Worksheet") },
        //     "Worksheet Initialize Time"
        // )?;
        Ok(Self {
            drawing_global: DrawingPartGlobal::new(),
            office_document,
            xml_document,
            common_service,
            worksheet_relationship_part,
            drawing_relationship_part,
            anchor_collection: None,
            file_path: file_path.to_string(),
        })
    }

    fn get_drawing_file_name(
        worksheet_relationship_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let drawing_content = EXCEL_TYPE_COLLECTION.get("drawing").unwrap();
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
            let root_id = xml_doc_mut.get_root_id();
            let root_element = xml_doc_mut
                .get_element(root_id)
                .context("Failed to get root element");
            let anchor_collection = VecDeque::new();

            Ok(Some(anchor_collection))
        } else {
            Ok(None)
        }
    }
}
