use crate::spreadsheet_2007::services::{CalculationChainPart, ShareStringPart, StylePart};

#[derive(Debug)]
pub(crate) struct CommonServices {
    pub(super) calculation_chain: CalculationChainPart,
    pub(super) share_string: ShareStringPart,
    pub(super) style: StylePart,
}
