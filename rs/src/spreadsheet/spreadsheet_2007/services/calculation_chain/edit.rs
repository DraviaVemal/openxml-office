use anyhow::{Error as AnyError, Result as AnyResult};

use crate::spreadsheet_2007::services::{CalculationChain, CalculationChainPart};

impl CalculationChainPart {
    pub(crate) fn add_replace_calculation_chain(
        &mut self,
        chain_item: CalculationChain,
    ) -> AnyResult<(), AnyError> {
        if let Some(calculation_entry) = self.calculation_collection.iter_mut().find(|cell| {
            cell.cell_ref == chain_item.cell_ref && cell.sheet_id == chain_item.sheet_id
        }) {
            // Update existing Item
            *calculation_entry = chain_item;
        } else {
            self.calculation_collection.push(chain_item);
        }
        Ok(())
    }
}
