use crate::{
    global_2007::{parts::drawing::Picture, traits::Enum},
    spreadsheet_2007::parts::drawing::struct_items::{
        AbsoluteAnchor, ConnectorShape, ContentPart, GraphicFrame, GroupShape, OneCellAnchor,
        Shape, TwoCellAnchor,
    },
};

#[derive(Debug, Clone)]
pub enum ExcelHyperlinkPropertyTypeValues {
    EXISTING_FILE,
    WEB_URL,
    TARGET_SHEET,
}

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
        Self::TwoCell
    }
}

impl Enum<EditAsValues> for EditAsValues {
    fn get_string(edit_as: EditAsValues) -> String {
        match edit_as {
            EditAsValues::TwoCell => "twoCell".to_string(),
            EditAsValues::OneCell => "oneCell".to_string(),
            EditAsValues::Absolute => "absolute".to_string(),
        }
    }

    fn get_enum(value: &str) -> Self {
        match value {
            "oneCell" => Self::OneCell,
            "absolute" => Self::Absolute,
            _ => Self::TwoCell,
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
        Self::Picture(Picture::default())
    }
}
