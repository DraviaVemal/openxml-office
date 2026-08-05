use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    rc::{Rc, Weak},
};

use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{
    NodeId, Tag, XmlAttribute, XmlDeserializer, XmlDocument, XmlElement, XmlElementContentType,
};
use phf::Map;

use crate::{
    element_dictionary::{Content, EXCEL_TYPE_COLLECTION},
    files::OfficeDocument,
    global_2007::{
        models::{AnchorPosition, GraphPosition},
        parts::{DrawingPartGlobal, RelationsPart},
        traits::{XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing},
    },
    log_elapsed,
    spreadsheet_2007::{
        models::{
            AbsoluteAnchor, AnchorContent, ConnectorShape, ContentPart, DrawingAnchor,
            GraphicFrame, GroupShape, OneCellAnchor, Picture, Shape, TwoCellAnchor,
        },
        services::CommonServices,
    },
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
        let content = EXCEL_TYPE_COLLECTION.get("drawing").unwrap();
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <xdr:wsDr xmlns:xdr="http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing">
            </xdr:wsDr>"#;
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(template_core_properties.as_bytes().to_vec())
                .context("draviavemal-openxml_office::Initializing Drawing part Failed")?,
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
        let file_path =
            DrawingPart::get_drawing_file_name(&worksheet_relationship_part, type_collection)
                .context("draviavemal-openxml_office::Failed to pull worksheet file name")?;
        // Detect whether this drawing already exists (loaded) before the xml document
        // handle is created, since fetching the handle moves it out of the archive.
        let is_new = if let Some(office_document) = office_document.upgrade() {
            !office_document
                .try_borrow()
                .context("draviavemal-openxml_office::Failed to borrow office document")?
                .check_file_exist(file_path.clone())
        } else {
            true
        };
        let xml_document = DrawingPart::get_xml_document(&office_document, &file_path)?;
        let drawing_relationship_part = Rc::new(RefCell::new(
            RelationsPart::new(
                office_document.clone(),
                &format!(
                    "{}/_rels/{}.rels",
                    &file_path[..file_path.rfind('/').unwrap()],
                    file_path.rsplit('/').next().unwrap()
                ),
            )
            .context(
                "draviavemal-openxml_office::Creating Relation ship part for workbook failed.",
            )?,
        ));
        let anchor_collection = log_elapsed!(
            || {
                DrawingPart::deserialize_drawing(&xml_document)
                    .context("draviavemal-openxml_office::Failed to open drawing part and deserialize anchors")
            },
            "drawing Initialize Time"
        )?;
        Ok(DrawingPart {
            drawing_global: DrawingPartGlobal::new(),
            office_document,
            xml_document,
            common_service,
            worksheet_relationship_part,
            drawing_relationship_part,
            anchor_collection,
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
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &drawing_content.schemas_type,
                    drawing_content,
                    Some(format!("xl/{}", drawing_content.default_path)),
                    None,
                )
                .context("draviavemal-openxml_office::Pull Path From Existing File Failed")?)
        } else {
            Err(AnyError::msg("draviavemal-openxml_office::Failed to upgrade relation part"))
        }
    }

    fn deserialize_drawing(
        xml_document: &Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Option<VecDeque<DrawingAnchor>>, AnyError> {
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to get XML doc handle")?;
            if let Ok(root_element) = xml_doc_mut.get_element(xml_doc_mut.get_root_id()) {
                let mut anchor_collection: VecDeque<DrawingAnchor> = VecDeque::new();
                let child_list: Vec<(NodeId, Tag)> = root_element
                    .get_child_contents()
                    .as_ref()
                    .map(|elements| {
                        elements
                            .iter()
                            .filter_map(|element| match element {
                                XmlElementContentType::Element((id, tag, _)) => {
                                    Some((*id, tag.clone()))
                                }
                                _ => None,
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                for (anchor_element_id, anchor_element_tag) in child_list {
                    match anchor_element_tag.as_str() {
                        "absoluteAnchor" => {
                            let absolute_anchor_element =
                                xml_doc_mut.get_element(anchor_element_id).context(
                                    "draviavemal-openxml_office::Failed to locate Child element",
                                )?;
                            let absolute_anchor = DrawingPart::deserialise_absolute_anchor(
                                &xml_doc_mut,
                                absolute_anchor_element,
                            )
                            .context(
                                "draviavemal-openxml_office::Failed to deserialise absolute anchor",
                            )?;
                            anchor_collection
                                .push_back(DrawingAnchor::AbsoluteAnchor(absolute_anchor));
                            xml_doc_mut.remove_element_mut(anchor_element_id).context(
                                "draviavemal-openxml_office::Failed to clean up absolute anchor",
                            )?;
                        }
                        "oneCellAnchor" => {
                            let one_cell_anchor_element =
                                xml_doc_mut.get_element(anchor_element_id).context(
                                    "draviavemal-openxml_office::Failed to locate Child element",
                                )?;
                            let one_cell_anchor = DrawingPart::deserialise_one_cell_anchor(
                                &xml_doc_mut,
                                one_cell_anchor_element,
                            )
                            .context(
                                "draviavemal-openxml_office::Failed to deserialise one cell anchor",
                            )?;
                            anchor_collection
                                .push_back(DrawingAnchor::OneCellAnchor(one_cell_anchor));
                            xml_doc_mut.remove_element_mut(anchor_element_id).context(
                                "draviavemal-openxml_office::Failed to clean up one cell anchor",
                            )?;
                        }
                        "twoCellAnchor" => {
                            let two_cell_anchor_element =
                                xml_doc_mut.get_element(anchor_element_id).context(
                                    "draviavemal-openxml_office::Failed to locate Child element",
                                )?;
                            let two_cell_anchor = DrawingPart::deserialise_two_cell_anchor(
                                &xml_doc_mut,
                                two_cell_anchor_element,
                            )
                            .context(
                                "draviavemal-openxml_office::Failed to deserialise two cell anchor",
                            )?;
                            anchor_collection
                                .push_back(DrawingAnchor::TwoCellAnchor(two_cell_anchor));
                            xml_doc_mut.remove_element_mut(anchor_element_id).context(
                                "draviavemal-openxml_office::Failed to clean up two cell anchor",
                            )?;
                        }
                        _ => {
                            return Err(AnyError::msg(format!(
                                "draviavemal-openxml_office::Unhandled Drawing Component Detected. '{}'",
                                anchor_element_tag
                            )));
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

    fn deserialise_two_cell_anchor(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        anchor_element: &XmlElement,
    ) -> AnyResult<TwoCellAnchor, AnyError> {
        let mut two_cell_anchor = TwoCellAnchor::default();
        let elements = anchor_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open Elemenet content type vec")?;
        for element in elements {
            match element {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "from" => {
                        let from_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        two_cell_anchor.from =
                            DrawingPart::deserialise_anchor_position(xml_doc_mut, from_element)
                                .context(
                                    "draviavemal-openxml_office::failed to parse from position",
                                )?;
                    }
                    "to" => {
                        let to_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        two_cell_anchor.to =
                            DrawingPart::deserialise_anchor_position(xml_doc_mut, to_element)
                                .context(
                                    "draviavemal-openxml_office::failed to parse to position",
                                )?;
                    }
                    // One of below type should exist
                    "sp" => {
                        two_cell_anchor.anchor_content = AnchorContent::Shape(Shape {});
                    }
                    "grpSp" => {
                        two_cell_anchor.anchor_content = AnchorContent::GroupShape(GroupShape {});
                    }
                    "graphicFrame" => {
                        let graphic_frame_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        two_cell_anchor.anchor_content = AnchorContent::GraphicFrame(
                            DrawingPart::deserialise_graphic_frame(
                                xml_doc_mut,
                                graphic_frame_element,
                            )
                            .context("draviavemal-openxml_office::failed to parse graphic frame")?,
                        );
                    }
                    "cxnSp" => {
                        two_cell_anchor.anchor_content =
                            AnchorContent::ConnectorShape(ConnectorShape {});
                    }
                    "pic" => {
                        let picture_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        two_cell_anchor.anchor_content = AnchorContent::Picture(
                            DrawingPart::deserialise_picture(xml_doc_mut, picture_element)
                                .context("draviavemal-openxml_office::failed to parse picture")?,
                        );
                    }
                    "contentPart" => {
                        two_cell_anchor.anchor_content = AnchorContent::ContentPart(ContentPart {});
                    }
                    // Mandatory node with optional attributes
                    "clientData" => {}
                    _ => {
                        log::error!("Unhandled Drawing Component Detected. '{}'", tag);
                    }
                },
                _ => {
                    log::warn!("Unhandled element content type found in drawing deserialisation. Ignoring will have data loss in end result")
                }
            }
        }
        Ok(two_cell_anchor)
    }

    fn deserialise_one_cell_anchor(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        anchor_element: &XmlElement,
    ) -> AnyResult<OneCellAnchor, AnyError> {
        let mut one_cell_anchor = OneCellAnchor {
            from: AnchorPosition::default(),
            anchor_content: AnchorContent::default(),
        };
        let elements = anchor_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open Elemenet content type vec")?;
        for element in elements {
            match element {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "from" => {
                        let from_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        one_cell_anchor.from =
                            DrawingPart::deserialise_anchor_position(xml_doc_mut, from_element)
                                .context(
                                    "draviavemal-openxml_office::failed to parse from position",
                                )?;
                    }
                    "ext" => {}
                    // One of below type should exist
                    "sp" => {
                        one_cell_anchor.anchor_content = AnchorContent::Shape(Shape {});
                    }
                    "grpSp" => {
                        one_cell_anchor.anchor_content = AnchorContent::GroupShape(GroupShape {});
                    }
                    "graphicFrame" => {
                        let graphic_frame_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        one_cell_anchor.anchor_content = AnchorContent::GraphicFrame(
                            DrawingPart::deserialise_graphic_frame(
                                xml_doc_mut,
                                graphic_frame_element,
                            )
                            .context("draviavemal-openxml_office::failed to parse graphic frame")?,
                        );
                    }
                    "cxnSp" => {
                        one_cell_anchor.anchor_content =
                            AnchorContent::ConnectorShape(ConnectorShape {});
                    }
                    "pic" => {
                        let picture_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        one_cell_anchor.anchor_content = AnchorContent::Picture(
                            DrawingPart::deserialise_picture(xml_doc_mut, picture_element)
                                .context("draviavemal-openxml_office::failed to parse picture")?,
                        );
                    }
                    "contentPart" => {
                        one_cell_anchor.anchor_content = AnchorContent::ContentPart(ContentPart {});
                    }
                    // Mandatory node with optional attributes
                    "clientData" => {}
                    _ => {
                        log::error!("Unhandled Drawing Component Detected. '{}'", tag);
                    }
                },
                _ => {
                    log::warn!("Unhandled element content type found in drawing deserialisation. Ignoring will have data loss in end result")
                }
            }
        }
        Ok(one_cell_anchor)
    }

    fn deserialise_absolute_anchor(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        anchor_element: &XmlElement,
    ) -> AnyResult<AbsoluteAnchor, AnyError> {
        let mut absolute_anchor = AbsoluteAnchor {
            pos: GraphPosition::default(),
            anchor_content: AnchorContent::default(),
        };
        let elements = anchor_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open Elemenet content type vec")?;
        for element in elements {
            match element {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "pos" => {}
                    "ext" => {}
                    // One of below type should exist
                    "sp" => {
                        absolute_anchor.anchor_content = AnchorContent::Shape(Shape {});
                    }
                    "grpSp" => {
                        absolute_anchor.anchor_content = AnchorContent::GroupShape(GroupShape {});
                    }
                    "graphicFrame" => {
                        let graphic_frame_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        absolute_anchor.anchor_content = AnchorContent::GraphicFrame(
                            DrawingPart::deserialise_graphic_frame(
                                xml_doc_mut,
                                graphic_frame_element,
                            )
                            .context("draviavemal-openxml_office::failed to parse graphic frame")?,
                        );
                    }
                    "cxnSp" => {
                        absolute_anchor.anchor_content =
                            AnchorContent::ConnectorShape(ConnectorShape {});
                    }
                    "pic" => {
                        let picture_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to locate Child element",
                        )?;
                        absolute_anchor.anchor_content = AnchorContent::Picture(
                            DrawingPart::deserialise_picture(xml_doc_mut, picture_element)
                                .context("draviavemal-openxml_office::failed to parse picture")?,
                        );
                    }
                    "contentPart" => {
                        absolute_anchor.anchor_content = AnchorContent::ContentPart(ContentPart {});
                    }
                    // Mandatory node with optional attributes
                    "clientData" => {}
                    _ => {
                        log::error!("Unhandled Drawing Component Detected. '{}'", tag);
                    }
                },
                _ => {
                    log::warn!("Unhandled element content type found in drawing deserialisation. Ignoring will have data loss in end result")
                }
            }
        }
        Ok(absolute_anchor)
    }

    fn deserialise_picture(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        picture_element: &XmlElement,
    ) -> AnyResult<Picture, AnyError> {
        let mut picture = Picture::default();
        let picture_children = picture_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open picture content vec")?;
        for picture_child in picture_children {
            match picture_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "nvPicPr" => {
                        let non_visual_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get non visual picture element",
                        )?;
                        DrawingPart::deserialise_picture_non_visual(
                            xml_doc_mut,
                            non_visual_element,
                            &mut picture,
                        )
                        .context("draviavemal-openxml_office::Failed to parse non visual picture properties")?;
                    }
                    "blipFill" => {
                        let blip_fill_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get blip fill element",
                        )?;
                        DrawingPart::deserialise_blip_fill(
                            xml_doc_mut,
                            blip_fill_element,
                            &mut picture,
                        )
                        .context("draviavemal-openxml_office::Failed to parse blip fill")?;
                    }
                    _ => {
                        log::warn!("Unsupported picture node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!(
                        "Unsupported picture content is not deserialised and will be lost on save"
                    );
                }
            }
        }
        Ok(picture)
    }

    fn deserialise_picture_non_visual(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        non_visual_element: &XmlElement,
        picture: &mut Picture,
    ) -> AnyResult<(), AnyError> {
        let non_visual_children = non_visual_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open non visual picture content vec")?;
        for non_visual_child in non_visual_children {
            match non_visual_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "cNvPr" => {
                        let properties_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get picture properties element",
                        )?;
                        if let Some(picture_id) = properties_element.get_attribute("id") {
                            picture.id = picture_id.get_value().parse().context(
                                "draviavemal-openxml_office::Failed to parse picture id",
                            )?;
                        }
                        if let Some(picture_name) = properties_element.get_attribute("name") {
                            picture.name = picture_name.get_value().to_string();
                        }
                    }
                    "cNvPicPr" => {
                        let non_visual_picture_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get non visual picture properties element")?;
                        DrawingPart::deserialise_picture_locks(
                            xml_doc_mut,
                            non_visual_picture_element,
                            picture,
                        )
                        .context("draviavemal-openxml_office::Failed to parse picture locks")?;
                    }
                    _ => {
                        log::warn!("Unsupported picture node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!(
                        "Unsupported picture content is not deserialised and will be lost on save"
                    );
                }
            }
        }
        Ok(())
    }

    fn deserialise_picture_locks(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        non_visual_picture_element: &XmlElement,
        picture: &mut Picture,
    ) -> AnyResult<(), AnyError> {
        let non_visual_picture_children = non_visual_picture_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open non visual picture properties content vec")?;
        for non_visual_picture_child in non_visual_picture_children {
            match non_visual_picture_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "picLocks" => {
                        let picture_locks_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get picture locks element",
                        )?;
                        if let Some(aspect_ratio) =
                            picture_locks_element.get_attribute("noChangeAspect")
                        {
                            picture.aspect_ratio = aspect_ratio.get_value() == "1";
                        }
                    }
                    _ => {
                        log::warn!("Unsupported picture node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!(
                        "Unsupported picture content is not deserialised and will be lost on save"
                    );
                }
            }
        }
        Ok(())
    }

    fn deserialise_blip_fill(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        blip_fill_element: &XmlElement,
        picture: &mut Picture,
    ) -> AnyResult<(), AnyError> {
        let blip_fill_children = blip_fill_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open blip fill content vec")?;
        for blip_fill_child in blip_fill_children {
            match blip_fill_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "blip" => {
                        let blip_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get blip element")?;
                        // Think to use more dynamic namespace parsing adoption so its stable with alias change
                        if let Some(relationship_id) = blip_element.get_attribute_ns("r:embed") {
                            picture.relationship_id = relationship_id.get_value().to_string();
                        }
                    }
                    _ => {
                        log::warn!("Unsupported picture node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!(
                        "Unsupported picture content is not deserialised and will be lost on save"
                    );
                }
            }
        }
        Ok(())
    }

    fn deserialise_graphic_frame(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        graphic_frame_element: &XmlElement,
    ) -> AnyResult<GraphicFrame, AnyError> {
        let mut graphic_frame = GraphicFrame::default();
        let graphic_frame_children = graphic_frame_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open graphic frame content vec")?;
        for graphic_frame_child in graphic_frame_children {
            match graphic_frame_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "nvGraphicFramePr" => {
                        let non_visual_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get non visual graphic frame element")?;
                        DrawingPart::deserialise_graphic_frame_non_visual(
                            xml_doc_mut,
                            non_visual_element,
                            &mut graphic_frame,
                        )
                        .context("draviavemal-openxml_office::Failed to parse non visual graphic frame properties")?;
                    }
                    "graphic" => {
                        let graphic_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get graphic element")?;
                        DrawingPart::deserialise_graphic(
                            xml_doc_mut,
                            graphic_element,
                            &mut graphic_frame,
                        )
                        .context("draviavemal-openxml_office::Failed to parse graphic")?;
                    }
                    _ => {
                        log::warn!("Unsupported graphic frame node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!("Unsupported graphic frame content is not deserialised and will be lost on save");
                }
            }
        }
        Ok(graphic_frame)
    }

    fn deserialise_graphic_frame_non_visual(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        non_visual_element: &XmlElement,
        graphic_frame: &mut GraphicFrame,
    ) -> AnyResult<(), AnyError> {
        let non_visual_children = non_visual_element.get_child_contents().as_ref().context(
            "draviavemal-openxml_office::Failed to open non visual graphic frame content vec",
        )?;
        for non_visual_child in non_visual_children {
            match non_visual_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "cNvPr" => {
                        let properties_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get graphic frame properties element")?;
                        if let Some(frame_id) = properties_element.get_attribute("id") {
                            graphic_frame.id = frame_id.get_value().parse().context(
                                "draviavemal-openxml_office::Failed to parse graphic frame id",
                            )?;
                        }
                        if let Some(frame_name) = properties_element.get_attribute("name") {
                            graphic_frame.name = frame_name.get_value().to_string();
                        }
                    }
                    _ => {
                        log::warn!("Unsupported graphic frame node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!("Unsupported graphic frame content is not deserialised and will be lost on save");
                }
            }
        }
        Ok(())
    }

    fn deserialise_graphic(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        graphic_element: &XmlElement,
        graphic_frame: &mut GraphicFrame,
    ) -> AnyResult<(), AnyError> {
        let graphic_children = graphic_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open graphic content vec")?;
        for graphic_child in graphic_children {
            match graphic_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "graphicData" => {
                        let graphic_data_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get graphic data element",
                        )?;
                        DrawingPart::deserialise_graphic_data(
                            xml_doc_mut,
                            graphic_data_element,
                            graphic_frame,
                        )
                        .context("draviavemal-openxml_office::Failed to parse graphic data")?;
                    }
                    _ => {
                        log::warn!("Unsupported graphic frame node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!("Unsupported graphic frame content is not deserialised and will be lost on save");
                }
            }
        }
        Ok(())
    }

    fn deserialise_graphic_data(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        graphic_data_element: &XmlElement,
        graphic_frame: &mut GraphicFrame,
    ) -> AnyResult<(), AnyError> {
        let graphic_data_children = graphic_data_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open graphic data content vec")?;
        for graphic_data_child in graphic_data_children {
            match graphic_data_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "chart" => {
                        let chart_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get chart element")?;
                        if let Some(chart_relationship_id) = chart_element.get_attribute_ns("r:id")
                        {
                            graphic_frame.relationship_id =
                                chart_relationship_id.get_value().to_string();
                        }
                    }
                    _ => {
                        log::warn!("Unsupported graphic frame node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!("Unsupported graphic frame content is not deserialised and will be lost on save");
                }
            }
        }
        Ok(())
    }

    fn deserialise_anchor_position(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        position_element: &XmlElement,
    ) -> AnyResult<AnchorPosition, AnyError> {
        let mut anchor_position = AnchorPosition::default();
        let elements = position_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open Elemenet content type vec")?;
        for element in elements {
            match element {
                XmlElementContentType::Element((element_id, tag, _)) => {
                    match tag.as_str() {
                        "col" => {
                            let col_element = xml_doc_mut
                                .get_element(*element_id)
                                .context("draviavemal-openxml_office::Failed to get element id")?;
                            let value = col_element
                            .get_element_text_value()
                            .context("draviavemal-openxml_office::Failed to get child value of the node")?
                            .context("draviavemal-openxml_office::No Valid Value found for col element")?;
                            anchor_position.column = value.trim().parse::<u16>().context(
                                "draviavemal-openxml_office::Failed to parse column value",
                            )?;
                        }
                        "colOff" => {
                            let col_offset_element = xml_doc_mut
                                .get_element(*element_id)
                                .context("draviavemal-openxml_office::Failed to get element id")?;
                            let value = col_offset_element
                            .get_element_text_value()
                            .context("draviavemal-openxml_office::Failed to get child value of the node")?
                            .context("draviavemal-openxml_office::No Valid Value found for col offset element")?;
                            anchor_position.column_offset = value.trim().parse::<u64>().context(
                                "draviavemal-openxml_office::Failed to parse column offset value",
                            )?;
                        }
                        "row" => {
                            let row_element = xml_doc_mut
                                .get_element(*element_id)
                                .context("draviavemal-openxml_office::Failed to get element id")?;
                            let value = row_element
                            .get_element_text_value()
                            .context("draviavemal-openxml_office::Failed to get child value of the node")?
                            .context("draviavemal-openxml_office::No Valid Value found for row element")?;
                            anchor_position.row = value
                                .trim()
                                .parse::<u32>()
                                .context("draviavemal-openxml_office::Failed to parse row value")?;
                        }
                        "rowOff" => {
                            let row_offset_element = xml_doc_mut
                                .get_element(*element_id)
                                .context("draviavemal-openxml_office::Failed to get element id")?;
                            let value = row_offset_element
                            .get_element_text_value()
                            .context("draviavemal-openxml_office::Failed to get child value of the node")?
                            .context("draviavemal-openxml_office::No Valid Value found for row offset element")?;
                            anchor_position.row_offset = value.trim().parse::<u64>().context(
                                "draviavemal-openxml_office::Failed to parse row offset value",
                            )?;
                        }
                        _ => {
                            log::error!("Unhandled Anchor position Component Detected. '{}'", tag);
                        }
                    }
                }
                _ => {
                    log::warn!("Unhandled Content Type Detected Ignoring. ");
                }
            }
        }
        Ok(anchor_position)
    }

    fn serialize_drawing(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        if let Some(mut anchor_collection) = self.anchor_collection.take() {
            let root_id = xml_doc_mut.get_root_id();
            while let Some(anchor) = anchor_collection.pop_front() {
                match anchor {
                    DrawingAnchor::TwoCellAnchor(two_cell_anchor) => {
                        DrawingPart::serialize_two_cell_anchor(
                            xml_doc_mut,
                            root_id,
                            two_cell_anchor,
                        )
                        .context(
                            "draviavemal-openxml_office::Failed to serialise two cell anchor",
                        )?;
                    }
                    DrawingAnchor::OneCellAnchor(one_cell_anchor) => {
                        DrawingPart::serialize_one_cell_anchor(
                            xml_doc_mut,
                            root_id,
                            one_cell_anchor,
                        )
                        .context(
                            "draviavemal-openxml_office::Failed to serialise one cell anchor",
                        )?;
                    }
                    DrawingAnchor::AbsoluteAnchor(absolute_anchor) => {
                        DrawingPart::serialize_absolute_anchor(
                            xml_doc_mut,
                            root_id,
                            absolute_anchor,
                        )
                        .context(
                            "draviavemal-openxml_office::Failed to serialise absolute anchor",
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    fn serialize_two_cell_anchor(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        two_cell_anchor: TwoCellAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_doc_mut
            .append_child_element_mut(parent_id, "xdr:twoCellAnchor", None)
            .context("draviavemal-openxml_office::Failed to add two cell anchor element")?;
        DrawingPart::serialize_anchor_position(
            xml_doc_mut,
            anchor_id,
            "xdr:from",
            &two_cell_anchor.from,
        )
        .context("draviavemal-openxml_office::Failed to serialise from position")?;
        DrawingPart::serialize_anchor_position(
            xml_doc_mut,
            anchor_id,
            "xdr:to",
            &two_cell_anchor.to,
        )
        .context("draviavemal-openxml_office::Failed to serialise to position")?;
        DrawingPart::serialize_anchor_content(
            xml_doc_mut,
            anchor_id,
            two_cell_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_doc_mut
            .append_child_element_mut(anchor_id, "xdr:clientData", None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_one_cell_anchor(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        one_cell_anchor: OneCellAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_doc_mut
            .append_child_element_mut(parent_id, "xdr:oneCellAnchor", None)
            .context("draviavemal-openxml_office::Failed to add one cell anchor element")?;
        DrawingPart::serialize_anchor_position(
            xml_doc_mut,
            anchor_id,
            "xdr:from",
            &one_cell_anchor.from,
        )
        .context("draviavemal-openxml_office::Failed to serialise from position")?;
        DrawingPart::serialize_anchor_content(
            xml_doc_mut,
            anchor_id,
            one_cell_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_doc_mut
            .append_child_element_mut(anchor_id, "xdr:clientData", None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_absolute_anchor(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        absolute_anchor: AbsoluteAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_doc_mut
            .append_child_element_mut(parent_id, "xdr:absoluteAnchor", None)
            .context("draviavemal-openxml_office::Failed to add absolute anchor element")?;
        DrawingPart::serialize_anchor_content(
            xml_doc_mut,
            anchor_id,
            absolute_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_doc_mut
            .append_child_element_mut(anchor_id, "xdr:clientData", None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_anchor_position(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        tag: &str,
        anchor_position: &AnchorPosition,
    ) -> AnyResult<(), AnyError> {
        let position_id = xml_doc_mut
            .append_child_element_mut(parent_id, tag, None)
            .context("draviavemal-openxml_office::Failed to add anchor position element")?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "xdr:col",
            &anchor_position.column.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "xdr:colOff",
            &anchor_position.column_offset.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "xdr:row",
            &anchor_position.row.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "xdr:rowOff",
            &anchor_position.row_offset.to_string(),
        )?;
        Ok(())
    }

    fn serialize_text_element(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        tag: &str,
        value: &str,
    ) -> AnyResult<(), AnyError> {
        let element_id = xml_doc_mut
            .append_child_element_mut(parent_id, tag, None)
            .context("draviavemal-openxml_office::Failed to add text element")?;
        xml_doc_mut
            .get_element_mut(element_id)
            .context("draviavemal-openxml_office::Failed to get text element")?
            .add_text_mut(value)
            .context("draviavemal-openxml_office::Failed to add text value")?;
        Ok(())
    }

    fn serialize_anchor_content(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        anchor_content: AnchorContent,
    ) -> AnyResult<(), AnyError> {
        match anchor_content {
            AnchorContent::Picture(picture) => {
                DrawingPart::serialize_picture(xml_doc_mut, parent_id, picture)
                    .context("draviavemal-openxml_office::Failed to serialise picture")?;
            }
            AnchorContent::GraphicFrame(graphic_frame) => {
                DrawingPart::serialize_graphic_frame(xml_doc_mut, parent_id, graphic_frame)
                    .context("draviavemal-openxml_office::Failed to serialise graphic frame")?;
            }
            AnchorContent::Shape(_) => {
                xml_doc_mut
                    .append_child_element_mut(parent_id, "xdr:sp", None)
                    .context("draviavemal-openxml_office::Failed to add shape element")?;
            }
            AnchorContent::GroupShape(_) => {
                xml_doc_mut
                    .append_child_element_mut(parent_id, "xdr:grpSp", None)
                    .context("draviavemal-openxml_office::Failed to add group shape element")?;
            }
            AnchorContent::ConnectorShape(_) => {
                xml_doc_mut
                    .append_child_element_mut(parent_id, "xdr:cxnSp", None)
                    .context("draviavemal-openxml_office::Failed to add connector shape element")?;
            }
            AnchorContent::ContentPart(_) => {
                xml_doc_mut
                    .append_child_element_mut(parent_id, "xdr:contentPart", None)
                    .context("draviavemal-openxml_office::Failed to add content part element")?;
            }
        }
        Ok(())
    }

    fn serialize_picture(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        picture: Picture,
    ) -> AnyResult<(), AnyError> {
        let picture_id = xml_doc_mut
            .append_child_element_mut(parent_id, "xdr:pic", None)
            .context("draviavemal-openxml_office::Failed to add picture element")?;
        let non_visual_id = xml_doc_mut
            .append_child_element_mut(picture_id, "xdr:nvPicPr", None)
            .context("draviavemal-openxml_office::Failed to add non visual picture element")?;
        xml_doc_mut
            .append_child_element_mut(
                non_visual_id,
                "xdr:cNvPr",
                Some(vec![
                    XmlAttribute::new("id".to_string(), picture.id.to_string()),
                    XmlAttribute::new("name".to_string(), picture.name),
                ]),
            )
            .context("draviavemal-openxml_office::Failed to add picture properties element")?;
        let non_visual_picture_id = xml_doc_mut
            .append_child_element_mut(non_visual_id, "xdr:cNvPicPr", None)
            .context(
                "draviavemal-openxml_office::Failed to add non visual picture properties element",
            )?;
        xml_doc_mut
            .append_child_element_mut(
                non_visual_picture_id,
                "a:picLocks",
                Some(vec![XmlAttribute::new(
                    "noChangeAspect".to_string(),
                    if picture.aspect_ratio { "1" } else { "0" }.to_string(),
                )]),
            )
            .context("draviavemal-openxml_office::Failed to add picture locks element")?;
        let blip_fill_id = xml_doc_mut
            .append_child_element_mut(picture_id, "xdr:blipFill", None)
            .context("draviavemal-openxml_office::Failed to add blip fill element")?;
        xml_doc_mut
            .append_child_element_mut(
                blip_fill_id,
                "a:blip",
                Some(vec![
                    XmlAttribute::new(
                        "xmlns:a".to_string(),
                        "http://schemas.openxmlformats.org/drawingml/2006/main".to_string(),
                    ),
                    XmlAttribute::new(
                        "xmlns:r".to_string(),
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                            .to_string(),
                    ),
                    XmlAttribute::new("r:embed".to_string(), picture.relationship_id),
                ]),
            )
            .context("draviavemal-openxml_office::Failed to add blip element")?;
        Ok(())
    }

    fn serialize_graphic_frame(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        graphic_frame: GraphicFrame,
    ) -> AnyResult<(), AnyError> {
        let graphic_frame_id = xml_doc_mut
            .append_child_element_mut(parent_id, "xdr:graphicFrame", None)
            .context("draviavemal-openxml_office::Failed to add graphic frame element")?;
        let non_visual_id = xml_doc_mut
            .append_child_element_mut(graphic_frame_id, "xdr:nvGraphicFramePr", None)
            .context(
                "draviavemal-openxml_office::Failed to add non visual graphic frame element",
            )?;
        xml_doc_mut
            .append_child_element_mut(
                non_visual_id,
                "xdr:cNvPr",
                Some(vec![
                    XmlAttribute::new("id".to_string(), graphic_frame.id.to_string()),
                    XmlAttribute::new("name".to_string(), graphic_frame.name),
                ]),
            )
            .context(
                "draviavemal-openxml_office::Failed to add graphic frame properties element",
            )?;
        xml_doc_mut
            .append_child_element_mut(non_visual_id, "xdr:cNvGraphicFramePr", None)
            .context("draviavemal-openxml_office::Failed to add non visual graphic frame properties element")?;
        let graphic_id = xml_doc_mut
            .append_child_element_mut(graphic_frame_id, "a:graphic", None)
            .context("draviavemal-openxml_office::Failed to add graphic element")?;
        let graphic_data_id = xml_doc_mut
            .append_child_element_mut(
                graphic_id,
                "a:graphicData",
                Some(vec![XmlAttribute::new(
                    "uri".to_string(),
                    "http://schemas.openxmlformats.org/drawingml/2006/chart".to_string(),
                )]),
            )
            .context("draviavemal-openxml_office::Failed to add graphic data element")?;
        xml_doc_mut
            .append_child_element_mut(
                graphic_data_id,
                "c:chart",
                Some(vec![XmlAttribute::new(
                    "r:id".to_string(),
                    graphic_frame.relationship_id,
                )]),
            )
            .context("draviavemal-openxml_office::Failed to add chart element")?;
        Ok(())
    }
}
