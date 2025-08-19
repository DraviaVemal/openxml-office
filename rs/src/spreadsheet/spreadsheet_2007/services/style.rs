use crate::{
    converters::ConverterUtil,
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{
            Enum, XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush,
            XmlDocumentPartInitializing,
        },
    },
    log_elapsed,
    spreadsheet_2007::models::{
        BorderSetting, BorderStyle, BorderStyleValues, CellXfs, ColorSetting,
        ColorSettingTypeValues, FillStyle, FontSchemeValues, FontStyle, HorizontalAlignmentValues,
        NumberFormat, NumberFormatValues, PatternTypeValues, StyleId, StyleSetting,
        VerticalAlignmentValues,
    },
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{
    NodeId, XmlAttribute, XmlDeserializer, XmlDocument, XmlElement, XmlElementContentType,
};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
    rc::Weak,
};

#[derive(Debug)]
pub struct StylePart {
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
    cache_id: HashMap<u64, u32>,
    cache_order: VecDeque<u64>,
    cache_capacity: u8,
    number_format_collection: Vec<(u64, NumberFormat)>,
    font_collection: Vec<(u64, FontStyle)>,
    fill_collection: Vec<(u64, FillStyle)>,
    border_collection: Vec<(u64, BorderStyle)>,
    cell_style_xfs_collection: Vec<(u64, CellXfs)>,
    cell_xfs_collection: Vec<(u64, CellXfs)>,
}

impl Drop for StylePart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for StylePart {}

impl XmlDocumentPartClose for StylePart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                log_elapsed!(
                    || {
                        {
                            self.save_content_to_tree_mut()
                                .context("Style Save Content Failed")
                        }
                    },
                    "Save Tree"
                )?;
                if let Some(xml_tree) = self.office_document.upgrade() {
                    log_elapsed!(
                        || {
                            xml_tree
                                .try_borrow_mut()
                                .context("Failed To pull XML Handle")?
                                .close_xml_document(&self.file_path)
                        },
                        "Close Document"
                    )?;
                }
                Ok(())
            },
            "Close Style Service"
        )
    }
}

impl XmlDocumentPartInitializing for StylePart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION.get("style").unwrap();
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(include_str!("style.xml").as_bytes().to_vec())
                .context("Initializing Theme Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

impl XmlDocumentPart for StylePart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<Self, AnyError> {
        let file_name = Self::get_style_file_name(&parent_relationship_part)
            .context("Failed to pull style file name")?
            .to_string();
        let mut xml_document = Self::get_xml_document(&office_document, &file_name)?;
        let (
            number_format_collection,
            font_collection,
            fill_collection,
            border_collection,
            cell_style_collection,
            cell_collection,
        ) = Self::deserialize_content(&mut xml_document)
            .context("Load Share String To Object Failed")?;
        Ok(Self {
            office_document,
            xml_document,
            file_path: file_name,
            cache_id: HashMap::new(),
            cache_order: VecDeque::new(),
            cache_capacity: 25,
            number_format_collection,
            font_collection,
            fill_collection,
            border_collection,
            cell_style_xfs_collection: cell_style_collection,
            cell_xfs_collection: cell_collection,
        })
    }
}

