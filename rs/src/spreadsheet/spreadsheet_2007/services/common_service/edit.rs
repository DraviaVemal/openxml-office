use crate::{
    global::traits::XmlDocumentPartClose,
    spreadsheet_2007::{
        services::{CalculationChain, CommonServices},
        CellStyleSetting, StyleId,
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};

impl CommonServices {
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

    pub(crate) fn get_string_id_mut(&mut self, value: String) -> AnyResult<String, AnyError> {
        self.share_string.get_string_id_mut(value)
    }

    pub(crate) fn get_string_id_value(&self, id: String) -> AnyResult<String, AnyError> {
        self.share_string.get_string_id_value(id)
    }

    pub(crate) fn get_string_id_value_ref(&self, id: &str) -> AnyResult<&str, AnyError> {
        self.share_string.get_string_id_value_ref(id)
    }

    pub(crate) fn get_style_id_mut(
        &mut self,
        style_setting: CellStyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        self.style.get_style_id_mut(style_setting)
    }

    pub(crate) fn add_replace_calculation_chain(
        &mut self,
        chain_item: CalculationChain,
    ) -> AnyResult<(), AnyError> {
        self.calculation_chain
            .add_replace_calculation_chain(chain_item)
    }
}
