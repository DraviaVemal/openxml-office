use crate::converters::ConverterUtil;
use crate::element_dictionary::EXCEL_TYPE_COLLECTION;
use crate::global_2007::parts::RelationsPart;
use crate::global_2007::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing};
use crate::log_elapsed;
use crate::{
    files::{OfficeDocument, XmlDocument},
    global_2007::traits::XmlDocumentPart,
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, collections::HashMap, rc::Weak};

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
pub struct CalculationChainPart {
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
                                .context("Failed to pull document handle")?;
                            for calc_chain in self.calculation_collection.iter() {
                                let mut attributes = HashMap::new();
                                attributes.insert("r".to_string(), calc_chain.cell_ref.to_string());
                                attributes.insert("i".to_string(), calc_chain.sheet_id.to_string());
                                if let Some(is_leaf) = calc_chain.level_calcualtion {
                                    attributes.insert(
                                        "l".to_string(),
                                        ConverterUtil::bool_xml_flag(&is_leaf),
                                    );
                                }
                                if let Some(formulat_type) = calc_chain.formula_type.as_ref() {
                                    attributes.insert("t".to_string(), formulat_type.to_string());
                                }
                                if let Some(share_formula) = calc_chain.share_formula {
                                    attributes.insert(
                                        "s".to_string(),
                                        ConverterUtil::bool_xml_flag(&share_formula),
                                    );
                                }
                                if let Some(array_formula) = calc_chain.array_formula {
                                    attributes.insert(
                                        "a".to_string(),
                                        ConverterUtil::bool_xml_flag(&array_formula),
                                    );
                                }
                                xml_doc_mut
                                    .append_child_mut("c", None)
                                    .context("Failed To Add Child Item")?
                                    .set_attribute_mut(attributes)?;
                            }
                        }
                        office_doc_ref
                            .try_borrow_mut()
                            .context("Failed to Borrow Share Tree")?
                            .close_xml_document(&self.file_path)?;
                    } else {
                        if let Some(relationship_part) = self.parent_relationship_part.upgrade() {
                            relationship_part
                                .try_borrow_mut()
                                .context("Failed To pull parent relation ship part of Calc Chain")?
                                .delete_relationship_mut(&self.file_path);
                            office_doc_ref
                                .try_borrow_mut()
                                .context("Failed to Borrow Share Tree")?
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
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(
            "xmlns".to_string(),
            EXCEL_TYPE_COLLECTION
                .get("calc_chain")
                .unwrap()
                .schemas_namespace
                .to_string(),
        );
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_mut("calcChain")
            .context("Create XML Root Element Failed")?
            .set_attribute_mut(attributes)
            .context("Set Attribute Failed")?;
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
    ) -> AnyResult<Self, AnyError> {
        let file_name = Self::get_calc_chain_file_name(&parent_relationship_part)
            .context("Failed to pull calc chain file name")?
            .to_string();
        let mut xml_document = Self::get_xml_document(&office_document, &file_name)?;
        let calculation_collection = Self::deserialise_calc_chain(&mut xml_document)
            .context("Load Calculation Chain To DB Failed")?;
        Ok(Self {
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
                .context("Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &calc_chain_content.schemas_type,
                    calc_chain_content,
                    None,
                    None,
                )
                .context("Pull Path From Existing File Failed")?)
        } else {
            Err(anyhow!("Failed to upgrade relation part"))
        }
    }
    fn deserialise_calc_chain(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<CalculationChain>, AnyError> {
        let mut calculation_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("xml doc borrow failed")?;
            if let Some(elements) = xml_doc_mut.pop_elements_by_tag_mut("c", None) {
                for element in elements {
                    if let Some(attributes) = element.get_attribute() {
                        calculation_collection.push(CalculationChain {
                            cell_ref: attributes["r"].clone(),
                            sheet_id: attributes
                                .get("i")
                                .unwrap_or(&"1".to_string())
                                .clone()
                                .parse::<u32>()
                                .context("Failed to Convert Sheet ID")?, // Set Default Sheet id to 1
                            level_calcualtion: if let Some(is_leaf) = attributes.get("l").cloned() {
                                Some(ConverterUtil::normalize_bool_property_bool(&is_leaf))
                            } else {
                                None
                            },
                            formula_type: attributes.get("t").cloned(),
                            share_formula: if let Some(is_share_formula) =
                                attributes.get("s").cloned()
                            {
                                Some(ConverterUtil::normalize_bool_property_bool(
                                    &is_share_formula,
                                ))
                            } else {
                                None
                            },
                            array_formula: if let Some(is_array_formula) =
                                attributes.get("a").cloned()
                            {
                                Some(ConverterUtil::normalize_bool_property_bool(
                                    &is_array_formula,
                                ))
                            } else {
                                None
                            },
                        });
                    }
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
