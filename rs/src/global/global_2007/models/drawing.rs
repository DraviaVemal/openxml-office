use crate::global_2007::traits::Enum;

#[derive(Debug, Default)]
pub(crate) struct GraphPosition {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub(crate) struct AnchorPosition {
    pub column: u16,
    pub column_offset: u16,
    pub row: u32,
    pub row_offset: u32,
}

impl Default for AnchorPosition {
    fn default() -> Self {
        Self {
            column: 1,
            column_offset: 0,
            row: 1,
            row_offset: 0,
        }
    }
}

#[derive(Debug)]
pub enum ImageType {
    JPEG,
    PNG,
    GIF,
    BMP,
    TIFF,
}

impl Enum<ImageType> for ImageType {
    fn get_string(input_enum: ImageType) -> String {
        match input_enum {
            ImageType::BMP => "bmp".to_string(),
            ImageType::GIF => "gif".to_string(),
            ImageType::PNG => "png".to_string(),
            ImageType::TIFF => "tiff".to_string(),
            _ => "jpeg".to_string(),
        }
    }

    fn get_enum(input_string: &str) -> ImageType {
        match input_string {
            "bmp" => ImageType::BMP,
            "gif" => ImageType::GIF,
            "png" => ImageType::PNG,
            "tiff" => ImageType::TIFF,
            _ => ImageType::JPEG,
        }
    }
}
