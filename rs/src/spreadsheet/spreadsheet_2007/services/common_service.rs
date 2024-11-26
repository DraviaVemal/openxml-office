use crate::spreadsheet_2007::services::CalculationChain;
use crate::spreadsheet_2007::services::ShareString;
use crate::spreadsheet_2007::services::Style;

#[derive(Debug)]
pub struct CommonServices {
    calculation_chain: CalculationChain,
    share_string: ShareString,
    style: Style,
}

impl CommonServices {
    pub fn new(
        calculation_chain: CalculationChain,
        share_string: ShareString,
        style: Style,
    ) -> Self {
        Self {
            calculation_chain,
            share_string,
            style,
        }
    }
}
