use crate::global_2007::traits::Enum;

#[derive(Debug, Default)]
pub(crate) struct GraphPosition {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct AnchorPosition {
    pub column: u16,
    pub column_offset: u64,
    pub row: u32,
    pub row_offset: u64,
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

#[derive(Debug, Default)]
pub(crate) struct RelativeRect {
    pub(crate) left: Option<i32>,
    pub(crate) top: Option<i32>,
    pub(crate) right: Option<i32>,
    pub(crate) bottom: Option<i32>,
}

#[derive(Debug, Default)]
pub(crate) struct TileProperties {
    pub(crate) offset_x: Option<i64>,
    pub(crate) offset_y: Option<i64>,
    pub(crate) scale_x: Option<i32>,
    pub(crate) scale_y: Option<i32>,
    pub(crate) flip: Option<String>,
    pub(crate) alignment: Option<String>,
}

#[derive(Debug)]
pub(crate) enum BlipFillMode {
    Stretch(RelativeRect),
    Tile(TileProperties),
}

impl Default for BlipFillMode {
    fn default() -> Self {
        BlipFillMode::Stretch(RelativeRect::default())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Offset {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

#[derive(Debug, Default)]
pub(crate) struct Extent {
    pub(crate) width: i64,
    pub(crate) height: i64,
}

#[derive(Debug, Default)]
pub(crate) struct Transform {
    pub(crate) rotation: Option<i64>,
    pub(crate) flip_horizontal: bool,
    pub(crate) flip_vertical: bool,
    pub(crate) offset: Option<Offset>,
    pub(crate) extent: Option<Extent>,
}

#[derive(Debug, Default)]
pub(crate) struct PresetGeometry {
    pub(crate) preset: String,
}

#[derive(Debug, Default)]
pub(crate) struct ShapeProperties {
    pub(crate) transform: Option<Transform>,
    pub(crate) preset_geometry: Option<PresetGeometry>,
}

#[derive(Debug)]
pub(crate) struct Picture {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) hidden: bool,
    pub(crate) aspect_ratio: bool,
    pub(crate) relationship_id: String,
    pub(crate) compression_state: Option<String>,
    pub(crate) source_rectangle: Option<RelativeRect>,
    pub(crate) fill_mode: BlipFillMode,
    pub(crate) shape_properties: ShapeProperties,
}

impl Default for Picture {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Picture 1".to_string(),
            description: None,
            hidden: false,
            aspect_ratio: true,
            relationship_id: "rId1".to_string(),
            compression_state: None,
            source_rectangle: None,
            fill_mode: BlipFillMode::default(),
            shape_properties: ShapeProperties::default(),
        }
    }
}

#[derive(Debug)]
pub struct PictureSetting {
    pub image_type: ImageType,
}

impl Default for PictureSetting {
    fn default() -> Self {
        Self {
            image_type: ImageType::JPEG,
        }
    }
}
