use crate::converters::ConverterUtil;
use crate::element_dictionary::EXCEL_TYPE_COLLECTION;
use crate::global_2007::parts::RelationsPart;
use crate::global_2007::traits::{
    XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing,
};
use crate::log_elapsed;
use crate::{files::OfficeDocument, global_2007::traits::XmlDocumentPart};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlAttribute, XmlDocument};
use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
pub(crate) struct CalculationChain {
    pub(crate) cell_ref: String,
    pub(crate) sheet_id: u32,
    pub(crate) level_calcualtion: Option<bool>,
    pub(crate) formula_type: Option<String>,
    pub(crate) share_formula: Option<bool>,
    pub(crate) array_formula: Option<bool>,
}

#[derive(Debug)]
pub(crate) struct CalculationChainPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    parent_relationship_part: Weak<RefCell<RelationsPart>>,
    calculation_collection: Vec<CalculationChain>,
    xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
}

impl Drop for CalculationChainPart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for CalculationChainPart {}

impl XmlDocumentPartClose for CalculationChainPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if let Some(office_doc_ref) = self.office_document.upgrade() {
                    if self.calculation_collection.len() > 0 {
                        if let Some(xml_document) = self.xml_document.upgrade() {
                            let mut xml_doc_mut = xml_document
                                .try_borrow_mut()
                                .context("draviavemal-openxml_office::Failed to pull document handle")?;
                            let root_id = xml_doc_mut.get_root_id();
                            for calc_chain in self.calculation_collection.iter() {
                                let mut attributes = Vec::new();
                                attributes.push(XmlAttribute::new(
                                    "r".to_string(),
                                    calc_chain.cell_ref.to_string(),
                                ));
                                attributes.push(XmlAttribute::new(
                                    "i".to_string(),
                                    calc_chain.sheet_id.to_string(),
                                ));
                                if let Some(is_leaf) = calc_chain.level_calcualtion {
                                    attributes.push(XmlAttribute::new(
                                        "l".to_string(),
                                        ConverterUtil::bool_xml_flag(&is_leaf),
                                    ));
                                }
                                if let Some(formulat_type) = calc_chain.formula_type.as_ref() {
                                    attributes.push(XmlAttribute::new(
                                        "t".to_string(),
                                        formulat_type.to_string(),
                                    ));
                                }
                                if let Some(share_formula) = calc_chain.share_formula {
                                    attributes.push(XmlAttribute::new(
                                        "s".to_string(),
                                        ConverterUtil::bool_xml_flag(&share_formula),
                                    ));
                                }
                                if let Some(array_formula) = calc_chain.array_formula {
                                    attributes.push(XmlAttribute::new(
                                        "a".to_string(),
                                        ConverterUtil::bool_xml_flag(&array_formula),
                                    ));
                                }
                                xml_doc_mut
                                    .append_child_element_mut(root_id, "c", Some(attributes))
                                    .context("draviavemal-openxml_office::Failed To Add Child Item")?;
                            }
                        }
                        office_doc_ref
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to Borrow Share Tree")?
                            .close_xml_document(&self.file_path)?;
                    } else {
                        if let Some(relationship_part) = self.parent_relationship_part.upgrade() {
                            relationship_part
                                .try_borrow_mut()
                                .context("draviavemal-openxml_office::Failed To pull parent relation ship part of Calc Chain")?
                                .delete_relationship_mut(&self.file_path);
                            office_doc_ref
                                .try_borrow_mut()
                                .context("draviavemal-openxml_office::Failed to Borrow Share Tree")?
                                .delete_document_mut(&self.file_path);
                        }
                    }
                }
                Ok(())
            },
            "Close Calculation Chain"
        )
    }
}

impl XmlDocumentPartInitializing for CalculationChainPart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION.get("calc_chain").unwrap();
        let mut attributes: Vec<XmlAttribute> = Vec::new();
        attributes.push(XmlAttribute::new(
            "xmlns".to_string(),
            EXCEL_TYPE_COLLECTION
                .get("calc_chain")
                .unwrap()
                .schemas_namespace
                .to_string(),
        ));
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_element_mut("calcChain", Some(attributes))
            .context("draviavemal-openxml_office::Create XML Root Element Failed")?;
        Ok((
            xml_document,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

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
    fn get_calc_chain_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let calc_chain_content = EXCEL_TYPE_COLLECTION.get("calc_chain").unwrap();
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
            Err(AnyError::msg("draviavemal-openxml_office::Failed to upgrade relation part"))
        }
    }
    fn deserialise_calc_chain(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<CalculationChain>, AnyError> {
        let mut calculation_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::xml doc borrow failed")?;
            let root_id = xml_doc_mut.get_root_id();
            if let Some(element_ids) = xml_doc_mut
                .find_all_child(root_id, "c")
                .context("draviavemal-openxml_office::Failed to find calc chain elements")?
            {
                for element_id in element_ids {
                    let element = xml_doc_mut
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
                                .context("draviavemal-openxml_office::Failed to Convert Sheet ID")?, // Set Default Sheet id to 1
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
                    xml_doc_mut
                        .remove_element_mut(element_id)
                        .context("draviavemal-openxml_office::Falied to remove element")?;
                }
            }
        }
        Ok(calculation_collection)
    }

    pub(crate) fn add_replace_calculation_chain(
        &mut self,
        chain_item: CalculationChain,
    ) -> Result<(), AnyError> {
        if let Some(calc) = self.calculation_collection.iter_mut().find(|cell| {
            cell.cell_ref == chain_item.cell_ref && cell.sheet_id == chain_item.sheet_id
        }) {
            // Update existing Item
            *calc = chain_item;
        } else {
            self.calculation_collection.push(chain_item);
        }
        Ok(())
    }
}
