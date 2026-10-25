use crate::{
    global_2007::{
        models::{AnchorPosition, GraphPosition, Picture},
        parts::DrawingPartGlobal,
        traits::Enum,
    },
    namespaces::RELATIONSHIP_OFFICE_DOC_NS,
    spreadsheet_2007::{
        models::{
            AbsoluteAnchor, AnchorContent, ConnectorShape, ContentPart, DrawingAnchor,
            EditAsValues, GraphicFrame, GroupShape, OneCellAnchor, Shape, TwoCellAnchor,
        },
        DrawingPart,
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, Tag, XmlDocument, XmlElement, XmlElementContentType};
use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    rc::Weak,
};

impl DrawingPart {
    pub(crate) fn deserialize_drawing(
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
}
