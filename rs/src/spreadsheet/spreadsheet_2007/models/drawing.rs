use crate::global_2007::models::{AnchorPosition, GraphPosition};
use crate::global_2007::traits::Enum;

#[derive(Debug)]
pub(crate) enum DrawingAnchor {
    AbsoluteAnchor(AbsoluteAnchor),
    OneCellAnchor(OneCellAnchor),
    TwoCellAnchor(TwoCellAnchor),
}

#[derive(Debug)]
pub(crate) enum EditAsValues {
    TwoCell,
    OneCell,
    Absolute,
}

impl Default for EditAsValues {
    fn default() -> Self {
        EditAsValues::TwoCell
    }
}

impl Enum<EditAsValues> for EditAsValues {
    fn get_string(input_enum: EditAsValues) -> String {
        match input_enum {
            EditAsValues::TwoCell => "twoCell".to_string(),
            EditAsValues::OneCell => "oneCell".to_string(),
            EditAsValues::Absolute => "absolute".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> Self {
        match input_string {
            "oneCell" => EditAsValues::OneCell,
            "absolute" => EditAsValues::Absolute,
            _ => EditAsValues::TwoCell,
        }
    }
}

#[derive(Debug)]
pub(crate) enum AnchorContent {
    Shape(Shape),
    GroupShape(GroupShape),
    GraphicFrame(GraphicFrame),
    ConnectorShape(ConnectorShape),
    Picture(Picture),
    ContentPart(ContentPart),
}

impl Default for AnchorContent {
    fn default() -> Self {
        AnchorContent::Picture(Picture::default())
    }
}

#[derive(Debug)]
pub(crate) struct Shape {}

#[derive(Debug)]
pub(crate) struct GroupShape {}

#[derive(Debug, Default)]
pub(crate) struct Offset {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

#[derive(Debug, Default)]
pub(crate) struct Extent {
    pub(crate) width: i64,
    pub(crate) height: i64,
}

#[derive(Debug, Default)]
pub(crate) struct Transform {
    pub(crate) rotation: Option<i64>,
    pub(crate) flip_horizontal: bool,
    pub(crate) flip_vertical: bool,
    pub(crate) offset: Option<Offset>,
    pub(crate) extent: Option<Extent>,
}

#[derive(Debug, Default)]
pub(crate) struct PresetGeometry {
    pub(crate) preset: String,
}

#[derive(Debug, Default)]
pub(crate) struct ShapeProperties {
    pub(crate) transform: Option<Transform>,
    pub(crate) preset_geometry: Option<PresetGeometry>,
}

#[derive(Debug, Default)]
pub(crate) struct RelativeRect {
    pub(crate) left: Option<i32>,
    pub(crate) top: Option<i32>,
    pub(crate) right: Option<i32>,
    pub(crate) bottom: Option<i32>,
}

#[derive(Debug, Default)]
pub(crate) struct TileProperties {
    pub(crate) offset_x: Option<i64>,
    pub(crate) offset_y: Option<i64>,
    pub(crate) scale_x: Option<i32>,
    pub(crate) scale_y: Option<i32>,
    pub(crate) flip: Option<String>,
    pub(crate) alignment: Option<String>,
}

#[derive(Debug)]
pub(crate) enum BlipFillMode {
    Stretch(RelativeRect),
    Tile(TileProperties),
}

impl Default for BlipFillMode {
    fn default() -> Self {
        BlipFillMode::Stretch(RelativeRect::default())
    }
}

#[derive(Debug, Default)]
pub(crate) struct GraphicFrame {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) macro_reference: Option<String>,
    pub(crate) transform: Option<Transform>,
    pub(crate) graphic_uri: String,
    pub(crate) relationship_id: String,
}

#[derive(Debug)]
pub(crate) struct ConnectorShape {}

#[derive(Debug)]
pub(crate) struct Picture {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) hidden: bool,
    pub(crate) aspect_ratio: bool,
    pub(crate) relationship_id: String,
    pub(crate) compression_state: Option<String>,
    pub(crate) source_rectangle: Option<RelativeRect>,
    pub(crate) fill_mode: BlipFillMode,
    pub(crate) shape_properties: ShapeProperties,
}

impl Default for Picture {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Picture 1".to_string(),
            description: None,
            hidden: false,
            aspect_ratio: true,
            relationship_id: "rId1".to_string(),
            compression_state: None,
            source_rectangle: None,
            fill_mode: BlipFillMode::default(),
            shape_properties: ShapeProperties::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ContentPart {}

#[derive(Debug)]
pub(crate) struct AnchorClientData {}

#[derive(Debug)]
pub(crate) struct AbsoluteAnchor {
    pub(crate) pos: GraphPosition,
    pub(crate) anchor_content: AnchorContent,
}

#[derive(Debug)]
pub(crate) struct OneCellAnchor {
    pub(crate) from: AnchorPosition,
    pub(crate) anchor_content: AnchorContent,
}

#[derive(Debug, Default)]
pub(crate) struct TwoCellAnchor {
    pub(crate) edit_as: EditAsValues,
    pub(crate) from: AnchorPosition,
    pub(crate) to: AnchorPosition,
    pub(crate) anchor_content: AnchorContent,
    pub(crate) client_data: Option<AnchorClientData>,
}
