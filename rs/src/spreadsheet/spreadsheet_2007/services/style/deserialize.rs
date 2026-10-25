use std::{
    cell::RefCell,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Weak,
};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlDocument, XmlElement};

use crate::{
    converters::ConverterUtil,
    global_2007::traits::Enum,
    spreadsheet_2007::{
        models::{
            BorderSetting, BorderStyle, BorderStyleValues, CellXfs, ColorSetting,
            ColorSettingTypeValues, FillStyle, FontSchemeValues, FontStyle,
            HorizontalAlignmentValues, NumberFormat, PatternTypeValues, VerticalAlignmentValues,
        },
        services::StylePart,
    },
};

impl StylePart {
    pub(crate) fn deserialize_content(
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
                .context("draviavemal-openxml_office::xml doc borrow failed")?;
            let root_id = xml_doc_mut.get_root_id();
            // Load Number Format Region
            if let Some(num_fmts_id) = xml_doc_mut
                .find_first_child(root_id, "numFmts")
                .context("draviavemal-openxml_office::Failed to find numFmts element")?
            {
                for element_id in StylePart::collect_child_ids(&xml_doc_mut, num_fmts_id)? {
                    let num_fmt = xml_doc_mut
                        .get_element(element_id)
                        .context("draviavemal-openxml_office::Element not Found Error")?;
                    let mut number_format = NumberFormat::default();
                    number_format.format_id = num_fmt
                        .get_attribute("numFmtId")
                        .map(|attribute| attribute.get_value())
                        .context("draviavemal-openxml_office::numFmtId Attribute Not Found!")?
                        .parse()
                        .context("draviavemal-openxml_office::Number format ID parsing Failed")?;
                    number_format.format_code = num_fmt
                        .get_attribute("formatCode")
                        .map(|attribute| attribute.get_value())
                        .context("draviavemal-openxml_office::formatCode Attribute Not Found!")?
                        .to_string();
                    let mut hasher = DefaultHasher::new();
                    number_format.hash(&mut hasher);
                    num_format_records.push((hasher.finish(), number_format));
                }
                xml_doc_mut
                    .remove_element_mut(num_fmts_id)
                    .context("draviavemal-openxml_office::Failed to remove numFmts element")?;
            }
            if let Some(fonts_id) = xml_doc_mut
                .find_first_child(root_id, "fonts")
                .context("draviavemal-openxml_office::Failed to find fonts element")?
            {
                for font_id in StylePart::collect_child_ids(&xml_doc_mut, fonts_id)? {
                    let mut font_style = FontStyle::default();
                    for item_id in StylePart::collect_child_ids(&xml_doc_mut, font_id)? {
                        let current_element = xml_doc_mut
                            .get_element(item_id)
                            .context("draviavemal-openxml_office::Failed to pull child element")?;
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
                                    font_style.size = value.parse().context(
                                        "draviavemal-openxml_office::Font Size Parse Failed",
                                    )?;
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
                                    font_style.family = value.parse().context(
                                        "draviavemal-openxml_office::Font Family Parse Failed",
                                    )?;
                                }
                            }
                            "scheme" => {
                                if let Some(value) = value {
                                    font_style.font_scheme = FontSchemeValues::get_enum(value);
                                }
                            }
                            _ => {
                                return Err(AnyError::msg(
                                    "draviavemal-openxml_office::Unknown Font Style Found!",
                                ))
                            }
                        }
                    }
                    let mut hasher = DefaultHasher::new();
                    font_style.hash(&mut hasher);
                    font_records.push((hasher.finish(), font_style));
                }
                xml_doc_mut
                    .remove_element_mut(fonts_id)
                    .context("draviavemal-openxml_office::Failed to remove fonts element")?;
            }
            if let Some(fills_id) = xml_doc_mut
                .find_first_child(root_id, "fills")
                .context("draviavemal-openxml_office::Failed to find fills element")?
            {
                for fill_id in StylePart::collect_child_ids(&xml_doc_mut, fills_id)? {
                    let mut fill_style = FillStyle::default();
                    for pattern_fill_id in StylePart::collect_child_ids(&xml_doc_mut, fill_id)? {
                        let pattern_type = xml_doc_mut
                            .get_element(pattern_fill_id)
                            .context(
                                "draviavemal-openxml_office::Failed to pull pattern fill element",
                            )?
                            .get_attribute("patternType")
                            .map(|attribute| attribute.get_value().to_string());
                        if let Some(pattern_type) = pattern_type {
                            fill_style.pattern_type = PatternTypeValues::get_enum(&pattern_type);
                            for child_id in
                                StylePart::collect_child_ids(&xml_doc_mut, pattern_fill_id)?
                            {
                                let color_element = xml_doc_mut
                                    .get_element(child_id)
                                    .context("draviavemal-openxml_office::Failed to pull color child element")?;
                                match color_element.get_tag().as_str() {
                                    "fgColor" => {
                                        fill_style.foreground_color =
                                            StylePart::deserialize_color_setting(color_element);
                                    }
                                    "bgColor" => {
                                        fill_style.background_color =
                                            StylePart::deserialize_color_setting(color_element);
                                    }
                                    _ => return Err(AnyError::msg(
                                        "draviavemal-openxml_office::Unknown Color patter found",
                                    )),
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
                    .context("draviavemal-openxml_office::Failed to remove fills element")?;
            }
            if let Some(borders_id) = xml_doc_mut
                .find_first_child(root_id, "borders")
                .context("draviavemal-openxml_office::Failed to find borders element")?
            {
                for border_id in StylePart::collect_child_ids(&xml_doc_mut, borders_id)? {
                    let mut border_style = BorderStyle::default();
                    for border_child_id in StylePart::collect_child_ids(&xml_doc_mut, border_id)? {
                        let tag = xml_doc_mut
                            .get_element(border_child_id)
                            .context(
                                "draviavemal-openxml_office::Failed to pull border child element",
                            )?
                            .get_tag();
                        match tag.as_str() {
                            "left" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.left,
                                    &mut xml_doc_mut,
                                )
                                .context("draviavemal-openxml_office::Left Border Decode Failed")?;
                            }
                            "right" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.right,
                                    &mut xml_doc_mut,
                                )
                                .context("draviavemal-openxml_office::Left Border Decode Failed")?;
                            }
                            "top" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.top,
                                    &mut xml_doc_mut,
                                )
                                .context("draviavemal-openxml_office::Left Border Decode Failed")?;
                            }
                            "bottom" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.bottom,
                                    &mut xml_doc_mut,
                                )
                                .context("draviavemal-openxml_office::Left Border Decode Failed")?;
                            }
                            "diagonal" => {
                                StylePart::deserialize_border_setting(
                                    border_child_id,
                                    &mut border_style.diagonal,
                                    &mut xml_doc_mut,
                                )
                                .context("draviavemal-openxml_office::Left Border Decode Failed")?;
                            }
                            _ => {
                                return Err(AnyError::msg(
                                    "draviavemal-openxml_office::Unknown border style found",
                                ));
                            }
                        }
                    }
                    let mut hasher = DefaultHasher::new();
                    border_style.hash(&mut hasher);
                    border_records.push((hasher.finish(), border_style));
                }
                xml_doc_mut
                    .remove_element_mut(borders_id)
                    .context("draviavemal-openxml_office::Failed to remove borders element")?;
            }
            if let Some(cell_style_xfs_id) =
                xml_doc_mut
                    .find_first_child(root_id, "cellStyleXfs")
                    .context("draviavemal-openxml_office::Failed to find cellStyleXfs element")?
            {
                style_collection =
                    StylePart::deserialize_cell_style(cell_style_xfs_id, &mut xml_doc_mut)
                        .context(
                            "draviavemal-openxml_office::Deserializing Cell Style Xfs Failed",
                        )?;
                xml_doc_mut
                    .remove_element_mut(cell_style_xfs_id)
                    .context("draviavemal-openxml_office::Failed to remove cellStyleXfs element")?;
            }
            if let Some(cell_xfs_id) = xml_doc_mut
                .find_first_child(root_id, "cellXfs")
                .context("draviavemal-openxml_office::Failed to find cellXfs element")?
            {
                xfs_collection =
                    StylePart::deserialize_cell_style(cell_xfs_id, &mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Deserializing Cell Xfs Failed")?;
                xml_doc_mut
                    .remove_element_mut(cell_xfs_id)
                    .context("draviavemal-openxml_office::Failed to remove cellXfs element")?;
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
    fn deserialize_cell_style(
        style_xfs_id: NodeId,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
    ) -> AnyResult<Vec<(u64, CellXfs)>, AnyError> {
        let mut style_records = Vec::new();
        for xf_id in StylePart::collect_child_ids(xml_doc_mut, style_xfs_id)? {
            let mut cell_xf = CellXfs::default();
            {
                let current_element = xml_doc_mut
                    .get_element(xf_id)
                    .context("draviavemal-openxml_office::Failed to pull xf element")?;
                let attribute_value = |attribute_name: &str| {
                    current_element
                        .get_attribute(attribute_name)
                        .map(|attribute| attribute.get_value())
                };
                if let Some(value) = attribute_value("numFmtId") {
                    cell_xf.number_format_id = value.parse().context(
                        "draviavemal-openxml_office::Cell Number Format Id Parse Failed",
                    )?;
                }
                if let Some(value) = attribute_value("fontId") {
                    cell_xf.font_id = value
                        .parse()
                        .context("draviavemal-openxml_office::Cell Font Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("fillId") {
                    cell_xf.fill_id = value
                        .parse()
                        .context("draviavemal-openxml_office::Cell Fill Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("borderId") {
                    cell_xf.border_id = value
                        .parse()
                        .context("draviavemal-openxml_office::Cell Border Id Parse Failed")?;
                }
                if let Some(value) = attribute_value("xfId") {
                    cell_xf.format_id = value
                        .parse()
                        .context("draviavemal-openxml_office::Cell Format Id Parse Failed")?;
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
                    .context("draviavemal-openxml_office::Failed to pull alignment element")?;
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

    fn deserialize_border_setting(
        current_element_id: NodeId,
        border: &mut BorderSetting,
        xml_doc_mut: &mut std::cell::RefMut<'_, XmlDocument>,
    ) -> Result<(), AnyError> {
        let style = xml_doc_mut
            .get_element(current_element_id)
            .context("draviavemal-openxml_office::Failed to pull border side element")?
            .get_attribute("style")
            .map(|attribute| attribute.get_value().to_string());
        if let Some(style) = style {
            border.style = BorderStyleValues::get_enum(&style);
            if border.style != BorderStyleValues::None {
                for color_id in StylePart::collect_child_ids(xml_doc_mut, current_element_id)? {
                    let color_element = xml_doc_mut.get_element(color_id).context(
                        "draviavemal-openxml_office::Failed to pull border color element",
                    )?;
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
}
