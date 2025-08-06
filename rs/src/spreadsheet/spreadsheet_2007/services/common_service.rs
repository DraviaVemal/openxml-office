use crate::global_2007::traits::XmlDocumentPartCommon;
use crate::spreadsheet_2007::models::{StyleId, StyleSetting};
use crate::spreadsheet_2007::services::{CalculationChainPart, ShareStringPart, StylePart};
use anyhow::{Context, Error as AnyError, Result as AnyResult};

#[derive(Debug)]
pub(crate) struct CommonServices {
    calculation_chain: CalculationChainPart,
    share_string: ShareStringPart,
    style: StylePart,
}

impl CommonServices {
    pub(crate) fn new(
        calculation_chain: CalculationChainPart,
        share_string: ShareStringPart,
        style: StylePart,
    ) -> Self {
        Self {
            calculation_chain,
            share_string,
            style,
        }
    }
    pub(crate) fn close_service(&mut self) -> AnyResult<(), AnyError> {
        self.calculation_chain
            .close_document()
            .context("Common Service Calculation Chain Close Failed")?;
        self.share_string
            .close_document()
            .context("Common Service Share String Close Failed")?;
        self.style
            .close_document()
            .context("Common Style Chain Close Failed")?;
        Ok(())
    }
}

// ########################### Share String ########################
impl CommonServices {
    pub(crate) fn get_string_id_mut(&mut self, value: String) -> AnyResult<String, AnyError> {
        self.share_string.get_string_id_mut(value)
    }
    pub(crate) fn get_string_id_value(&self, id: String) -> AnyResult<String, AnyError> {
        self.share_string.get_string_id_value(id)
    }
}

// ########################### Style ########################
impl CommonServices {
    pub(crate) fn get_style_id_mut(
        &mut self,
        style_setting: StyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        self.style.get_style_id_mut(style_setting)
    }
}
