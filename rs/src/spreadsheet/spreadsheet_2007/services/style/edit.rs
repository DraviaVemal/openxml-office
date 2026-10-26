use std::hash::{DefaultHasher, Hash, Hasher};

use anyhow::{Error as AnyError, Result as AnyResult};

use crate::spreadsheet_2007::{
    
        BorderStyle, CellStyleSetting, CellXfs, ColorSetting, ColorSettingTypeValues, FillStyle,
        FontStyle, HorizontalAlignmentValues, NumberFormat, NumberFormatValues, StyleId,
        VerticalAlignmentValues,
    services::StylePart,
};

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
                    return Err(AnyError::msg(
                        "draviavemal-openxml_office::Custom Format Type is used without providing custom number format."
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
