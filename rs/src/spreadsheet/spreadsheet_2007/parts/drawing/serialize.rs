use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlAttribute, XmlDocument};

use super::{
    AbsoluteAnchor, AnchorContent, DrawingAnchor, EditAsValues, GraphicFrame, OneCellAnchor,
    TwoCellAnchor,
};
use crate::{
    global_2007::{
        parts::{AnchorPosition, BlipFillMode, Picture, RelativeRect, ShapeProperties, Transform},
        traits::Enum,
    },
    namespaces::{CHART_NS, DRAWINGML_NS, RELATIONSHIP_OFFICE_DOC_NS, SPREADSHEET_DRAWING_NS},
    spreadsheet_2007::DrawingPart,
};

impl DrawingPart {
    pub(crate) fn serialize_drawing(
        &mut self,
        xml_document_mut: &mut XmlDocument,
    ) -> AnyResult<(), AnyError> {
        if let Some(mut anchor_collection) = self.anchor_collection.take() {
            let root_id = xml_document_mut.get_root_id();
            while let Some(anchor) = anchor_collection.pop_front() {
                match anchor {
                    DrawingAnchor::TwoCellAnchor(two_cell_anchor) => {
                        DrawingPart::serialize_two_cell_anchor(
                            xml_document_mut,
                            root_id,
                            two_cell_anchor,
                        )
                        .context(
                            "draviavemal-openxml_office::Failed to serialise two cell anchor",
                        )?;
                    }
                    DrawingAnchor::OneCellAnchor(one_cell_anchor) => {
                        DrawingPart::serialize_one_cell_anchor(
                            xml_document_mut,
                            root_id,
                            one_cell_anchor,
                        )
                        .context(
                            "draviavemal-openxml_office::Failed to serialise one cell anchor",
                        )?;
                    }
                    DrawingAnchor::AbsoluteAnchor(absolute_anchor) => {
                        DrawingPart::serialize_absolute_anchor(
                            xml_document_mut,
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
        xml_document_mut: &mut XmlDocument,
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
        let anchor_id = xml_document_mut
            .append_child_element_ns_mut(
                parent_id,
                "twoCellAnchor",
                &SPREADSHEET_DRAWING_NS,
                anchor_attributes,
            )
            .context("draviavemal-openxml_office::Failed to add two cell anchor element")?;
        DrawingPart::serialize_anchor_position(
            xml_document_mut,
            anchor_id,
            "from",
            &two_cell_anchor.from,
        )
        .context("draviavemal-openxml_office::Failed to serialise from position")?;
        DrawingPart::serialize_anchor_position(xml_document_mut, anchor_id, "to", &two_cell_anchor.to)
            .context("draviavemal-openxml_office::Failed to serialise to position")?;
        DrawingPart::serialize_anchor_content(
            xml_document_mut,
            anchor_id,
            two_cell_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_document_mut
            .append_child_element_ns_mut(anchor_id, "clientData", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_one_cell_anchor(
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        one_cell_anchor: OneCellAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_document_mut
            .append_child_element_ns_mut(parent_id, "oneCellAnchor", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add one cell anchor element")?;
        DrawingPart::serialize_anchor_position(
            xml_document_mut,
            anchor_id,
            "from",
            &one_cell_anchor.from,
        )
        .context("draviavemal-openxml_office::Failed to serialise from position")?;
        DrawingPart::serialize_anchor_content(
            xml_document_mut,
            anchor_id,
            one_cell_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_document_mut
            .append_child_element_ns_mut(anchor_id, "clientData", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_absolute_anchor(
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        absolute_anchor: AbsoluteAnchor,
    ) -> AnyResult<(), AnyError> {
        let anchor_id = xml_document_mut
            .append_child_element_ns_mut(parent_id, "absoluteAnchor", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add absolute anchor element")?;
        DrawingPart::serialize_anchor_content(
            xml_document_mut,
            anchor_id,
            absolute_anchor.anchor_content,
        )
        .context("draviavemal-openxml_office::Failed to serialise anchor content")?;
        xml_document_mut
            .append_child_element_ns_mut(anchor_id, "clientData", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add client data element")?;
        Ok(())
    }

    fn serialize_anchor_position(
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        local_name: &str,
        anchor_position: &AnchorPosition,
    ) -> AnyResult<(), AnyError> {
        let position_id = xml_document_mut
            .append_child_element_ns_mut(parent_id, local_name, &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add anchor position element")?;
        DrawingPart::serialize_text_element(
            xml_document_mut,
            position_id,
            "col",
            &anchor_position.column.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_document_mut,
            position_id,
            "colOff",
            &anchor_position.column_offset.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_document_mut,
            position_id,
            "row",
            &anchor_position.row.to_string(),
        )?;
        DrawingPart::serialize_text_element(
            xml_document_mut,
            position_id,
            "rowOff",
            &anchor_position.row_offset.to_string(),
        )?;
        Ok(())
    }

    fn serialize_text_element(
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        local_name: &str,
        value: &str,
    ) -> AnyResult<(), AnyError> {
        let element_id = xml_document_mut
            .append_child_element_ns_mut(parent_id, local_name, &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add text element")?;
        xml_document_mut
            .get_element_mut(element_id)
            .context("draviavemal-openxml_office::Failed to get text element")?
            .add_text_mut(value)
            .context("draviavemal-openxml_office::Failed to add text value")?;
        Ok(())
    }

    fn serialize_anchor_content(
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        anchor_content: AnchorContent,
    ) -> AnyResult<(), AnyError> {
        match anchor_content {
            AnchorContent::Picture(picture) => {
                DrawingPart::serialize_picture(xml_document_mut, parent_id, picture)
                    .context("draviavemal-openxml_office::Failed to serialise picture")?;
            }
            AnchorContent::GraphicFrame(graphic_frame) => {
                DrawingPart::serialize_graphic_frame(xml_document_mut, parent_id, graphic_frame)
                    .context("draviavemal-openxml_office::Failed to serialise graphic frame")?;
            }
            AnchorContent::Shape(_) => {
                xml_document_mut
                    .append_child_element_ns_mut(parent_id, "sp", &SPREADSHEET_DRAWING_NS, None)
                    .context("draviavemal-openxml_office::Failed to add shape element")?;
            }
            AnchorContent::GroupShape(_) => {
                xml_document_mut
                    .append_child_element_ns_mut(parent_id, "grpSp", &SPREADSHEET_DRAWING_NS, None)
                    .context("draviavemal-openxml_office::Failed to add group shape element")?;
            }
            AnchorContent::ConnectorShape(_) => {
                xml_document_mut
                    .append_child_element_ns_mut(parent_id, "cxnSp", &SPREADSHEET_DRAWING_NS, None)
                    .context("draviavemal-openxml_office::Failed to add connector shape element")?;
            }
            AnchorContent::ContentPart(_) => {
                xml_document_mut
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
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        picture: Picture,
    ) -> AnyResult<(), AnyError> {
        let picture_id = xml_document_mut
            .append_child_element_ns_mut(parent_id, "pic", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add picture element")?;
        let non_visual_id = xml_document_mut
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
        xml_document_mut
            .append_child_element_ns_mut(
                non_visual_id,
                "cNvPr",
                &SPREADSHEET_DRAWING_NS,
                Some(properties_attributes),
            )
            .context("draviavemal-openxml_office::Failed to add picture properties element")?;
        let non_visual_picture_id = xml_document_mut
            .append_child_element_ns_mut(non_visual_id, "cNvPicPr", &SPREADSHEET_DRAWING_NS, None)
            .context(
                "draviavemal-openxml_office::Failed to add non visual picture properties element",
            )?;
        xml_document_mut
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
        let blip_fill_id = xml_document_mut
            .append_child_element_ns_mut(picture_id, "blipFill", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add blip fill element")?;
        let mut blip_attributes = Vec::new();
        if let Some(compression_state) = picture.compression_state {
            blip_attributes.push(XmlAttribute::new("cstate".to_string(), compression_state));
        }
        let blip_id = xml_document_mut
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
        xml_document_mut
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
                xml_document_mut,
                blip_fill_id,
                "srcRect",
                &source_rectangle,
            )
            .context("draviavemal-openxml_office::Failed to add source rectangle element")?;
        }
        match picture.fill_mode {
            BlipFillMode::Stretch(fill_rect) => {
                let stretch_id = xml_document_mut
                    .append_child_element_ns_mut(blip_fill_id, "stretch", &DRAWINGML_NS, None)
                    .context("draviavemal-openxml_office::Failed to add stretch element")?;
                DrawingPart::serialize_relative_rect(
                    xml_document_mut,
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
                xml_document_mut
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
        DrawingPart::serialize_shape_properties(xml_document_mut, picture_id, &picture.shape_properties)
            .context("draviavemal-openxml_office::Failed to serialise shape properties")?;
        Ok(())
    }

    fn serialize_shape_properties(
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        shape_properties: &ShapeProperties,
    ) -> AnyResult<(), AnyError> {
        let shape_property_id = xml_document_mut
            .append_child_element_ns_mut(parent_id, "spPr", &SPREADSHEET_DRAWING_NS, None)
            .context("draviavemal-openxml_office::Failed to add shape property element")?;
        if let Some(transform) = &shape_properties.transform {
            DrawingPart::serialize_transform(xml_document_mut, shape_property_id, transform)
                .context("draviavemal-openxml_office::Failed to serialise transform")?;
        }
        if let Some(preset_geometry) = &shape_properties.preset_geometry {
            let preset_geometry_id = xml_document_mut
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
            xml_document_mut
                .append_child_element_ns_mut(preset_geometry_id, "avLst", &DRAWINGML_NS, None)
                .context("draviavemal-openxml_office::Failed to add adjust value list element")?;
        }
        Ok(())
    }

    fn serialize_transform(
        xml_document_mut: &mut XmlDocument,
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
        let transform_id = xml_document_mut
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
            xml_document_mut
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
            xml_document_mut
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
        xml_document_mut: &mut XmlDocument,
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
        xml_document_mut
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
        xml_document_mut: &mut XmlDocument,
        parent_id: NodeId,
        graphic_frame: GraphicFrame,
    ) -> AnyResult<(), AnyError> {
        let graphic_frame_id = xml_document_mut
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
        let non_visual_id = xml_document_mut
            .append_child_element_ns_mut(
                graphic_frame_id,
                "nvGraphicFramePr",
                &SPREADSHEET_DRAWING_NS,
                None,
            )
            .context(
                "draviavemal-openxml_office::Failed to add non visual graphic frame element",
            )?;
        xml_document_mut
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
        xml_document_mut
            .append_child_element_ns_mut(
                non_visual_id,
                "cNvGraphicFramePr",
                &SPREADSHEET_DRAWING_NS,
                None,
            )
            .context("draviavemal-openxml_office::Failed to add non visual graphic frame properties element")?;
        if let Some(transform) = &graphic_frame.transform {
            let transform_id = xml_document_mut
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
                xml_document_mut
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
                xml_document_mut
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
        let graphic_id = xml_document_mut
            .append_child_element_ns_mut(graphic_frame_id, "graphic", &DRAWINGML_NS, None)
            .context("draviavemal-openxml_office::Failed to add graphic element")?;
        let graphic_uri = if graphic_frame.graphic_uri.is_empty() {
            CHART_NS.uri.to_string()
        } else {
            graphic_frame.graphic_uri.clone()
        };
        let graphic_data_id = xml_document_mut
            .append_child_element_ns_mut(
                graphic_id,
                "graphicData",
                &DRAWINGML_NS,
                Some(vec![XmlAttribute::new("uri".to_string(), graphic_uri)]),
            )
            .context("draviavemal-openxml_office::Failed to add graphic data element")?;
        let chart_id = xml_document_mut
            .append_child_element_ns_mut(graphic_data_id, "chart", &CHART_NS, None)
            .context("draviavemal-openxml_office::Failed to add chart element")?;
        xml_document_mut
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
