use std::{cell::RefCell, rc::Weak};

use anyhow::{Context, Error as AnyError, Result as AnyResult};

use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{XmlDocumentPart, XmlDocumentPartInitializing},
    },
    spreadsheet_2007::services::CalculationChainPart,
};

impl XmlDocumentPart for CalculationChainPart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<CalculationChainPart, AnyError> {
        let file_name = CalculationChainPart::get_calc_chain_file_name(&parent_relationship_part)
            .context("draviavemal-openxml_office::Failed to pull calc chain file name")?
            .to_string();
        let mut xml_document =
            CalculationChainPart::get_xml_document(&office_document, &file_name)?;
        let calculation_collection =
            CalculationChainPart::deserialise_calc_chain(&mut xml_document)
                .context("draviavemal-openxml_office::Load Calculation Chain To DB Failed")?;
        Ok(CalculationChainPart {
            office_document,
            parent_relationship_part,
            calculation_collection,
            xml_document,
            file_path: file_name,
        })
    }
}

impl CalculationChainPart {
    pub(crate) fn get_calc_chain_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let calc_chain_content = EXCEL_TYPE_COLLECTION
            .get("calc_chain")
            .context("Failed to read Excel type collection")?;
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &calc_chain_content.schemas_type,
                    calc_chain_content,
                    None,
                    None,
                )
                .context("draviavemal-openxml_office::Pull Path From Existing File Failed")?)
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to upgrade relation part",
            ))
        }
    }
}
