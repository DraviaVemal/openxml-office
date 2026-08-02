use crate::global_2007::models::{AnchorPosition, GraphPosition};

#[derive(Debug)]
pub(crate) enum DrawingAnchor {
    AbsoluteAnchor(AbsoluteAnchor),
    OneCellAnchor(OneCellAnchor),
    TwoCellAnchor(TwoCellAnchor),
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
    pub(crate) relationship_id: String,
}

#[derive(Debug)]
pub(crate) struct ConnectorShape {}

#[derive(Debug)]
pub(crate) struct Picture {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) aspect_ratio: bool,
    pub(crate) relationship_id: String,
}

impl Default for Picture {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Picture 1".to_string(),
            aspect_ratio: true,
            relationship_id: "rId1".to_string(),
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
    pub(crate) from: AnchorPosition,
    pub(crate) to: AnchorPosition,
    pub(crate) anchor_content: AnchorContent,
    pub(crate) client_data: Option<AnchorClientData>,
}
