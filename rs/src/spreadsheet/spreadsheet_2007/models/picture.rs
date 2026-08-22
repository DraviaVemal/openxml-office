use crate::{
    global_2007::models::{drawing::AnchorPosition, PictureSetting},
    spreadsheet_2007::models::ExcelHyperlinkProperties,
};

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
