use crate::spreadsheet_2007::services::{
    CalculationChainPart, CommonServices, ShareStringPart, StylePart,
};

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
}
