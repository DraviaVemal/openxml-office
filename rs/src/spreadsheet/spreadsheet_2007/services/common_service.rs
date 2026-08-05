use crate::global_2007::service::MediaFiles;
use crate::global_2007::traits::XmlDocumentPartClose;
use crate::spreadsheet_2007::models::{StyleId, CellStyleSetting};
use crate::spreadsheet_2007::services::{
    CalculationChain, CalculationChainPart, ShareStringPart, StylePart,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};

#[derive(Debug)]
pub(crate) struct CommonServices {
    media_files: MediaFiles,
    calculation_chain: CalculationChainPart,
    share_string: ShareStringPart,
    style: StylePart,
}

impl CommonServices {
    pub(crate) fn new(
        media_files: MediaFiles,
        calculation_chain: CalculationChainPart,
        share_string: ShareStringPart,
        style: StylePart,
    ) -> Self {
        Self {
            media_files,
            calculation_chain,
            share_string,
            style,
        }
    }
    pub(crate) fn close_service(&mut self) -> AnyResult<(), AnyError> {
        self.calculation_chain
            .close_document()
            .context("draviavemal-openxml_office::Common Service Calculation Chain Close Failed")?;
        self.share_string
            .close_document()
            .context("draviavemal-openxml_office::Common Service Share String Close Failed")?;
        self.style
            .close_document()
            .context("draviavemal-openxml_office::Common Style Chain Close Failed")?;
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
        style_setting: CellStyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        self.style.get_style_id_mut(style_setting)
    }
}

// ########################### Calculation Chain ########################
impl CommonServices {
    pub(crate) fn add_replace_calculation_chain(
        &mut self,
        chain_item: CalculationChain,
    ) -> AnyResult<(), AnyError> {
        self.calculation_chain
            .add_replace_calculation_chain(chain_item)
    }
}
