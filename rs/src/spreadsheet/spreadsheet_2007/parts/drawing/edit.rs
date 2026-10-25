use anyhow::{Context, Error as AnyError, Result as AnyResult};

use crate::spreadsheet_2007::{models::ExcelPictureSetting, DrawingPart};

// ##################################### Mut Feature Function ################################
impl DrawingPart {
    pub fn add_picture_mut(
        &mut self,
        image_path: &str,
        picture_setting: ExcelPictureSetting,
    ) -> AnyResult<(), AnyError> {
        self.drawing_global
            .add_picture_mut(image_path, picture_setting.picture_setting)
            .context("Failed Global Image Handling")?;
        Ok(())
    }
}
