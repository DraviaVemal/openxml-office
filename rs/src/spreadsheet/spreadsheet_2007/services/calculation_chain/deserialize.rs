use std::{cell::RefCell, rc::Weak};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::XmlDocument;

use crate::{
    converters::ConverterUtil,
    spreadsheet_2007::services::{CalculationChain, CalculationChainPart},
};

impl CalculationChainPart {
    pub(crate) fn deserialise_calc_chain(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<CalculationChain>, AnyError> {
        let mut calculation_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_document_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::xml doc borrow failed")?;
            let root_id = xml_document_mut.get_root_id();
            if let Some(element_ids) = xml_document_mut
                .find_all_child(root_id, "c")
                .context("draviavemal-openxml_office::Failed to find calc chain elements")?
            {
                for element_id in element_ids {
                    let element = xml_document_mut
                        .get_element(element_id)
                        .context("draviavemal-openxml_office::Failed to pull calc chain element")?;
                    if let Some(cell_ref) = element.get_attribute("r") {
                        calculation_collection.push(CalculationChain {
                            cell_ref: cell_ref.get_value().to_string(),
                            sheet_id: element
                                .get_attribute("i")
                                .map(|attribute| attribute.get_value().to_string())
                                .unwrap_or_else(|| "1".to_string())
                                .parse::<u32>()
                                .context(
                                    "draviavemal-openxml_office::Failed to Convert Sheet ID",
                                )?, // Set Default Sheet id to 1
                            level_calcualtion: element.get_attribute("l").map(|attribute| {
                                ConverterUtil::normalize_bool_property_bool(attribute.get_value())
                            }),
                            formula_type: element
                                .get_attribute("t")
                                .map(|attribute| attribute.get_value().to_string()),
                            share_formula: element.get_attribute("s").map(|attribute| {
                                ConverterUtil::normalize_bool_property_bool(attribute.get_value())
                            }),
                            array_formula: element.get_attribute("a").map(|attribute| {
                                ConverterUtil::normalize_bool_property_bool(attribute.get_value())
                            }),
                        });
                    }
                    xml_document_mut
                        .remove_element_mut(element_id)
                        .context("draviavemal-openxml_office::Falied to remove element")?;
                }
            }
        }
        Ok(calculation_collection)
    }
}
