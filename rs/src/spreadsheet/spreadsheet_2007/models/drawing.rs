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
}

#[derive(Debug)]
pub(crate) struct Shape {}

#[derive(Debug)]
pub(crate) struct GroupShape {}

#[derive(Debug)]
pub(crate) struct GraphicFrame {
    id: u32,
    name: String,
    relationship_id: String,
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

#[derive(Debug)]
pub(crate) struct TwoCellAnchor {
    pub(crate) from: AnchorPosition,
    pub(crate) to: AnchorPosition,
    pub(crate) anchor_content: AnchorContent,
}
