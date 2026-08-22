use std::{
    cell::{RefCell, RefMut},
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
    rc::Rc,
};

use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use blake3::Hasher;
use draviavemal_xml_rs::{XmlDocument, XmlElement, XmlElementContentType};

use crate::{
    global_2007::{
        models::{
            BlipFillMode, Extent, Offset, Picture, PictureSetting, PresetGeometry, RelativeRect,
            TileProperties, Transform,
        },
        parts::{MediaGlobal, RelationsPart},
    },
    namespaces::RELATIONSHIP_OFFICE_DOC_NS,
};

#[derive(Debug)]
pub(crate) struct DrawingPartGlobal {
    drawing_relationship_part: Rc<RefCell<RelationsPart>>,
    medias: Vec<MediaGlobal>,
}

impl DrawingPartGlobal {
    pub(crate) fn new(drawing_relationship_part: Rc<RefCell<RelationsPart>>) -> DrawingPartGlobal {
        Self {
            drawing_relationship_part,
            medias: vec![],
        }
    }

    pub(crate) fn deserialise_blip_fill(
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
                        if let Some(relationship_id) =
                            blip_element.get_attribute_ns("embed", &RELATIONSHIP_OFFICE_DOC_NS)
                        {
                            picture.relationship_id = relationship_id.get_value().to_string();
                        }
                        if let Some(compression_state) = blip_element.get_attribute("cstate") {
                            picture.compression_state =
                                Some(compression_state.get_value().to_string());
                        }
                    }
                    "srcRect" => {
                        let source_rect_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get source rectangle element",
                        )?;
                        picture.source_rectangle = Some(
                            DrawingPartGlobal::deserialise_relative_rect(source_rect_element),
                        );
                    }
                    "stretch" => {
                        let stretch_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get stretch element")?;
                        let fill_rect = stretch_element
                            .find_first_child("fillRect")
                            .and_then(|fill_rect_id| xml_doc_mut.get_element(fill_rect_id).ok())
                            .map(DrawingPartGlobal::deserialise_relative_rect)
                            .unwrap_or_default();
                        picture.fill_mode = BlipFillMode::Stretch(fill_rect);
                    }
                    "tile" => {
                        let tile_element = xml_doc_mut
                            .get_element(*element_id)
                            .context("draviavemal-openxml_office::Failed to get tile element")?;
                        picture.fill_mode =
                            BlipFillMode::Tile(DrawingPartGlobal::deserialise_tile(tile_element)?);
                    }
                    "extLst" => {}
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

    pub(crate) fn deserialise_shape_property(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        shape_property_element: &XmlElement,
        picture: &mut Picture,
    ) -> AnyResult<(), AnyError> {
        let shape_property_children = shape_property_element
            .get_child_contents()
            .as_ref()
            .context("draviavemal-openxml_office::Failed to open shape property content vec")?;
        for shape_property_child in shape_property_children {
            match shape_property_child {
                XmlElementContentType::Element((element_id, tag, _)) => match tag.as_str() {
                    "xfrm" => {
                        let transform_element = xml_doc_mut.get_element(*element_id).context(
                            "draviavemal-openxml_office::Failed to get transform element",
                        )?;
                        picture.shape_properties.transform = Some(
                            DrawingPartGlobal::deserialise_transform(
                                xml_doc_mut,
                                transform_element,
                            )
                            .context("draviavemal-openxml_office::Failed to parse transform")?,
                        );
                    }
                    "prstGeom" => {
                        let preset_geometry_element =
                            xml_doc_mut.get_element(*element_id).context(
                                "draviavemal-openxml_office::Failed to get preset geometry element",
                            )?;
                        let mut preset_geometry = PresetGeometry::default();
                        if let Some(preset) = preset_geometry_element.get_attribute("prst") {
                            preset_geometry.preset = preset.get_value().to_string();
                        }
                        picture.shape_properties.preset_geometry = Some(preset_geometry);
                    }
                    "custGeom" | "noFill" | "solidFill" | "gradFill" | "blipFill" | "pattFill"
                    | "grpFill" | "ln" | "effectLst" | "effectDag" | "scene3d" | "sp3d"
                    | "extLst" => {}
                    _ => {
                        log::warn!("Unsupported shape property node '{}' is not deserialised and will be lost on save", tag);
                    }
                },
                _ => {
                    log::warn!(
                        "Unsupported shape property content is not deserialised and will be lost on save"
                    );
                }
            }
        }
        Ok(())
    }

    pub(crate) fn deserialise_relative_rect(rect_element: &XmlElement) -> RelativeRect {
        let parse_edge = |name: &str| -> Option<i32> {
            rect_element
                .get_attribute(name)
                .and_then(|attribute| attribute.get_value().trim().parse().ok())
        };
        RelativeRect {
            left: parse_edge("l"),
            top: parse_edge("t"),
            right: parse_edge("r"),
            bottom: parse_edge("b"),
        }
    }

    pub(crate) fn deserialise_tile(
        tile_element: &XmlElement,
    ) -> AnyResult<TileProperties, AnyError> {
        let mut tile = TileProperties::default();
        if let Some(offset_x) = tile_element.get_attribute("tx") {
            tile.offset_x = Some(
                offset_x
                    .get_value()
                    .trim()
                    .parse()
                    .context("draviavemal-openxml_office::Failed to parse tile offset x")?,
            );
        }
        if let Some(offset_y) = tile_element.get_attribute("ty") {
            tile.offset_y = Some(
                offset_y
                    .get_value()
                    .trim()
                    .parse()
                    .context("draviavemal-openxml_office::Failed to parse tile offset y")?,
            );
        }
        if let Some(scale_x) = tile_element.get_attribute("sx") {
            tile.scale_x = Some(
                scale_x
                    .get_value()
                    .trim()
                    .parse()
                    .context("draviavemal-openxml_office::Failed to parse tile scale x")?,
            );
        }
        if let Some(scale_y) = tile_element.get_attribute("sy") {
            tile.scale_y = Some(
                scale_y
                    .get_value()
                    .trim()
                    .parse()
                    .context("draviavemal-openxml_office::Failed to parse tile scale y")?,
            );
        }
        if let Some(flip) = tile_element.get_attribute("flip") {
            tile.flip = Some(flip.get_value().to_string());
        }
        if let Some(alignment) = tile_element.get_attribute("algn") {
            tile.alignment = Some(alignment.get_value().to_string());
        }
        Ok(tile)
    }
    pub(crate) fn deserialise_transform(
        xml_doc_mut: &RefMut<'_, XmlDocument>,
        transform_element: &XmlElement,
    ) -> AnyResult<Transform, AnyError> {
        let mut transform = Transform::default();
        if let Some(rotation) = transform_element.get_attribute("rot") {
            transform.rotation =
                Some(
                    rotation.get_value().trim().parse().context(
                        "draviavemal-openxml_office::Failed to parse transform rotation",
                    )?,
                );
        }
        if let Some(flip_horizontal) = transform_element.get_attribute("flipH") {
            transform.flip_horizontal = flip_horizontal.get_value() == "1";
        }
        if let Some(flip_vertical) = transform_element.get_attribute("flipV") {
            transform.flip_vertical = flip_vertical.get_value() == "1";
        }
        if let Some(children) = transform_element.get_child_contents().as_ref() {
            for child in children {
                if let XmlElementContentType::Element((element_id, tag, _)) = child {
                    match tag.as_str() {
                        "off" => {
                            let offset_element = xml_doc_mut.get_element(*element_id).context(
                                "draviavemal-openxml_office::Failed to get offset element",
                            )?;
                            let mut offset = Offset::default();
                            if let Some(x) = offset_element.get_attribute("x") {
                                offset.x = x.get_value().trim().parse().context(
                                    "draviavemal-openxml_office::Failed to parse offset x",
                                )?;
                            }
                            if let Some(y) = offset_element.get_attribute("y") {
                                offset.y = y.get_value().trim().parse().context(
                                    "draviavemal-openxml_office::Failed to parse offset y",
                                )?;
                            }
                            transform.offset = Some(offset);
                        }
                        "ext" => {
                            let extent_element = xml_doc_mut.get_element(*element_id).context(
                                "draviavemal-openxml_office::Failed to get extent element",
                            )?;
                            let mut extent = Extent::default();
                            if let Some(width) = extent_element.get_attribute("cx") {
                                extent.width = width.get_value().trim().parse().context(
                                    "draviavemal-openxml_office::Failed to parse extent width",
                                )?;
                            }
                            if let Some(height) = extent_element.get_attribute("cy") {
                                extent.height = height.get_value().trim().parse().context(
                                    "draviavemal-openxml_office::Failed to parse extent height",
                                )?;
                            }
                            transform.extent = Some(extent);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(transform)
    }
}

impl DrawingPartGlobal {
    pub(crate) fn add_picture_mut(
        &mut self,
        image_path: &str,
        picture_setting: PictureSetting,
    ) -> Result<&str, AnyError> {
        // Check File path exist
        let file_path = Path::new(image_path);
        let is_file_exist = file_path
            .try_exists()
            .context("Failed to validate provided image file path")?;
        if is_file_exist {
            let mut file = File::open(file_path).context("Failed to open Image File")?;
            let mut reader = BufReader::new(&file);
            let mut chunk_buffer = [0; 4096];
            let mut hasher = Hasher::new();
            loop {
                let bytes_read = reader
                    .read(&mut chunk_buffer)
                    .context("Failed to read image chunk")?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&chunk_buffer[..bytes_read]);
            }
            file.seek(SeekFrom::Start(0))?;
            let hash_signature = hasher.finalize().to_string();

            Ok("rd1")
        } else {
            Err(anyhow!("Provided image file path does not exist"))
        }
    }
}
