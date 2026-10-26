use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use crate::{
    global_2007::files::OfficeDocument,
    global_2007::parts::{
        AnchorPosition, DrawingPartGlobal, GraphPosition, Picture, PictureSetting, RelationsPart,
        Transform,
    },
    spreadsheet_2007::{
        parts::drawing::enum_items::{
            AnchorContent, DrawingAnchor, EditAsValues, ExcelHyperlinkPropertyTypeValues,
        },
        services::CommonServices,
    },
};
use draviavemal_xml_rs::XmlDocument;

#[derive(Debug, Clone)]
pub struct ExcelHyperlinkProperties {
    pub display: Option<String>,
    pub link_type: ExcelHyperlinkPropertyTypeValues,
    pub link: String,
}

#[derive(Debug)]
pub struct ExcelPictureSetting {
    pub picture_setting: PictureSetting,
    pub hyperlink_properties: Option<ExcelHyperlinkProperties>,
    pub from: AnchorPosition,
    pub to: AnchorPosition,
}

impl Default for ExcelPictureSetting {
    fn default() -> Self {
        Self {
            picture_setting: PictureSetting::default(),
            hyperlink_properties: None,
            from: AnchorPosition::default(),
            to: AnchorPosition::default(),
        }
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

#[derive(Debug)]
pub(crate) struct DrawingPart {
    pub(crate) drawing_global: DrawingPartGlobal,
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) common_service: Weak<RefCell<CommonServices>>,
    pub(crate) worksheet_relationship_part: Weak<RefCell<RelationsPart>>,
    pub(crate) drawing_relationship_part: Rc<RefCell<RelationsPart>>,
    pub(crate) anchor_collection: Option<VecDeque<DrawingAnchor>>,
    pub(crate) is_new: bool,
    pub(crate) file_path: String,
}
