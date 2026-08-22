use crate::global_2007::models::{AnchorPosition, GraphPosition, Picture, Transform};
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

#[derive(Debug, Clone)]
pub enum ExcelHyperlinkPropertyTypeValues {
    EXISTING_FILE,
    WEB_URL,
    TARGET_SHEET,
}
#[derive(Debug, Clone)]
pub struct ExcelHyperlinkProperties {
    pub display: Option<String>,
    pub link_type: ExcelHyperlinkPropertyTypeValues,
    pub link: String,
}
