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
        NumberFormat, NumberFormatValues, PatternTypeValues, StyleId, CellStyleSetting,
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
pub(crate) struct StylePart {
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
            XmlDeserializer::vec_to_xml_doc_tree(
                include_str!("style.xml").as_bytes().to_vec(),
            )
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
            // Load Number Format Region
            if let Some(num_fmts_id) = xml_doc_mut
                .find_first_child(root_id, "numFmts")
                .context("Failed to find numFmts element")?
            {
                for element_id in StylePart::collect_child_ids(&xml_doc_mut, num_fmts_id)? {
                    let num_fmt = xml_doc_mut
                        .get_element(element_id)
                        .context("Element not Found Error")?;
                    let mut number_format = NumberFormat::default();
                    number_format.format_id = num_fmt
                        .get_attribute("numFmtId")
                        .map(|attribute| attribute.get_value())
                        .context("numFmtId Attribute Not Found!")?
                        .parse()
                        .context("Number format ID parsing Failed")?;
                    number_format.format_code = num_fmt
                        .get_attribute("formatCode")
                        .map(|attribute| attribute.get_value())
                        .context("formatCode Attribute Not Found!")?
                        .to_string();
                    let mut hasher = DefaultHasher::new();
                    number_format.hash(&mut hasher);
                    num_format_records.push((hasher.finish(), number_format));
                }
                xml_doc_mut
                    .remove_element_mut(num_fmts_id)
                    .context("Failed to remove numFmts element")?;
            }
            if let Some(fonts_id) = xml_doc_mut
                .find_first_child(root_id, "fonts")
                .context("Failed to find fonts element")?
            {
                for font_id in StylePart::collect_child_ids(&xml_doc_mut, fonts_id)? {
                    let mut font_style = FontStyle::default();
                    for item_id in StylePart::collect_child_ids(&xml_doc_mut, font_id)? {
                        let current_element = xml_doc_mut
                            .get_element(item_id)
                            .context("Failed to pull child element")?;
                        let value = current_element
                            .get_attribute("val")
                            .map(|attribute| attribute.get_value());
                        match current_element.get_tag().as_str() {
                            "b" => font_style.is_bold = true,
                            "u" => {
                                if value == Some("double") {
                                    font_style.is_double_underline = true;
                                }
                                font_style.is_underline = false;
                            }
                            "i" => font_style.is_italic = false,
                            "sz" => {
                                if let Some(value) = value {
                                    font_style.size =
                                        value.parse().context("Font Size Parse Failed")?;
                                }
                            }
                            "color" => {
                                if let Some(color) =
                                    StylePart::deserialize_color_setting(current_element)
                                {
                                    font_style.color = color;
                                }
                            }
                            "name" => {
                                if let Some(value) = value {
                                    font_style.name = value.to_string();
                                }
                            }
                            "family" => {
                                if let Some(value) = value {
                                    font_style.family =
                                        value.parse().context("Font Family Parse Failed")?;
                                }
                            }
                            "scheme" => {
                                if let Some(value) = value {
                                    font_style.font_scheme = FontSchemeValues::get_enum(value);
                                }
                            }
                            _ => return Err(anyhow!("Unknown Font Style Found!")),
                        }
                    }
                    let mut hasher = DefaultHasher::new();
                    font_style.hash(&mut hasher);
                    font_records.push((hasher.finish(), font_style));
                }
                xml_doc_mut
                    .remove_element_mut(fonts_id)
                    .context("Failed to remove fonts element")?;
            }
            if let Some(fills_id) = xml_doc_mut
                .find_first_child(root_id, "fills")
                .context("Failed to find fills element")?
            {
                for fill_id in StylePart::collect_child_ids(&xml_doc_mut, fills_id)? {
                    let mut fill_style = FillStyle::default();
                    for pattern_fill_id in StylePart::collect_child_ids(&xml_doc_mut, fill_id)? {
                        let pattern_type = xml_doc_mut
                            .get_element(pattern_fill_id)
                            .context("Failed to pull pattern fill element")?
                            .get_attribute("patternType")
                            .map(|attribute| attribute.get_value().to_string());
                        if let Some(pattern_type) = pattern_type {
                            fill_style.pattern_type = PatternTypeValues::get_enum(&pattern_type);
                            for child_id in
                                StylePart::collect_child_ids(&xml_doc_mut, pattern_fill_id)?
                            {
                                let color_element = xml_doc_mut
                                    .get_element(child_id)
                                    .context("Failed to pull color child element")?;
                                match color_element.get_tag().as_str() {
                                    "fgColor" => {
                                        fill_style.foreground_color =
                                            StylePart::deserialize_color_setting(color_element);
                                    }
                                    "bgColor" => {
                                        fill_style.background_color =
                                            StylePart::deserialize_color_setting(color_element);
                                    }
                                    _ => return Err(anyhow!("Unknown Color patter found")),
                                }
                            }
                        }
                    }
                    let mut hasher = DefaultHasher::new();
                    fill_style.hash(&mut hasher);
                    fill_records.push((hasher.finish(), fill_style));
                }
                xml_doc_mut
                    .remove_element_mut(fills_id)
                    .context("Failed to remove fills element")?;
            }
            if let Some(borders_id) = xml_doc_mut
                .find_first_child(root_id, "borders")
                .context("Failed to find borders element")?
            {
                for border_id in StylePart::collect_child_ids(&xml_doc_mut, borders_id)? {
                    let mut border_style = BorderStyle::default();
                    for border_child_id in StylePart::collect_child_ids(&xml_doc_mut, border_id)? {
                        let tag = xml_doc_mut
                            .get_element(border_child_id)
                            .context("Failed to pull border child element")?
                            .get_tag();
                        match tag.as_str() {
                            "left" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.left,
                                    &mut xml_doc_mut,
                                )
                                .context("Left Border Decode Failed")?;
                            }
                            "right" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.right,
                                    &mut xml_doc_mut,
                                )
                                .context("Left Border Decode Failed")?;
                            }
                            "top" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.top,
                                    &mut xml_doc_mut,
                                )
                                .context("Left Border Decode Failed")?;
                            }
                            "bottom" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.bottom,
                                    &mut xml_doc_mut,
                                )
                                .context("Left Border Decode Failed")?;
                            }
                            "diagonal" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.diagonal,
                                    &mut xml_doc_mut,
                                )
                                .context("Left Border Decode Failed")?;
                            }
                            _ => {
                                return Err(anyhow!("Unknown border style found"));
                            }
                        }
                    }
                    let mut hasher = DefaultHasher::new();
                    border_style.hash(&mut hasher);
                    border_records.push((hasher.finish(), border_style));
                }
                xml_doc_mut
                    .remove_element_mut(borders_id)
                    .context("Failed to remove borders element")?;
            }
            if let Some(cell_style_xfs_id) = xml_doc_mut
                .find_first_child(root_id, "cellStyleXfs")
                .context("Failed to find cellStyleXfs element")?
            {
                style_collection =
                    StylePart::deserialize_cell_style(cell_style_xfs_id, &mut xml_doc_mut)
                        .context("Deserializing Cell Style Xfs Failed")?;
                xml_doc_mut
                    .remove_element_mut(cell_style_xfs_id)
                    .context("Failed to remove cellStyleXfs element")?;
            }
            if let Some(cell_xfs_id) = xml_doc_mut
                .find_first_child(root_id, "cellXfs")
                .context("Failed to find cellXfs element")?
            {
                xfs_collection = StylePart::deserialize_cell_style(cell_xfs_id, &mut xml_doc_mut)
                    .context("Deserializing Cell Xfs Failed")?;
                xml_doc_mut
                    .remove_element_mut(cell_xfs_id)
                    .context("Failed to remove cellXfs element")?;
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
            // Number Formats
            {
                let num_formats_id = xml_doc_mut
                    .inser_child_element_before_first_tag_mut(
                        root_id,
                        "numFmts",
                        "cellStyles",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.number_format_collection.len().to_string(),
                        )]),
                    )
                    .context("Create Number Formats Parent Failed.")?;
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
            // Fonts
            {
                let fonts_id = xml_doc_mut
                    .inser_child_element_after_last_tag_mut(
                        root_id,
                        "fonts",
                        "numFmts",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.font_collection.len().to_string(),
                        )]),
                    )
                    .context("Create Fonts Parent Failed.")?;
                for (_, font_style) in self.font_collection.as_slice() {
                    let font_id = xml_doc_mut
                        .append_child_element_mut(fonts_id, "font", None)
                        .context("Adding Font to Fonts Failed")?;
                    if font_style.is_bold {
                        xml_doc_mut
                            .append_child_element_mut(font_id, "b", None)
                            .context("Create Bold Element Failed")?;
                    }
                    if font_style.is_italic {
                        xml_doc_mut
                            .append_child_element_mut(font_id, "i", None)
                            .context("Create Italic Element Failed")?;
                    }
                    if font_style.is_underline {
                        xml_doc_mut
                            .append_child_element_mut(font_id, "u", None)
                            .context("Create Underline Element Failed")?;
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
                            .context("Create Double Underline Element Failed")?;
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
                        .context("Create Font Size Element Failed")?;
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
                        .context("Create Font Name Element Failed")?;
                    xml_doc_mut
                        .append_child_element_mut(
                            font_id,
                            "family",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.family.to_string(),
                            )]),
                        )
                        .context("Create Font Family Element Failed")?;
                    xml_doc_mut
                        .append_child_element_mut(
                            font_id,
                            "scheme",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                FontSchemeValues::get_string(font_style.font_scheme.clone()),
                            )]),
                        )
                        .context("Create Font Scheme Element Failed")?;
                }
            }
            // Fills
            {
                let fills_id = xml_doc_mut
                    .inser_child_element_after_last_tag_mut(
                        root_id,
                        "fills",
                        "fonts",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.fill_collection.len().to_string(),
                        )]),
                    )
                    .context("Create Fills parents Failed.")?;
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
            // Borders
            {
                let borders_id = xml_doc_mut
                    .inser_child_element_after_last_tag_mut(
                        root_id,
                        "borders",
                        "fills",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.border_collection.len().to_string(),
                        )]),
                    )
                    .context("Create borders parents Failed.")?;
                for (_, border_data) in self.border_collection.as_slice() {
                    let border_id = xml_doc_mut
                        .append_child_element_mut(borders_id, "border", None)
                        .context("Create Border Failed")?;
                    for (border_side, border_setting) in [
                        ("left", &border_data.left),
                        ("right", &border_data.right),
                        ("top", &border_data.top),
                        ("bottom", &border_data.bottom),
                        ("diagonal", &border_data.diagonal),
                    ] {
                        StylePart::add_border_element(
                            border_side,
                            &mut xml_doc_mut,
                            border_id,
                            border_setting.clone(),
                        )?;
                    }
                }
            }
            // Cell Style Xfs
            StylePart::add_cell_style(&mut xml_doc_mut, &mut self.cell_xfs_collection, true)?;
            // Cell Styles Xfs
            StylePart::add_cell_style(&mut xml_doc_mut, &mut self.cell_style_xfs_collection, false)?;
        }
        Ok(())
    }

    pub(crate) fn deserialize_cell_style(
        style_xfs_id: NodeId,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
    ) -> AnyResult<Vec<(u64, CellXfs)>, AnyError> {
        let mut style_records = Vec::new();
        for xf_id in StylePart::collect_child_ids(xml_doc_mut, style_xfs_id)? {
            let mut cell_xf = CellXfs::default();
            {
                let current_element = xml_doc_mut
                    .get_element(xf_id)
                    .context("Failed to pull xf element")?;
                let attribute_value = |attribute_name: &str| {
                    current_element
                        .get_attribute(attribute_name)
                        .map(|attribute| attribute.get_value())
                };
                if let Some(value) = attribute_value("numFmtId") {
                    cell_xf.number_format_id =
                        value.parse().context("Cell Number Format Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("fontId") {
                    cell_xf.font_id = value.parse().context("Cell Font Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("fillId") {
                    cell_xf.fill_id = value.parse().context("Cell Fill Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("borderId") {
                    cell_xf.border_id = value.parse().context("Cell Border Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("xfId") {
                    cell_xf.format_id = value.parse().context("Cell Format Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("applyProtection") {
                    cell_xf.apply_protection = ConverterUtil::normalize_bool_property_u8(value);
                }
                if let Some(value) = attribute_value("applyAlignment") {
                    cell_xf.apply_alignment = ConverterUtil::normalize_bool_property_u8(value);
                }
                if let Some(value) = attribute_value("applyBorder") {
                    cell_xf.apply_border = ConverterUtil::normalize_bool_property_u8(value);
                }
                if let Some(value) = attribute_value("applyFill") {
                    cell_xf.apply_fill = ConverterUtil::normalize_bool_property_u8(value);
                }
                if let Some(value) = attribute_value("applyFont") {
                    cell_xf.apply_font = ConverterUtil::normalize_bool_property_u8(value);
                }
                if let Some(value) = attribute_value("applyNumberFormat") {
                    cell_xf.apply_number_format = ConverterUtil::normalize_bool_property_u8(value);
                }
            }
            // Load Alignment Values if exist
            for alignment_id in StylePart::collect_child_ids(xml_doc_mut, xf_id)? {
                let alignment_element = xml_doc_mut
                    .get_element(alignment_id)
                    .context("Failed to pull alignment element")?;
                let attribute_value = |attribute_name: &str| {
                    alignment_element
                        .get_attribute(attribute_name)
                        .map(|attribute| attribute.get_value())
                };
                if let Some(value) = attribute_value("wrapText") {
                    cell_xf.is_wrap_text = ConverterUtil::normalize_bool_property_u8(value);
                }
                if let Some(value) = attribute_value("vertical") {
                    cell_xf.vertical_alignment = VerticalAlignmentValues::get_enum(value);
                }
                if let Some(value) = attribute_value("horizontal") {
                    cell_xf.horizontal_alignment = HorizontalAlignmentValues::get_enum(value);
                }
            }
            let mut hasher = DefaultHasher::new();
            cell_xf.hash(&mut hasher);
            style_records.push((hasher.finish(), cell_xf));
        }
        Ok(style_records)
    }

    pub(crate) fn deserialize_border_setting(
        current_element_id: NodeId,
        border: &mut BorderSetting,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
    ) -> Result<(), AnyError> {
        let style = xml_doc_mut
            .get_element(current_element_id)
            .context("Failed to pull border side element")?
            .get_attribute("style")
            .map(|attribute| attribute.get_value().to_string());
        if let Some(style) = style {
            border.style = BorderStyleValues::get_enum(&style);
            if border.style != BorderStyleValues::None {
                for color_id in StylePart::collect_child_ids(xml_doc_mut, current_element_id)? {
                    let color_element = xml_doc_mut
                        .get_element(color_id)
                        .context("Failed to pull border color element")?;
                    if let Some(color) = StylePart::deserialize_color_setting(color_element) {
                        border.border_color = Some(color);
                    }
                }
            }
        }
        Ok(())
    }

    /// Build a `ColorSetting` from the theme, rgb or indexed attribute of a color element
    fn deserialize_color_setting(color_element: &XmlElement) -> Option<ColorSetting> {
        for (attribute_name, color_setting_type) in [
            ("theme", ColorSettingTypeValues::Theme),
            ("rgb", ColorSettingTypeValues::Rgb),
            ("indexed", ColorSettingTypeValues::Indexed),
        ] {
            if let Some(attribute) = color_element.get_attribute(attribute_name) {
                return Some(ColorSetting {
                    color_setting_type,
                    value: attribute.get_value().to_string(),
                });
            }
        }
        None
    }

    /// Collect the NodeId of every child element of the given parent
    fn collect_child_ids(
        xml_doc: &XmlDocument,
        parent_id: NodeId,
    ) -> AnyResult<Vec<NodeId>, AnyError> {
        Ok(xml_doc
            .get_element(parent_id)
            .context("Failed to pull parent element")?
            .get_child_contents()
            .as_ref()
            .map(|contents| {
                contents
                    .iter()
                    .filter_map(|content| match content {
                        XmlElementContentType::Element((id, _, _)) => Some(*id),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Add Color Element Node To XML
    fn add_color_element(
        color_setting: Option<ColorSetting>,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        parent_id: NodeId,
    ) -> Result<(), AnyError> {
        if let Some(border_color_setting) = color_setting {
            xml_doc_mut
                .append_child_element_mut(
                    parent_id,
                    "color",
                    Some(vec![XmlAttribute::new(
                        ColorSettingTypeValues::get_string(
                            border_color_setting.color_setting_type,
                        ),
                        border_color_setting.value,
                    )]),
                )
                .context("Create Color Element Failed")?;
        }
        Ok(())
    }

    /// Add Border Element Node to XML
    fn add_border_element(
        border_side: &str,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        border_id: NodeId,
        border_data: BorderSetting,
    ) -> Result<(), AnyError> {
        let has_style = border_data.style != BorderStyleValues::None;
        let attributes = if has_style {
            Some(vec![XmlAttribute::new(
                "style".to_string(),
                BorderStyleValues::get_string(border_data.style.clone()),
            )])
        } else {
            None
        };
        let id = xml_doc_mut
            .append_child_element_mut(border_id, border_side, attributes)
            .context(format!("{} Border Element Creation Failed", border_side))?;
        if has_style {
            StylePart::add_color_element(border_data.border_color, xml_doc_mut, id)?;
        }
        Ok(())
    }

    fn add_cell_style(
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        xfs_data: &mut Vec<(u64, CellXfs)>,
        enable_format_id: bool,
    ) -> Result<(), AnyError> {
        let root_id = xml_doc_mut.get_root_id();
        let cell_style_xfs_id = xml_doc_mut
            .inser_child_element_after_last_tag_mut(
                root_id,
                if enable_format_id {
                    "cellXfs"
                } else {
                    "cellStyleXfs"
                },
                "borders",
                Some(vec![XmlAttribute::new(
                    "count".to_string(),
                    xfs_data.len().to_string(),
                )]),
            )
            .context("Create Cell Style parents Failed.")?;
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
        style_setting: CellStyleSetting,
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