// ################################# Load / Save Functions ################
impl StylePart {
    fn get_style_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let style_content = EXCEL_TYPE_COLLECTION.get("style").unwrap();
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &style_content.schemas_type,
                    style_content,
                    None,
                    None,
                )
                .context("Pull Path From Existing File Failed")?)
        } else {
            Err(anyhow!("Failed to upgrade relation part"))
        }
    }

    /// Load existing file style to object  
    fn deserialize_content(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<
        (
            Vec<(u64, NumberFormat)>,
            Vec<(u64, FontStyle)>,
            Vec<(u64, FillStyle)>,
            Vec<(u64, BorderStyle)>,
            Vec<(u64, CellXfs)>,
            Vec<(u64, CellXfs)>,
        ),
        AnyError,
    > {
        let mut num_format_records = Vec::new();
        let mut font_records = Vec::new();
        let mut fill_records = Vec::new();
        let mut border_records = Vec::new();
        let mut style_collection = Vec::new();
        let mut xfs_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("xml doc borrow failed")?;
            let root_id = xml_doc_mut.get_root_id();
            if let Some(number_format_id) = xml_doc_mut
                .find_first_child(root_id, "numFmts")
                .context("Failed to get number format element")?
            {
                if let Some(num_fmt_ids) = xml_doc_mut
                    .find_all_child(number_format_id, "numFmt")
                    .context("Failed to get element group")?
                {
                    for num_fmt_id in num_fmt_ids {
                        let mut number_format = NumberFormat::default();
                        let num_fmt_element = xml_doc_mut
                            .get_element(num_fmt_id)
                            .context("Failed to get number format element")?;
                        number_format.format_id = num_fmt_element
                            .get_attribute("numFmtId")
                            .context("numFmtId Attribute Not Found!")?
                            .get_value()
                            .parse()
                            .context("Number format ID parsing Failed")?;
                        number_format.format_code = num_fmt_element
                            .get_attribute("formatCode")
                            .context("formatCode Attribute Not Found!")?
                            .get_value()
                            .to_owned();
                        let mut hasher = DefaultHasher::new();
                        number_format.hash(&mut hasher);
                        num_format_records.push((hasher.finish(), number_format));
                    }
                }
                xml_doc_mut
                    .remove_element_mut(number_format_id)
                    .context("Failed to clear numFmt element childs")?;
            }
            if let Some(fonts_id) = xml_doc_mut
                .find_first_child(root_id, "fonts")
                .context("Failed to get fonts element")?
            {
                if let Some(font_ids) = xml_doc_mut
                    .find_all_child(fonts_id, "font")
                    .context("Failed to get font element group")?
                {
                    for font_id in font_ids {
                        if let Some(font_element_childs) = xml_doc_mut
                            .get_element(font_id)
                            .context("Failed to get font element")?
                            .get_child_contents()
                        {
                            let mut font_style = FontStyle::default();
                            for font_element_child in font_element_childs {
                                match font_element_child {
                                    XmlElementContentType::Element((id, _, _)) => {
                                        let current_element = xml_doc_mut
                                            .get_element(*id)
                                            .context("Failed to get child element")?;
                                        match current_element.get_tag().as_str() {
                                            "b" => font_style.is_bold = true,
                                            "u" => {
                                                if let Some(double) =
                                                    current_element.get_attribute("val")
                                                {
                                                    if double.get_value() == "double" {
                                                        font_style.is_double_underline = true;
                                                    }
                                                }
                                                font_style.is_underline = false;
                                            }
                                            "i" => font_style.is_italic = false,
                                            "sz" => {
                                                if let Some(val) =
                                                    current_element.get_attribute("val")
                                                {
                                                    font_style.size = val
                                                        .get_value()
                                                        .parse()
                                                        .context("Font Size Parse Failed")?
                                                }
                                            }
                                            "color" => {
                                                if let Some(theme) =
                                                    current_element.get_attribute("theme")
                                                {
                                                    font_style.color.color_setting_type =
                                                        ColorSettingTypeValues::Theme;
                                                    font_style.color.value = theme
                                                        .get_value()
                                                        .parse()
                                                        .context("Font color theme parse failed")?
                                                } else if let Some(rgb) =
                                                    current_element.get_attribute("rgb")
                                                {
                                                    font_style.color.color_setting_type =
                                                        ColorSettingTypeValues::Rgb;
                                                    let rgb_string = rgb.get_value().to_string();
                                                    font_style.color.value = rgb_string;
                                                } else if let Some(indexed) =
                                                    current_element.get_attribute("indexed")
                                                {
                                                    font_style.color.color_setting_type =
                                                        ColorSettingTypeValues::Indexed;
                                                    let indexed_string =
                                                        indexed.get_value().to_string();
                                                    font_style.color.value = indexed_string;
                                                }
                                            }
                                            "name" => {
                                                if let Some(val) =
                                                    current_element.get_attribute("val")
                                                {
                                                    font_style.name = val.get_value().to_string()
                                                }
                                            }
                                            "family" => {
                                                if let Some(val) =
                                                    current_element.get_attribute("val")
                                                {
                                                    font_style.family = val
                                                        .get_value()
                                                        .parse()
                                                        .context("Font Size Parse Failed")?
                                                }
                                            }
                                            "scheme" => {
                                                if let Some(val) =
                                                    current_element.get_attribute("val")
                                                {
                                                    font_style.font_scheme =
                                                        FontSchemeValues::get_enum(val.get_value())
                                                }
                                            }
                                            _ => {
                                                return Err(anyhow!("Unknown Font Style Found!"));
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(anyhow!("Unknown Element type"));
                                    }
                                }
                            }
                            let mut hasher = DefaultHasher::new();
                            font_style.hash(&mut hasher);
                            font_records.push((hasher.finish(), font_style));
                        }
                    }
                }
                xml_doc_mut
                    .remove_element_mut(fonts_id)
                    .context("Failed to clear fonts element childs")?;
            }

            if let Some(fills_id) = xml_doc_mut
                .find_first_child(root_id, "fills")
                .context("Failed to get fills element")?
            {
                if let Some(fill_ids) = xml_doc_mut
                    .find_all_child(fills_id, "fill")
                    .context("Failed to get fill group")?
                {
                    for fill_id in fill_ids {
                        let mut fill_style = FillStyle::default();
                        if let Some(pattern_fill_id) = xml_doc_mut
                            .find_first_child(fill_id, "patternFill")
                            .context("Failed to pull pattern fill element")?
                        {
                            let pattern_fill_element = xml_doc_mut
                                .get_element(pattern_fill_id)
                                .context("Failed to pull Pattern fill element")?;
                            fill_style.pattern_type = PatternTypeValues::get_enum(
                                pattern_fill_element
                                    .get_attribute("patternType")
                                    .context(
                                        "Failed to get mandatory patternType from patternFill",
                                    )?
                                    .get_value(),
                            );
                            if let Some(child_contents) = pattern_fill_element.get_child_contents()
                            {
                                for fill_child in child_contents {
                                    match fill_child {
                                        XmlElementContentType::Element((id, _, _)) => {
                                            let current_element = xml_doc_mut
                                                .get_element(*id)
                                                .context("Failed to get child element")?;
                                            match current_element.get_tag().as_str() {
                                                "fgColor" => {
                                                    if let Some(theme) =
                                                        current_element.get_attribute("theme")
                                                    {
                                                        fill_style.foreground_color =
                                                            Some(ColorSetting {
                                                                color_setting_type:
                                                                    ColorSettingTypeValues::Theme,
                                                                value: theme
                                                                    .get_value()
                                                                    .parse()
                                                                    .context(
                                                                        "color theme parse failed",
                                                                    )?,
                                                            });
                                                    } else if let Some(rgb) =
                                                        current_element.get_attribute("rgb")
                                                    {
                                                        let rgb_string =
                                                            rgb.get_value().to_string();
                                                        fill_style.foreground_color =
                                                            Some(ColorSetting {
                                                                color_setting_type:
                                                                    ColorSettingTypeValues::Rgb,
                                                                value: rgb_string,
                                                            });
                                                    } else if let Some(indexed) =
                                                        current_element.get_attribute("indexed")
                                                    {
                                                        fill_style.foreground_color =
                                                                        Some(ColorSetting {
                                                                            color_setting_type:ColorSettingTypeValues::Indexed,
                                                                            value:indexed
                                                                            .get_value()
                                                                            .parse()
                                                                            .context("color color index parse failed")?
                                                                        });
                                                    }
                                                }
                                                "bgColor" => {
                                                    if let Some(theme) =
                                                        current_element.get_attribute("theme")
                                                    {
                                                        fill_style.background_color =
                                                            Some(ColorSetting {
                                                                color_setting_type:
                                                                    ColorSettingTypeValues::Theme,
                                                                value: theme
                                                                    .get_value()
                                                                    .parse()
                                                                    .context(
                                                                        "color theme parse failed",
                                                                    )?,
                                                            });
                                                    } else if let Some(rgb) =
                                                        current_element.get_attribute("rgb")
                                                    {
                                                        let rgb_string =
                                                            rgb.get_value().to_string();
                                                        fill_style.background_color =
                                                            Some(ColorSetting {
                                                                color_setting_type:
                                                                    ColorSettingTypeValues::Rgb,
                                                                value: rgb_string,
                                                            });
                                                    } else if let Some(indexed) =
                                                        current_element.get_attribute("indexed")
                                                    {
                                                        fill_style
                                                                        .background_color =
                                                                        Some(ColorSetting {
                                                                            color_setting_type:ColorSettingTypeValues::Indexed,
                                                                            value:indexed
                                                                            .get_value()
                                                                            .parse()
                                                                            .context("color color index parse failed")?
                                                                        });
                                                    }
                                                }
                                                _ => {
                                                    return Err(anyhow!(
                                                        "Unknown Color patter found"
                                                    ));
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        let mut hasher = DefaultHasher::new();
                        fill_style.hash(&mut hasher);
                        fill_records.push((hasher.finish(), fill_style));
                    }
                }
                xml_doc_mut
                    .remove_element_mut(fills_id)
                    .context("Failed to clear fills element childs")?;
            }

            if let Some(borders_id) = xml_doc_mut
                .find_first_child(root_id, "borders")
                .context("Failed to pull Borders Element")?
            {
                if let Some(border_ids) = xml_doc_mut
                    .find_all_child(borders_id, "border")
                    .context("Failed to get border group")?
                {
                    for border_id in border_ids {
                        let mut border_style = BorderStyle::default();
                        let border_element = xml_doc_mut
                            .get_element(border_id)
                            .context("Failed to get border element")?;
                        if let Some(child_content) = border_element.get_child_contents() {
                            for content_type in child_content {
                                match content_type {
                                    XmlElementContentType::Element((id, _, _)) => {
                                        let current_element = xml_doc_mut
                                            .get_element(*id)
                                            .context("Failed to get Child element of border")?;
                                        match current_element.get_tag().as_str() {
                                            "left" => {
                                                StylePart::deserialize_border_setting(
                                                    &current_element,
                                                    &mut border_style.left,
                                                    &xml_doc_mut,
                                                )
                                                .context("Left Border Decode Failed")?;
                                            }
                                            "right" => {
                                                StylePart::deserialize_border_setting(
                                                    &current_element,
                                                    &mut border_style.right,
                                                    &xml_doc_mut,
                                                )
                                                .context("Left Border Decode Failed")?;
                                            }
                                            "top" => {
                                                StylePart::deserialize_border_setting(
                                                    &current_element,
                                                    &mut border_style.top,
                                                    &xml_doc_mut,
                                                )
                                                .context("Left Border Decode Failed")?;
                                            }
                                            "bottom" => {
                                                StylePart::deserialize_border_setting(
                                                    &current_element,
                                                    &mut border_style.bottom,
                                                    &xml_doc_mut,
                                                )
                                                .context("Left Border Decode Failed")?;
                                            }
                                            "diagonal" => {
                                                StylePart::deserialize_border_setting(
                                                    &current_element,
                                                    &mut border_style.diagonal,
                                                    &xml_doc_mut,
                                                )
                                                .context("Left Border Decode Failed")?;
                                            }
                                            _ => {
                                                return Err(AnyError::msg(
                                                    "Unknow Content type found in border",
                                                ));
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(AnyError::msg(
                                            "Unknow Content type found in border",
                                        ));
                                    }
                                }
                            }
                            let mut hasher = DefaultHasher::new();
                            border_style.hash(&mut hasher);
                            border_records.push((hasher.finish(), border_style));
                        }
                    }
                }
                xml_doc_mut
                    .remove_element_mut(borders_id)
                    .context("Failed to clear borders element childs")?;
            }

            if let Some(cell_style_xfs_id) = xml_doc_mut
                .find_first_child(root_id, "cellStyleXfs")
                .context("Failed to get <cellStyleXfs>element")?
            {
                style_collection =
                    StylePart::deserialize_cell_style(cell_style_xfs_id, &xml_doc_mut)
                        .context("Deserializing Cell Style Xfs Failed")?;
                xml_doc_mut
                    .remove_element_mut(cell_style_xfs_id)
                    .context("Failed to clear cellStyleXfs element childs")?;
            }

            if let Some(cell_xfs_id) = xml_doc_mut
                .find_first_child(root_id, "cellXfs")
                .context("Failed to get <cellXfs>element")?
            {
                xfs_collection = StylePart::deserialize_cell_style(cell_xfs_id, &xml_doc_mut)
                    .context("Deserializing Cell Xfs Failed")?;
                xml_doc_mut
                    .remove_element_mut(cell_xfs_id)
                    .context("Failed to clear cellXfs element childs")?;
            }
        }
        Ok((
            num_format_records,
            font_records,
            fill_records,
            border_records,
            style_collection,
            xfs_collection,
        ))
    }

    /// Save Object record back to XML File
    fn save_content_to_tree_mut(&mut self) -> AnyResult<(), AnyError> {
        if let Some(xml_document) = self.xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("xml doc borrow failed")?;
            let root_id = xml_doc_mut.get_root_id();
            // Create Number Formats Elements
            {
                let num_formats_id = xml_doc_mut
                    .append_child_element_mut(root_id, "numFmts", None)
                    .context("Failed to Create numFmts")?;
                let num_formats_element = xml_doc_mut
                    .get_element_mut(num_formats_id)
                    .context("Failed to get number format element")?;
                num_formats_element
                    .add_attribute_mut(XmlAttribute::new(
                        "count".to_owned(),
                        self.number_format_collection.len().to_string(),
                    ))
                    .context("Updating Number Formats Element Attributes Failed")?;
                for (_, num_format) in self.number_format_collection.as_slice() {
                    xml_doc_mut
                        .append_child_element_mut(
                            num_formats_id,
                            "numFmt",
                            Some(vec![
                                XmlAttribute::new(
                                    "numFmtId".to_string(),
                                    num_format.format_id.to_string(),
                                ),
                                XmlAttribute::new(
                                    "formatCode".to_string(),
                                    num_format.format_code.clone(),
                                ),
                            ]),
                        )
                        .context("Create Number Format Element Failed")?;
                }
            }
            // Create Fonts Elements
            {
                let fonts_id = xml_doc_mut
                    .append_child_element_mut(root_id, "fonts", None)
                    .context("Failed to create fonts element")?;
                let fonts_element = xml_doc_mut
                    .get_element_mut(fonts_id)
                    .context("Failed to get fonts element")?;
                fonts_element
                    .add_attribute_mut(XmlAttribute::new(
                        "count".to_owned(),
                        self.font_collection.len().to_string(),
                    ))
                    .context("Updating font Formats Element Attributes Failed")?;
                for (_, font_style) in self.font_collection.as_slice() {
                    let font_id = xml_doc_mut
                        .append_child_element_mut(fonts_id, "font", None)
                        .context("Adding Font to Fonts Failed")?;
                    if font_style.is_bold {
                        xml_doc_mut
                            .append_child_element_mut(font_id, "b", None)
                            .context("Create Size Failed")?;
                    }
                    if font_style.is_italic {
                        xml_doc_mut
                            .append_child_element_mut(font_id, "i", None)
                            .context("Create Size Failed")?;
                    }
                    if font_style.is_underline {
                        xml_doc_mut
                            .append_child_element_mut(font_id, "u", None)
                            .context("Create Size Failed")?;
                    }
                    if font_style.is_double_underline {
                        xml_doc_mut
                            .append_child_element_mut(
                                font_id,
                                "u",
                                Some(vec![XmlAttribute::new(
                                    "val".to_string(),
                                    "double".to_string(),
                                )]),
                            )
                            .context("Create Size Failed")?;
                    }
                    xml_doc_mut
                        .append_child_element_mut(
                            font_id,
                            "sz",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.size.to_string(),
                            )]),
                        )
                        .context("Create Size Failed")?;
                    StylePart::add_color_element(
                        Some(font_style.color.clone()),
                        &mut xml_doc_mut,
                        font_id,
                    )?;
                    xml_doc_mut
                        .append_child_element_mut(
                            font_id,
                            "name",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.name.clone(),
                            )]),
                        )
                        .context("Create Name Failed")?;
                    xml_doc_mut
                        .append_child_element_mut(
                            font_id,
                            "family",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.family.to_string(),
                            )]),
                        )
                        .context("Create Family Failed")?;
                    xml_doc_mut
                        .append_child_element_mut(
                            font_id,
                            "scheme",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                FontSchemeValues::get_string(font_style.font_scheme.clone()),
                            )]),
                        )
                        .context("Create scheme Failed")?;
                }
            }
            // Create Fills Elements
            {
                let fills_id = xml_doc_mut
                    .append_child_element_mut(root_id, "fills", None)
                    .context("Failed to create fills element")?;
                let fills_element = xml_doc_mut
                    .get_element_mut(fills_id)
                    .context("Failed to get fills element")?;
                fills_element
                    .add_attribute_mut(XmlAttribute::new(
                        "count".to_owned(),
                        self.fill_collection.len().to_string(),
                    ))
                    .context("Updating Fills Element Attributes Failed")?;
                for (_, fill_data) in self.fill_collection.as_slice() {
                    let fill_id = xml_doc_mut
                        .append_child_element_mut(fills_id, "fill", None)
                        .context("Adding Fill Element Failed")?;
                    let pattern_fill_id = xml_doc_mut
                        .append_child_element_mut(
                            fill_id,
                            "patternFill",
                            Some(vec![XmlAttribute::new(
                                "patternType".to_string(),
                                PatternTypeValues::get_string(fill_data.pattern_type.clone()),
                            )]),
                        )
                        .context("Pattern Fill Element Failed")?;
                    if let Some(fg_setting) = fill_data.foreground_color.clone() {
                        xml_doc_mut
                            .append_child_element_mut(
                                pattern_fill_id,
                                "fgColor",
                                Some(vec![XmlAttribute::new(
                                    ColorSettingTypeValues::get_string(
                                        fg_setting.color_setting_type,
                                    ),
                                    fg_setting.value,
                                )]),
                            )
                            .context("Pattern Fill Foreground Element Failed")?;
                    }
                    if let Some(bg_setting) = fill_data.background_color.clone() {
                        xml_doc_mut
                            .append_child_element_mut(
                                pattern_fill_id,
                                "bgColor",
                                Some(vec![XmlAttribute::new(
                                    ColorSettingTypeValues::get_string(
                                        bg_setting.color_setting_type,
                                    ),
                                    bg_setting.value,
                                )]),
                            )
                            .context("Pattern Fill Background Element Failed")?;
                    }
                }
            }
            // Create Border Elements
            {
                let borders_id = xml_doc_mut
                    .append_child_element_mut(root_id, "borders", None)
                    .context("Failed to create borders element")?;
                let borders_element = xml_doc_mut
                    .get_element_mut(borders_id)
                    .context("Failed to get border element")?;
                borders_element
                    .add_attribute_mut(XmlAttribute::new(
                        "count".to_owned(),
                        self.border_collection.len().to_string(),
                    ))
                    .context("Updating Fills Element Attributes Failed")?;
                for (_, border_data) in self.border_collection.as_slice() {
                    let border_id = xml_doc_mut
                        .append_child_element_mut(borders_id, "border", None)
                        .context("Create Border Failed")?;
                    // Left Border Setting
                    StylePart::add_border_element(
                        "left",
                        &mut xml_doc_mut,
                        &border_id,
                        border_data.left.clone(),
                    )?;
                    //Right Border Setting
                    StylePart::add_border_element(
                        "right",
                        &mut xml_doc_mut,
                        &border_id,
                        border_data.right.clone(),
                    )?;
                    // Top Border Setting
                    StylePart::add_border_element(
                        "top",
                        &mut xml_doc_mut,
                        &border_id,
                        border_data.top.clone(),
                    )?;
                    // Bottom Border Setting
                    StylePart::add_border_element(
                        "bottom",
                        &mut xml_doc_mut,
                        &border_id,
                        border_data.bottom.clone(),
                    )?;
                    // Diagonal Border Setting
                    StylePart::add_border_element(
                        "diagonal",
                        &mut xml_doc_mut,
                        &border_id,
                        border_data.diagonal.clone(),
                    )?;
                }
            }
            // Create Cell Style Elements
            {
                StylePart::add_cell_style(&mut xml_doc_mut, &mut self.cell_xfs_collection, true)?;
            }
            // Create Defined Cell Style Elements
            {
                StylePart::add_cell_style(
                    &mut xml_doc_mut,
                    &mut self.cell_style_xfs_collection,
                    false,
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn deserialize_cell_style(
        cell_style_xfs_id: NodeId,
        xml_document: &XmlDocument,
    ) -> AnyResult<Vec<(u64, CellXfs)>, AnyError> {
        let mut style_records = Vec::new();
        let cell_style_xfs = xml_document
            .get_element(cell_style_xfs_id)
            .context("Failed to get cell Style xfs")?;
        if let Some(child_contents) = cell_style_xfs.get_child_contents() {
            for child_content in child_contents {
                match child_content {
                    XmlElementContentType::Element((cell_xf_id, _, _)) => {
                        let cell_xf_element = xml_document
                            .get_element(*cell_xf_id)
                            .context("Failed to get cellxf element")?;
                        let mut cell_xf = CellXfs::default();
                        if let Some(number_format_id) = cell_xf_element.get_attribute("numFmtId") {
                            cell_xf.number_format_id = number_format_id
                                .get_value()
                                .parse()
                                .context("Number Number Format Id Parse Failed")?;
                        }
                        if let Some(font_id) = cell_xf_element.get_attribute("fontId") {
                            cell_xf.font_id = font_id
                                .get_value()
                                .parse()
                                .context("Number Font Id Parse Failed")?;
                        }
                        if let Some(fill_id) = cell_xf_element.get_attribute("fillId") {
                            cell_xf.fill_id = fill_id
                                .get_value()
                                .parse()
                                .context("Number Fill Id Parse Failed")?;
                        }
                        if let Some(border_id) = cell_xf_element.get_attribute("borderId") {
                            cell_xf.border_id = border_id
                                .get_value()
                                .parse()
                                .context("Number Border Id Parse Failed")?;
                        }
                        if let Some(format_id) = cell_xf_element.get_attribute("xfId") {
                            cell_xf.format_id = format_id
                                .get_value()
                                .parse()
                                .context("Number Format Id Parse Failed")?;
                        }
                        if let Some(apply_protection) =
                            cell_xf_element.get_attribute("applyProtection")
                        {
                            cell_xf.apply_protection = ConverterUtil::normalize_bool_property_u8(
                                apply_protection.get_value(),
                            );
                        }
                        if let Some(apply_alignment) =
                            cell_xf_element.get_attribute("applyAlignment")
                        {
                            cell_xf.apply_alignment = ConverterUtil::normalize_bool_property_u8(
                                apply_alignment.get_value(),
                            );
                        }
                        if let Some(apply_border) = cell_xf_element.get_attribute("applyBorder") {
                            cell_xf.apply_border =
                                ConverterUtil::normalize_bool_property_u8(apply_border.get_value());
                        }
                        if let Some(apply_fill) = cell_xf_element.get_attribute("applyFill") {
                            cell_xf.apply_fill =
                                ConverterUtil::normalize_bool_property_u8(apply_fill.get_value());
                        }
                        if let Some(apply_font) = cell_xf_element.get_attribute("applyFont") {
                            cell_xf.apply_font =
                                ConverterUtil::normalize_bool_property_u8(apply_font.get_value());
                        }
                        if let Some(apply_number_format) =
                            cell_xf_element.get_attribute("applyNumberFormat")
                        {
                            cell_xf.apply_number_format = ConverterUtil::normalize_bool_property_u8(
                                apply_number_format.get_value(),
                            );
                        }
                        // Load Alignment Values if exist
                        if let Some(child_contents) = cell_xf_element.get_child_contents() {
                            for child_content in child_contents {
                                match child_content {
                                    XmlElementContentType::Element((alignment_id, _, _)) => {
                                        let alignment_element = xml_document
                                            .get_element(*alignment_id)
                                            .context("Failed to get Alignment Element")?;
                                        if let Some(is_wrap_text) =
                                            alignment_element.get_attribute("wrapText")
                                        {
                                            cell_xf.is_wrap_text =
                                                ConverterUtil::normalize_bool_property_u8(
                                                    is_wrap_text.get_value(),
                                                );
                                        }
                                        if let Some(vertical_alignment) =
                                            alignment_element.get_attribute("vertical")
                                        {
                                            cell_xf.vertical_alignment =
                                                VerticalAlignmentValues::get_enum(
                                                    vertical_alignment.get_value(),
                                                );
                                        }
                                        if let Some(horizontal_alignment) =
                                            alignment_element.get_attribute("horizontal")
                                        {
                                            cell_xf.horizontal_alignment =
                                                HorizontalAlignmentValues::get_enum(
                                                    horizontal_alignment.get_value(),
                                                );
                                        }
                                    }
                                    _ => {
                                        return Err(AnyError::msg(
                                            "Unknown alignment content type",
                                        ));
                                    }
                                }
                            }
                        }
                        let mut hasher = DefaultHasher::new();
                        cell_xf.hash(&mut hasher);
                        style_records.push((hasher.finish(), cell_xf));
                    }
                    _ => {
                        return Err(AnyError::msg(
                            "Unknow processed content type found in element tree",
                        ));
                    }
                }
            }
        }
        Ok(style_records)
    }

    pub(crate) fn deserialize_border_setting(
        current_element: &XmlElement,
        border: &mut BorderSetting,
        xml_doc_mut: &XmlDocument,
    ) -> Result<(), AnyError> {
        if let Some(style) = current_element.get_attribute("style") {
            border.style = BorderStyleValues::get_enum(style.get_value());
            if border.style != BorderStyleValues::None {
                if let Some(child_content) = current_element.get_child_contents() {
                    for content in child_content {
                        match content {
                            XmlElementContentType::Element((id, _, _)) => {
                                let color_element = xml_doc_mut
                                    .get_element(*id)
                                    .context("Failed to get Color element")?;
                                if let Some(theme) = color_element.get_attribute("theme") {
                                    border.border_color = Some(ColorSetting {
                                        color_setting_type: ColorSettingTypeValues::Theme,
                                        value: theme
                                            .get_value()
                                            .parse()
                                            .context("color theme parse failed")?,
                                    });
                                } else if let Some(rgb) = color_element.get_attribute("rgb") {
                                    let rgb_string = rgb.get_value().to_string();
                                    border.border_color = Some(ColorSetting {
                                        color_setting_type: ColorSettingTypeValues::Rgb,
                                        value: rgb_string,
                                    });
                                } else if let Some(indexed) = color_element.get_attribute("indexed")
                                {
                                    border.border_color = Some(ColorSetting {
                                        color_setting_type: ColorSettingTypeValues::Indexed,
                                        value: indexed
                                            .get_value()
                                            .parse()
                                            .context("color color index parse failed")?,
                                    });
                                }
                            }
                            _ => {
                                return Err(AnyError::msg(
                                    "Unknow child element in <border> direction",
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Add Color Element Node To XML
    fn add_color_element(
        color_setting: Option<ColorSetting>,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        parent_id: u32,
    ) -> Result<(), AnyError> {
        Ok(if let Some(border_color_setting) = color_setting {
            xml_doc_mut
                .append_child_element_mut(
                    parent_id,
                    "color",
                    Some(vec![XmlAttribute::new(
                        ColorSettingTypeValues::get_string(border_color_setting.color_setting_type),
                        border_color_setting.value,
                    )]),
                )
                .context("Create Color Element Failed")?;
        })
    }

    /// Add Border Element Node to XML
    fn add_border_element(
        border_side: &str,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        border_id: &u32,
        border_data: BorderSetting,
    ) -> Result<(), AnyError> {
        let id = xml_doc_mut
            .append_child_element_mut(*border_id, border_side, None)
            .context(format!("{} Border Element Creation Failed", border_side))?;
        if border_data.style != BorderStyleValues::None {
            StylePart::add_color_element(border_data.border_color, xml_doc_mut, id)?;
            xml_doc_mut
                .get_element_mut(id)
                .context("Get Border Element Failed")?
                .add_attribute_mut(XmlAttribute::new(
                    "style".to_string(),
                    BorderStyleValues::get_string(border_data.style),
                ));
        }
        Ok(())
    }

    fn add_cell_style(
        xml_doc_mut: &mut XmlDocument,
        xfs_data: &mut Vec<(u64, CellXfs)>,
        enable_format_id: bool,
    ) -> Result<(), AnyError> {
        let root_id = xml_doc_mut.get_root_id();
        let cell_style_xfs_id = xml_doc_mut
            .append_child_element_mut(
                root_id,
                if enable_format_id {
                    "cellXfs"
                } else {
                    "cellStyleXfs"
                },
                None,
            )
            .context("Failed to create style element")?;
        let cell_style_xfs_element = xml_doc_mut
            .get_element_mut(cell_style_xfs_id)
            .context("Failed to get border element")?;
        cell_style_xfs_element
            .add_attribute_mut(XmlAttribute::new(
                "count".to_owned(),
                xfs_data.len().to_string(),
            ))
            .context("Updating Fills Element Attributes Failed")?;
        for (_, xfs) in xfs_data {
            let mut attributes = vec![
                XmlAttribute::new("numFmtId".to_string(), xfs.number_format_id.to_string()),
                XmlAttribute::new("fontId".to_string(), xfs.font_id.to_string()),
                XmlAttribute::new("fillId".to_string(), xfs.fill_id.to_string()),
                XmlAttribute::new("borderId".to_string(), xfs.border_id.to_string()),
            ];
            if enable_format_id {
                attributes.push(XmlAttribute::new(
                    "xfId".to_string(),
                    xfs.format_id.to_string(),
                ));
            }
            if xfs.apply_font > 0 {
                attributes.push(XmlAttribute::new(
                    "applyFont".to_string(),
                    xfs.apply_font.to_string(),
                ));
            }
            if xfs.apply_alignment > 0 {
                attributes.push(XmlAttribute::new(
                    "applyAlignment".to_string(),
                    xfs.apply_alignment.to_string(),
                ));
            }
            if xfs.apply_fill > 0 {
                attributes.push(XmlAttribute::new(
                    "applyFill".to_string(),
                    xfs.apply_fill.to_string(),
                ));
            }
            if xfs.apply_border > 0 {
                attributes.push(XmlAttribute::new(
                    "applyBorder".to_string(),
                    xfs.apply_border.to_string(),
                ));
            }
            if xfs.apply_number_format > 0 {
                attributes.push(XmlAttribute::new(
                    "applyNumberFormat".to_string(),
                    xfs.apply_number_format.to_string(),
                ));
            }
            if xfs.apply_protection > 0 {
                attributes.push(XmlAttribute::new(
                    "applyProtection".to_string(),
                    xfs.apply_protection.to_string(),
                ));
            }
            let xf_id = xml_doc_mut
                .append_child_element_mut(cell_style_xfs_id, "xf", Some(attributes))
                .context("Create Cell Style Config Failed")?;
            if xfs.is_wrap_text > 0
                || xfs.vertical_alignment != VerticalAlignmentValues::None
                || xfs.horizontal_alignment != HorizontalAlignmentValues::None
            {
                let mut alignment_attributes = Vec::new();
                if xfs.is_wrap_text > 0 {
                    alignment_attributes.push(XmlAttribute::new(
                        "wrapText".to_string(),
                        xfs.is_wrap_text.to_string(),
                    ));
                }
                if xfs.vertical_alignment != VerticalAlignmentValues::None {
                    alignment_attributes.push(XmlAttribute::new(
                        "vertical".to_string(),
                        VerticalAlignmentValues::get_string(xfs.vertical_alignment.clone()),
                    ));
                }
                if xfs.horizontal_alignment != HorizontalAlignmentValues::None {
                    alignment_attributes.push(XmlAttribute::new(
                        "horizontal".to_string(),
                        HorizontalAlignmentValues::get_string(xfs.horizontal_alignment.clone()),
                    ));
                }
                xml_doc_mut
                    .append_child_element_mut(xf_id, "alignment", Some(alignment_attributes))
                    .context("Create Cell Alignment Style Config Failed")?;
            }
        }
        Ok(())
    }
}

// ################################## mut feature ########################
impl StylePart {
    fn generate_setting_hash<T: Hash>(&mut self, item: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

    pub(crate) fn get_style_id_mut(
        &mut self,
        style_setting: StyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        let style_hash = self.generate_setting_hash(&style_setting);
        if let Some((_, id)) = self.cache_id.get_key_value(&style_hash) {
            Ok(StyleId::new(*id))
        } else {
            let mut cell_style = CellXfs::default();
            if style_setting.number_format == NumberFormatValues::Custom {
                cell_style.apply_number_format = 1;
                // Get Number Format ID
                if let Some(custom_format) = style_setting.custom_number_format {
                    let mut hasher = DefaultHasher::new();
                    let number_format = NumberFormat {
                        format_code: custom_format,
                        format_type: NumberFormatValues::Custom,
                        ..NumberFormat::default()
                    };
                    number_format.hash(&mut hasher);
                    let current_hash = hasher.finish();
                    if let Some(position) = self
                        .number_format_collection
                        .iter()
                        .position(|(hash, _)| *hash == current_hash)
                    {
                        cell_style.number_format_id = position as u16 + 163;
                    } else {
                        self.number_format_collection
                            .push((current_hash, number_format));
                        cell_style.number_format_id =
                            (self.number_format_collection.len() - 1) as u16;
                    }
                } else {
                    return Err(anyhow!(
                        "Custom Format Type is used without providing custom number format."
                    ));
                }
            }
            // Get Font Style ID
            {
                let mut hasher = DefaultHasher::new();
                let font_style = FontStyle {
                    name: style_setting.font_family,
                    is_bold: style_setting.is_bold,
                    is_italic: style_setting.is_italic,
                    is_underline: style_setting.is_underline,
                    is_double_underline: style_setting.is_double_underline,
                    color: style_setting.text_color,
                    size: style_setting.font_size,
                    ..FontStyle::default()
                };
                font_style.hash(&mut hasher);
                let current_hash = hasher.finish();
                if let Some(position) = self
                    .font_collection
                    .iter()
                    .position(|(hash, _)| *hash == current_hash)
                {
                    cell_style.font_id = position as u16;
                    if position > 0 {
                        cell_style.apply_font = 1;
                    }
                } else {
                    self.font_collection.push((current_hash, font_style));
                    cell_style.font_id = (self.font_collection.len() - 1) as u16;
                    cell_style.apply_font = 1;
                }
            }
            // Get Fill Style ID
            {
                let mut hasher = DefaultHasher::new();
                let fill_style = FillStyle {
                    background_color: if let Some(background_color) = style_setting.background_color
                    {
                        Some(ColorSetting {
                            color_setting_type: ColorSettingTypeValues::Rgb,
                            value: background_color,
                        })
                    } else {
                        None
                    },
                    foreground_color: if let Some(foreground_color) = style_setting.foreground_color
                    {
                        Some(ColorSetting {
                            color_setting_type: ColorSettingTypeValues::Rgb,
                            value: foreground_color,
                        })
                    } else {
                        None
                    },
                    pattern_type: style_setting.pattern_type,
                    ..FillStyle::default()
                };
                fill_style.hash(&mut hasher);
                let current_hash = hasher.finish();
                if let Some(position) = self
                    .fill_collection
                    .iter()
                    .position(|(hash, _)| *hash == current_hash)
                {
                    cell_style.fill_id = position as u16;
                    if position > 0 {
                        cell_style.apply_fill = 1;
                    }
                } else {
                    self.fill_collection.push((current_hash, fill_style));
                    cell_style.fill_id = (self.fill_collection.len() - 1) as u16;
                    cell_style.apply_fill = 1;
                }
            }
            // Get Border Style ID
            {
                let mut hasher = DefaultHasher::new();
                let border_style = BorderStyle {
                    left: style_setting.border_left,
                    top: style_setting.border_top,
                    right: style_setting.border_right,
                    bottom: style_setting.border_bottom,
                    diagonal: style_setting.border_diagonal,
                    ..BorderStyle::default()
                };
                border_style.hash(&mut hasher);
                let current_hash = hasher.finish();
                if let Some(position) = self
                    .border_collection
                    .iter()
                    .position(|(hash, _)| *hash == current_hash)
                {
                    cell_style.border_id = position as u16;
                    if position > 0 {
                        cell_style.apply_border = 1;
                    }
                } else {
                    self.border_collection.push((current_hash, border_style));
                    cell_style.border_id = (self.border_collection.len() - 1) as u16;
                    cell_style.apply_border = 1;
                }
            }
            // Get Cell Style xfs to find xfId
            {
                let mut hasher = DefaultHasher::new();
                if style_setting.vertical_alignment != VerticalAlignmentValues::None
                    || style_setting.horizontal_alignment != HorizontalAlignmentValues::None
                    || style_setting.is_wrap_text
                {
                    cell_style.apply_alignment = 1;
                }
                cell_style.vertical_alignment = style_setting.vertical_alignment;
                cell_style.horizontal_alignment = style_setting.horizontal_alignment;
                if style_setting.is_wrap_text {
                    cell_style.is_wrap_text = 1;
                }
                let cell_style_xfs = cell_style.clone();
                cell_style_xfs.hash(&mut hasher);
                let current_hash = hasher.finish();
                if let Some(position) = self
                    .cell_style_xfs_collection
                    .iter()
                    .position(|(hash, _)| *hash == current_hash)
                {
                    cell_style.format_id = position as u16;
                } else {
                    self.cell_style_xfs_collection
                        .push((current_hash, cell_style_xfs));
                    cell_style.format_id = (self.cell_style_xfs_collection.len() - 1) as u16;
                }
            }
            // Get Cell xfs
            {
                let mut hasher = DefaultHasher::new();
                let font_style = cell_style;
                font_style.hash(&mut hasher);
                let current_hash = hasher.finish();
                if let Some(position) = self
                    .cell_xfs_collection
                    .iter()
                    .position(|(hash, _)| *hash == current_hash)
                {
                    Ok(StyleId::new(position as u32))
                } else {
                    self.cell_xfs_collection.push((current_hash, font_style));
                    Ok(StyleId::new((self.cell_xfs_collection.len() - 1) as u32))
                }
            }
        }
    }
}
