use crate::global_2007::{
    parts::drawing::struct_items::{RelativeRect, TileProperties},
    traits::Enum,
};

#[derive(Debug)]
pub enum ImageType {
    JPEG,
    PNG,
    GIF,
    BMP,
    TIFF,
}

impl Enum<ImageType> for ImageType {
    fn get_string(image_type: ImageType) -> String {
        match image_type {
            ImageType::BMP => "bmp".to_string(),
            ImageType::GIF => "gif".to_string(),
            ImageType::PNG => "png".to_string(),
            ImageType::TIFF => "tiff".to_string(),
            _ => "jpeg".to_string(),
        }
    }

    fn get_enum(extension: &str) -> ImageType {
        match extension {
            "bmp" => ImageType::BMP,
            "gif" => ImageType::GIF,
            "png" => ImageType::PNG,
            "tiff" => ImageType::TIFF,
            _ => ImageType::JPEG,
        }
    }
}

#[derive(Debug)]
pub(crate) enum BlipFillMode {
    Stretch(RelativeRect),
    Tile(TileProperties),
}

impl Default for BlipFillMode {
    fn default() -> Self {
        Self::Stretch(RelativeRect::default())
    }
}
