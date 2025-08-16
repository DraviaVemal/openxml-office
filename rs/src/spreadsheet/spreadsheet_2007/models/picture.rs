use crate::global_2007::models::{drawing::AnchorPosition, ExcelHyperlinkProperties, ImageType};

pub struct ExcelPictureSetting {
    pub hyperlink_properties: Option<ExcelHyperlinkProperties>,
    pub image_type: ImageType,
    pub from: AnchorPosition,
    pub to: AnchorPosition,
}

impl Default for ExcelPictureSetting {
    fn default() -> Self {
        Self {
            hyperlink_properties: None,
            image_type: ImageType::JPEG,
            from: AnchorPosition::default(),
            to: AnchorPosition::default(),
        }
    }
}
