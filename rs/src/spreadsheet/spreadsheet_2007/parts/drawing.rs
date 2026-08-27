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
        let drawing_global = DrawingPartGlobal::new(drawing_relationship_part.clone())
            .context("Failed to create drawing global")?;
        Ok(DrawingPart {
            drawing_global,
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
        let drawing_content = type_collection
            .get("drawing")
            .context("Failed to read drawing type collection")?;
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
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to upgrade relation part",
            ))
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
        if let Some(edit_as) = anchor_element.get_attribute("editAs") {
            two_cell_anchor.edit_as = EditAsValues::get_enum(edit_as.get_value());
        }
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
                        DrawingPartGlobal::deserialise_blip_fill(
                            xml_doc_mut,
                            blip_fill_element,
                            &mut picture,
                        )
                        .context("draviavemal-openxml_office::Failed to parse blip fill")?;
                    }
                    "spPr" => {
                        let shape_property_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get shape property element",
                        )?;
                        DrawingPartGlobal::deserialise_shape_property(
                            xml_doc_mut,
                            shape_property_element,
                            &mut picture,
                        )
                        .context("draviavemal-openxml_office::Failed to parse blip fill")?;
                    }
                    "style" => {}
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
                        if let Some(description) = properties_element.get_attribute("descr") {
                            picture.description = Some(description.get_value().to_string());
                        }
                        if let Some(hidden) = properties_element.get_attribute("hidden") {
                            picture.hidden = hidden.get_value() == "1";
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

    fn deserialise_graphic_frame(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        graphic_frame_element: &XmlElement,
    ) -> AnyResult<GraphicFrame, AnyError> {
        let mut graphic_frame = GraphicFrame::default();
        if let Some(macro_reference) = graphic_frame_element.get_attribute("macro") {
            graphic_frame.macro_reference = Some(macro_reference.get_value().to_string());
        }
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
                    "xfrm" => {
                        let transform_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get graphic frame transform element",
                        )?;
                        graphic_frame.transform = Some(
                            DrawingPartGlobal::deserialise_transform(xml_doc_mut, transform_element)
                                .context(
                                    "draviavemal-openxml_office::Failed to parse graphic frame transform",
                                )?,
                        );
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
                    "cNvGraphicFramePr" => {}
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
        if let Some(graphic_uri) = graphic_data_element.get_attribute("uri") {
            graphic_frame.graphic_uri = graphic_uri.get_value().to_string();
        }
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
                        if let Some(chart_relationship_id) =
                            chart_element.get_attribute_ns("id", &RELATIONSHIP_OFFICE_DOC_NS)
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
        let anchor_attributes = match two_cell_anchor.edit_as {
            EditAsValues::TwoCell => None,
            edit_as => Some(vec![XmlAttribute::new(
                "editAs".to_string(),
                EditAsValues::get_string(edit_as),
            )]),
        };
        let anchor_id = xml_doc_mut
            .append_child_element_ns_mut(
                parent_id,
                "twoCellAnchor",
                &SPREADSHEET_DRAWING_NS,
                anchor_attributes,
            )
            .context("draviavemal-openxml_office::Failed to add two cell anchor element")?;
        DrawingPart::serialize_anchor_position(
            xml_doc_mut,
            anchor_id,
            "from",
            &two_cell_anchor.from,
        )
        .context("draviavemal-openxml_office::Failed to serialise from position")?;
        DrawingPart::serialize_anchor_position(xml_doc_mut, anchor_id, "to", &two_cell_anchor.to)
            .context("draviavemal-openxml_office::Failed to serialise to position")?;
        DrawingPart::serialize_anchor_content(
            xml_doc_mut,
            anchor_id,
            two_cell_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_doc_mut
            .append_child_element_ns_mut(anchor_id, "clientData", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_one_cell_anchor(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        one_cell_anchor: OneCellAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_doc_mut
            .append_child_element_ns_mut(parent_id, "oneCellAnchor", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add one cell anchor element")?;
        DrawingPart::serialize_anchor_position(
            xml_doc_mut,
            anchor_id,
            "from",
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
            .append_child_element_ns_mut(anchor_id, "clientData", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_absolute_anchor(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        absolute_anchor: AbsoluteAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_doc_mut
            .append_child_element_ns_mut(parent_id, "absoluteAnchor", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add absolute anchor element")?;
        DrawingPart::serialize_anchor_content(
            xml_doc_mut,
            anchor_id,
            absolute_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_doc_mut
            .append_child_element_ns_mut(anchor_id, "clientData", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_anchor_position(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        local_name: &str,
        anchor_position: &AnchorPosition,
    ) -> AnyResult<(), AnyError> {
        let position_id = xml_doc_mut
            .append_child_element_ns_mut(parent_id, local_name, &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add anchor position element")?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "col",
            &anchor_position.column.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "colOff",
            &anchor_position.column_offset.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "row",
            &anchor_position.row.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_doc_mut,
            position_id,
            "rowOff",
            &anchor_position.row_offset.to_string(),
        )?;
        Ok(())
    }

    fn serialize_text_element(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        local_name: &str,
        value: &str,
    ) -> AnyResult<(), AnyError> {
        let element_id = xml_doc_mut
            .append_child_element_ns_mut(parent_id, local_name, &SPREADSHEET_DRAWING_NS, None)
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
                    .append_child_element_ns_mut(parent_id, "sp", &SPREADSHEET_DRAWING_NS, None)
                    .context("draviavemal-openxml_office::Failed to add shape element")?;
            }
            AnchorContent::GroupShape(_) => {
                xml_doc_mut
                    .append_child_element_ns_mut(parent_id, "grpSp", &SPREADSHEET_DRAWING_NS, None)
                    .context("draviavemal-openxml_office::Failed to add group shape element")?;
            }
            AnchorContent::ConnectorShape(_) => {
                xml_doc_mut
                    .append_child_element_ns_mut(parent_id, "cxnSp", &SPREADSHEET_DRAWING_NS, None)
                    .context("draviavemal-openxml_office::Failed to add connector shape element")?;
            }
            AnchorContent::ContentPart(_) => {
                xml_doc_mut
                    .append_child_element_ns_mut(
                        parent_id,
                        "contentPart",
                        &SPREADSHEET_DRAWING_NS,
                        None,
                    )
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
            .append_child_element_ns_mut(parent_id, "pic", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add picture element")?;
        let non_visual_id = xml_doc_mut
            .append_child_element_ns_mut(picture_id, "nvPicPr", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add non visual picture element")?;
        let mut properties_attributes = vec![
            XmlAttribute::new("id".to_string(), picture.id.to_string()),
            XmlAttribute::new("name".to_string(), picture.name),
        ];
        if let Some(description) = picture.description {
            properties_attributes.push(XmlAttribute::new("descr".to_string(), description));
        }
        if picture.hidden {
            properties_attributes.push(XmlAttribute::new("hidden".to_string(), "1".to_string()));
        }
        xml_doc_mut
            .append_child_element_ns_mut(
                non_visual_id,
                "cNvPr",
                &SPREADSHEET_DRAWING_NS,
                Some(properties_attributes),
            )
            .context("draviavemal-openxml_office::Failed to add picture properties element")?;
        let non_visual_picture_id = xml_doc_mut
            .append_child_element_ns_mut(non_visual_id, "cNvPicPr", &SPREADSHEET_DRAWING_NS, None)
            .context(
                "draviavemal-openxml_office::Failed to add non visual picture properties element",
            )?;
        xml_doc_mut
            .append_child_element_ns_mut(
                non_visual_picture_id,
                "picLocks",
                &DRAWINGML_NS,
                Some(vec![XmlAttribute::new(
                    "noChangeAspect".to_string(),
                    if picture.aspect_ratio { "1" } else { "0" }.to_string(),
                )]),
            )
            .context("draviavemal-openxml_office::Failed to add picture locks element")?;
        let blip_fill_id = xml_doc_mut
            .append_child_element_ns_mut(picture_id, "blipFill", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add blip fill element")?;
        let mut blip_attributes = Vec::new();
        if let Some(compression_state) = picture.compression_state {
            blip_attributes.push(XmlAttribute::new("cstate".to_string(), compression_state));
        }
        let blip_id = xml_doc_mut
            .append_child_element_ns_mut(
                blip_fill_id,
                "blip",
                &DRAWINGML_NS,
                if blip_attributes.is_empty() {
                    None
                } else {
                    Some(blip_attributes)
                },
            )
            .context("draviavemal-openxml_office::Failed to add blip element")?;
        xml_doc_mut
            .get_element_mut(blip_id)
            .context("draviavemal-openxml_office::Failed to get blip element")?
            .add_attribute_ns_mut(
                "embed",
                &RELATIONSHIP_OFFICE_DOC_NS,
                &picture.relationship_id,
            )
            .context("draviavemal-openxml_office::Failed to add blip relationship")?;
        if let Some(source_rectangle) = picture.source_rectangle {
            DrawingPart::serialize_relative_rect(
                xml_doc_mut,
                blip_fill_id,
                "srcRect",
                &source_rectangle,
            )
            .context("draviavemal-openxml_office::Failed to add source rectangle element")?;
        }
        match picture.fill_mode {
            BlipFillMode::Stretch(fill_rect) => {
                let stretch_id = xml_doc_mut
                    .append_child_element_ns_mut(blip_fill_id, "stretch", &DRAWINGML_NS, None)
                    .context("draviavemal-openxml_office::Failed to add stretch element")?;
                DrawingPart::serialize_relative_rect(
                    xml_doc_mut,
                    stretch_id,
                    "fillRect",
                    &fill_rect,
                )
                .context("draviavemal-openxml_office::Failed to add fill rectangle element")?;
            }
            BlipFillMode::Tile(tile) => {
                let mut tile_attributes = Vec::new();
                if let Some(offset_x) = tile.offset_x {
                    tile_attributes.push(XmlAttribute::new("tx".to_string(), offset_x.to_string()));
                }
                if let Some(offset_y) = tile.offset_y {
                    tile_attributes.push(XmlAttribute::new("ty".to_string(), offset_y.to_string()));
                }
                if let Some(scale_x) = tile.scale_x {
                    tile_attributes.push(XmlAttribute::new("sx".to_string(), scale_x.to_string()));
                }
                if let Some(scale_y) = tile.scale_y {
                    tile_attributes.push(XmlAttribute::new("sy".to_string(), scale_y.to_string()));
                }
                if let Some(flip) = tile.flip {
                    tile_attributes.push(XmlAttribute::new("flip".to_string(), flip));
                }
                if let Some(alignment) = tile.alignment {
                    tile_attributes.push(XmlAttribute::new("algn".to_string(), alignment));
                }
                xml_doc_mut
                    .append_child_element_ns_mut(
                        blip_fill_id,
                        "tile",
                        &DRAWINGML_NS,
                        if tile_attributes.is_empty() {
                            None
                        } else {
                            Some(tile_attributes)
                        },
                    )
                    .context("draviavemal-openxml_office::Failed to add tile element")?;
            }
        }
        DrawingPart::serialize_shape_properties(xml_doc_mut, picture_id, &picture.shape_properties)
            .context("draviavemal-openxml_office::Failed to serialise shape properties")?;
        Ok(())
    }

    fn serialize_shape_properties(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        shape_properties: &ShapeProperties,
    ) -> AnyResult<(), AnyError> {
        let shape_property_id = xml_doc_mut
            .append_child_element_ns_mut(parent_id, "spPr", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add shape property element")?;
        if let Some(transform) = &shape_properties.transform {
            DrawingPart::serialize_transform(xml_doc_mut, shape_property_id, transform)
                .context("draviavemal-openxml_office::Failed to serialise transform")?;
        }
        if let Some(preset_geometry) = &shape_properties.preset_geometry {
            let preset_geometry_id = xml_doc_mut
                .append_child_element_ns_mut(
                    shape_property_id,
                    "prstGeom",
                    &DRAWINGML_NS,
                    Some(vec![XmlAttribute::new(
                        "prst".to_string(),
                        preset_geometry.preset.clone(),
                    )]),
                )
                .context("draviavemal-openxml_office::Failed to add preset geometry element")?;
            xml_doc_mut
                .append_child_element_ns_mut(preset_geometry_id, "avLst", &DRAWINGML_NS, None)
                .context("draviavemal-openxml_office::Failed to add adjust value list element")?;
        }
        Ok(())
    }

    fn serialize_transform(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        transform: &Transform,
    ) -> AnyResult<(), AnyError> {
        let mut transform_attributes = Vec::new();
        if let Some(rotation) = transform.rotation {
            transform_attributes.push(XmlAttribute::new("rot".to_string(), rotation.to_string()));
        }
        if transform.flip_horizontal {
            transform_attributes.push(XmlAttribute::new("flipH".to_string(), "1".to_string()));
        }
        if transform.flip_vertical {
            transform_attributes.push(XmlAttribute::new("flipV".to_string(), "1".to_string()));
        }
        let transform_id = xml_doc_mut
            .append_child_element_ns_mut(
                parent_id,
                "xfrm",
                &DRAWINGML_NS,
                if transform_attributes.is_empty() {
                    None
                } else {
                    Some(transform_attributes)
                },
            )
            .context("draviavemal-openxml_office::Failed to add transform element")?;
        if let Some(offset) = &transform.offset {
            xml_doc_mut
                .append_child_element_ns_mut(
                    transform_id,
                    "off",
                    &DRAWINGML_NS,
                    Some(vec![
                        XmlAttribute::new("x".to_string(), offset.x.to_string()),
                        XmlAttribute::new("y".to_string(), offset.y.to_string()),
                    ]),
                )
                .context("draviavemal-openxml_office::Failed to add offset element")?;
        }
        if let Some(extent) = &transform.extent {
            xml_doc_mut
                .append_child_element_ns_mut(
                    transform_id,
                    "ext",
                    &DRAWINGML_NS,
                    Some(vec![
                        XmlAttribute::new("cx".to_string(), extent.width.to_string()),
                        XmlAttribute::new("cy".to_string(), extent.height.to_string()),
                    ]),
                )
                .context("draviavemal-openxml_office::Failed to add extent element")?;
        }
        Ok(())
    }

    fn serialize_relative_rect(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        local_name: &str,
        relative_rect: &RelativeRect,
    ) -> AnyResult<(), AnyError> {
        let mut rect_attributes = Vec::new();
        if let Some(left) = relative_rect.left {
            rect_attributes.push(XmlAttribute::new("l".to_string(), left.to_string()));
        }
        if let Some(top) = relative_rect.top {
            rect_attributes.push(XmlAttribute::new("t".to_string(), top.to_string()));
        }
        if let Some(right) = relative_rect.right {
            rect_attributes.push(XmlAttribute::new("r".to_string(), right.to_string()));
        }
        if let Some(bottom) = relative_rect.bottom {
            rect_attributes.push(XmlAttribute::new("b".to_string(), bottom.to_string()));
        }
        xml_doc_mut
            .append_child_element_ns_mut(
                parent_id,
                local_name,
                &DRAWINGML_NS,
                if rect_attributes.is_empty() {
                    None
                } else {
                    Some(rect_attributes)
                },
            )
            .context("draviavemal-openxml_office::Failed to add relative rectangle element")?;
        Ok(())
    }

    fn serialize_graphic_frame(
        xml_doc_mut: &mut XmlDocument,
        parent_id: NodeId,
        graphic_frame: GraphicFrame,
    ) -> AnyResult<(), AnyError> {
        let graphic_frame_id = xml_doc_mut
            .append_child_element_ns_mut(
                parent_id,
                "graphicFrame",
                &SPREADSHEET_DRAWING_NS,
                graphic_frame
                    .macro_reference
                    .clone()
                    .map(|macro_reference| {
                        vec![XmlAttribute::new("macro".to_string(), macro_reference)]
                    }),
            )
            .context("draviavemal-openxml_office::Failed to add graphic frame element")?;
        let non_visual_id = xml_doc_mut
            .append_child_element_ns_mut(
                graphic_frame_id,
                "nvGraphicFramePr",
                &SPREADSHEET_DRAWING_NS,
                None,
            )
            .context(
                "draviavemal-openxml_office::Failed to add non visual graphic frame element",
            )?;
        xml_doc_mut
            .append_child_element_ns_mut(
                non_visual_id,
                "cNvPr",
                &SPREADSHEET_DRAWING_NS,
                Some(vec![
                    XmlAttribute::new("id".to_string(), graphic_frame.id.to_string()),
                    XmlAttribute::new("name".to_string(), graphic_frame.name),
                ]),
            )
            .context(
                "draviavemal-openxml_office::Failed to add graphic frame properties element",
            )?;
        xml_doc_mut
            .append_child_element_ns_mut(
                non_visual_id,
                "cNvGraphicFramePr",
                &SPREADSHEET_DRAWING_NS,
                None,
            )
            .context("draviavemal-openxml_office::Failed to add non visual graphic frame properties element")?;
        if let Some(transform) = &graphic_frame.transform {
            let transform_id = xml_doc_mut
                .append_child_element_ns_mut(
                    graphic_frame_id,
                    "xfrm",
                    &SPREADSHEET_DRAWING_NS,
                    None,
                )
                .context(
                    "draviavemal-openxml_office::Failed to add graphic frame transform element",
                )?;
            if let Some(offset) = &transform.offset {
                xml_doc_mut
                    .append_child_element_ns_mut(
                        transform_id,
                        "off",
                        &DRAWINGML_NS,
                        Some(vec![
                            XmlAttribute::new("x".to_string(), offset.x.to_string()),
                            XmlAttribute::new("y".to_string(), offset.y.to_string()),
                        ]),
                    )
                    .context("draviavemal-openxml_office::Failed to add offset element")?;
            }
            if let Some(extent) = &transform.extent {
                xml_doc_mut
                    .append_child_element_ns_mut(
                        transform_id,
                        "ext",
                        &DRAWINGML_NS,
                        Some(vec![
                            XmlAttribute::new("cx".to_string(), extent.width.to_string()),
                            XmlAttribute::new("cy".to_string(), extent.height.to_string()),
                        ]),
                    )
                    .context("draviavemal-openxml_office::Failed to add extent element")?;
            }
        }
        let graphic_id = xml_doc_mut
            .append_child_element_ns_mut(graphic_frame_id, "graphic", &DRAWINGML_NS, None)
            .context("draviavemal-openxml_office::Failed to add graphic element")?;
        let graphic_uri = if graphic_frame.graphic_uri.is_empty() {
            CHART_NS.uri.to_string()
        } else {
            graphic_frame.graphic_uri.clone()
        };
        let graphic_data_id = xml_doc_mut
            .append_child_element_ns_mut(
                graphic_id,
                "graphicData",
                &DRAWINGML_NS,
                Some(vec![XmlAttribute::new("uri".to_string(), graphic_uri)]),
            )
            .context("draviavemal-openxml_office::Failed to add graphic data element")?;
        let chart_id = xml_doc_mut
            .append_child_element_ns_mut(graphic_data_id, "chart", &CHART_NS, None)
            .context("draviavemal-openxml_office::Failed to add chart element")?;
        xml_doc_mut
            .get_element_mut(chart_id)
            .context("draviavemal-openxml_office::Failed to get chart element")?
            .add_attribute_ns_mut(
                "id",
                &RELATIONSHIP_OFFICE_DOC_NS,
                &graphic_frame.relationship_id,
            )
            .context("draviavemal-openxml_office::Failed to add chart relationship")?;
        Ok(())
    }
}

// ##################################### Mut Feature Function ################################
impl DrawingPart {
    pub fn add_picture_mut(
        &mut self,
        image_path: &str,
        picture_setting: ExcelPictureSetting,
    ) -> Result<(), AnyError> {
        self.drawing_global
            .add_picture_mut(image_path, picture_setting.picture_setting)
            .context("Failed Global Image Handling")?;
        Ok(())
    }
}
