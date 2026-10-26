use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlAttribute, XmlDocument};

use crate::{
    global::traits::Enum,
    spreadsheet_2007::{
        
            BorderSetting, BorderStyleValues, CellXfs, ColorSetting, ColorSettingTypeValues,
            FontSchemeValues, HorizontalAlignmentValues, PatternTypeValues,
            VerticalAlignmentValues,
        services::StylePart,
    },
};

impl StylePart {
    pub(crate) fn save_content_to_tree_mut(&mut self) -> AnyResult<(), AnyError> {
        if let Some(xml_document) = self.xml_document.upgrade() {
            let mut xml_document_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::xml doc borrow failed")?;
            let root_id = xml_document_mut.get_root_id();
            // Number Formats
            {
                let num_formats_id = xml_document_mut
                    .insert_child_element_before_first_tag_mut(
                        root_id,
                        "numFmts",
                        "cellStyles",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.number_format_collection.len().to_string(),
                        )]),
                    )
                    .context("draviavemal-openxml_office::Create Number Formats Parent Failed.")?;
                for (_, num_format) in self.number_format_collection.as_slice() {
                    xml_document_mut
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
                        .context(
                            "draviavemal-openxml_office::Create Number Format Element Failed",
                        )?;
                }
            }
            // Fonts
            {
                let fonts_id = xml_document_mut
                    .insert_child_element_after_last_tag_mut(
                        root_id,
                        "fonts",
                        "numFmts",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.font_collection.len().to_string(),
                        )]),
                    )
                    .context("draviavemal-openxml_office::Create Fonts Parent Failed.")?;
                for (_, font_style) in self.font_collection.as_slice() {
                    let font_id = xml_document_mut
                        .append_child_element_mut(fonts_id, "font", None)
                        .context("draviavemal-openxml_office::Adding Font to Fonts Failed")?;
                    if font_style.is_bold {
                        xml_document_mut
                            .append_child_element_mut(font_id, "b", None)
                            .context("draviavemal-openxml_office::Create Bold Element Failed")?;
                    }
                    if font_style.is_italic {
                        xml_document_mut
                            .append_child_element_mut(font_id, "i", None)
                            .context("draviavemal-openxml_office::Create Italic Element Failed")?;
                    }
                    if font_style.is_underline {
                        xml_document_mut
                            .append_child_element_mut(font_id, "u", None)
                            .context(
                                "draviavemal-openxml_office::Create Underline Element Failed",
                            )?;
                    }
                    if font_style.is_double_underline {
                        xml_document_mut
                            .append_child_element_mut(
                                font_id,
                                "u",
                                Some(vec![XmlAttribute::new(
                                    "val".to_string(),
                                    "double".to_string(),
                                )]),
                            )
                            .context("draviavemal-openxml_office::Create Double Underline Element Failed")?;
                    }
                    xml_document_mut
                        .append_child_element_mut(
                            font_id,
                            "sz",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.size.to_string(),
                            )]),
                        )
                        .context("draviavemal-openxml_office::Create Font Size Element Failed")?;
                    StylePart::add_color_element(
                        Some(font_style.color.clone()),
                        &mut xml_document_mut,
                        font_id,
                    )?;
                    xml_document_mut
                        .append_child_element_mut(
                            font_id,
                            "name",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.name.clone(),
                            )]),
                        )
                        .context("draviavemal-openxml_office::Create Font Name Element Failed")?;
                    xml_document_mut
                        .append_child_element_mut(
                            font_id,
                            "family",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                font_style.family.to_string(),
                            )]),
                        )
                        .context("draviavemal-openxml_office::Create Font Family Element Failed")?;
                    xml_document_mut
                        .append_child_element_mut(
                            font_id,
                            "scheme",
                            Some(vec![XmlAttribute::new(
                                "val".to_string(),
                                FontSchemeValues::get_string(font_style.font_scheme.clone()),
                            )]),
                        )
                        .context("draviavemal-openxml_office::Create Font Scheme Element Failed")?;
                }
            }
            // Fills
            {
                let fills_id = xml_document_mut
                    .insert_child_element_after_last_tag_mut(
                        root_id,
                        "fills",
                        "fonts",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.fill_collection.len().to_string(),
                        )]),
                    )
                    .context("draviavemal-openxml_office::Create Fills parents Failed.")?;
                for (_, fill_data) in self.fill_collection.as_slice() {
                    let fill_id = xml_document_mut
                        .append_child_element_mut(fills_id, "fill", None)
                        .context("draviavemal-openxml_office::Adding Fill Element Failed")?;
                    let pattern_fill_id = xml_document_mut
                        .append_child_element_mut(
                            fill_id,
                            "patternFill",
                            Some(vec![XmlAttribute::new(
                                "patternType".to_string(),
                                PatternTypeValues::get_string(fill_data.pattern_type.clone()),
                            )]),
                        )
                        .context("draviavemal-openxml_office::Pattern Fill Element Failed")?;
                    if let Some(fg_setting) = fill_data.foreground_color.clone() {
                        xml_document_mut
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
                            .context("draviavemal-openxml_office::Pattern Fill Foreground Element Failed")?;
                    }
                    if let Some(bg_setting) = fill_data.background_color.clone() {
                        xml_document_mut
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
                            .context("draviavemal-openxml_office::Pattern Fill Background Element Failed")?;
                    }
                }
            }
            // Borders
            {
                let borders_id = xml_document_mut
                    .insert_child_element_after_last_tag_mut(
                        root_id,
                        "borders",
                        "fills",
                        Some(vec![XmlAttribute::new(
                            "count".to_string(),
                            self.border_collection.len().to_string(),
                        )]),
                    )
                    .context("draviavemal-openxml_office::Create borders parents Failed.")?;
                for (_, border_data) in self.border_collection.as_slice() {
                    let border_id = xml_document_mut
                        .append_child_element_mut(borders_id, "border", None)
                        .context("draviavemal-openxml_office::Create Border Failed")?;
                    for (border_side, border_setting) in [
                        ("left", &border_data.left),
                        ("right", &border_data.right),
                        ("top", &border_data.top),
                        ("bottom", &border_data.bottom),
                        ("diagonal", &border_data.diagonal),
                    ] {
                        StylePart::add_border_element(
                            border_side,
                            &mut xml_document_mut,
                            border_id,
                            border_setting.clone(),
                        )?;
                    }
                }
            }
            // Cell Style Xfs
            StylePart::add_cell_style(&mut xml_document_mut, &mut self.cell_xfs_collection, true)?;
            // Cell Styles Xfs
            StylePart::add_cell_style(
                &mut xml_document_mut,
                &mut self.cell_style_xfs_collection,
                false,
            )?;
        }
        Ok(())
    }

    /// Add Color Element Node To XML
    fn add_color_element(
        color_setting: Option<ColorSetting>,
        xml_document_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        parent_id: NodeId,
    ) -> Result<(), AnyError> {
        if let Some(border_color_setting) = color_setting {
            xml_document_mut
                .append_child_element_mut(
                    parent_id,
                    "color",
                    Some(vec![XmlAttribute::new(
                        ColorSettingTypeValues::get_string(border_color_setting.color_setting_type),
                        border_color_setting.value,
                    )]),
                )
                .context("draviavemal-openxml_office::Create Color Element Failed")?;
        }
        Ok(())
    }

    /// Add Border Element Node to XML
    fn add_border_element(
        border_side: &str,
        xml_document_mut: &mut std::cell::RefMut<'_, XmlDocument>,
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
        let id = xml_document_mut
            .append_child_element_mut(border_id, border_side, attributes)
            .context(format!("{} Border Element Creation Failed", border_side))?;
        if has_style {
            StylePart::add_color_element(border_data.border_color, xml_document_mut, id)?;
        }
        Ok(())
    }

    fn add_cell_style(
        xml_document_mut: &mut std::cell::RefMut<'_, XmlDocument>,
        xfs_data: &mut Vec<(u64, CellXfs)>,
        enable_format_id: bool,
    ) -> Result<(), AnyError> {
        let root_id = xml_document_mut.get_root_id();
        let cell_style_xfs_id = xml_document_mut
            .insert_child_element_after_last_tag_mut(
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
            .context("draviavemal-openxml_office::Create Cell Style parents Failed.")?;
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
            let xf_id = xml_document_mut
                .append_child_element_mut(cell_style_xfs_id, "xf", Some(attributes))
                .context("draviavemal-openxml_office::Create Cell Style Config Failed")?;
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
                xml_document_mut
                    .append_child_element_mut(xf_id, "alignment", Some(alignment_attributes))
                    .context(
                        "draviavemal-openxml_office::Create Cell Alignment Style Config Failed",
                    )?;
            }
        }
        Ok(())
    }
}
