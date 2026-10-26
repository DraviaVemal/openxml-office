use anyhow::{Context, Error as AnyError, Result as AnyResult};

use super::{AnchorContent, DrawingAnchor, TwoCellAnchor};
use crate::spreadsheet_2007::{ExcelPictureSetting, DrawingPart};

// ##################################### Mut Feature Function ################################
impl DrawingPart {
    pub fn add_picture_mut(
        &mut self,
        image_path: &str,
        picture_setting: ExcelPictureSetting,
    ) -> AnyResult<(), AnyError> {
        let mut picture = self
            .drawing_global
            .add_picture_mut(image_path, picture_setting.picture_setting)
            .context("Failed Global Image Handling")?;
        let next_picture_id = self
            .anchor_collection
            .as_ref()
            .into_iter()
            .flatten()
            .filter_map(|anchor| match anchor {
                DrawingAnchor::AbsoluteAnchor(anchor) => match &anchor.anchor_content {
                    AnchorContent::Picture(picture) => Some(picture.id),
                    _ => None,
                },
                DrawingAnchor::OneCellAnchor(anchor) => match &anchor.anchor_content {
                    AnchorContent::Picture(picture) => Some(picture.id),
                    _ => None,
                },
                DrawingAnchor::TwoCellAnchor(anchor) => match &anchor.anchor_content {
                    AnchorContent::Picture(picture) => Some(picture.id),
                    _ => None,
                },
            })
            .max()
            .unwrap_or(0)
            + 1;
        picture.id = next_picture_id;
        picture.name = format!("Picture {}", next_picture_id);
        self.anchor_collection
            .get_or_insert_with(Default::default)
            .push_back(DrawingAnchor::TwoCellAnchor(TwoCellAnchor {
                from: picture_setting.from,
                to: picture_setting.to,
                anchor_content: AnchorContent::Picture(picture),
                ..TwoCellAnchor::default()
            }));
        Ok(())
    }
}
