use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    converters::ConverterUtil,
    element_dictionary::EXCEL_TYPE_COLLECTION,
    global::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing},
    log_elapsed,
    namespaces::SPREADSHEET_NS,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlAttribute, XmlDocument};

use super::CalculationChainPart;

impl_xml_part_lifecycle!(CalculationChainPart);

impl XmlDocumentPartClose for CalculationChainPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if let Some(office_document_ref) = self.office_document.upgrade() {
                    if !self.calculation_collection.is_empty() {
                        if let Some(xml_document) = self.xml_document.upgrade() {
                            let mut xml_document_mut = xml_document.try_borrow_mut().context(
                                "draviavemal-openxml_office::Failed to pull document handle",
                            )?;
                            let root_id = xml_document_mut.get_root_id();
                            for calculation in &self.calculation_collection {
                                let mut attributes = vec![
                                    XmlAttribute::new(
                                        "r".to_string(),
                                        calculation.cell_ref.clone(),
                                    ),
                                    XmlAttribute::new(
                                        "i".to_string(),
                                        calculation.sheet_id.to_string(),
                                    ),
                                ];
                                if let Some(is_leaf) = calculation.level_calcualtion {
                                    attributes.push(XmlAttribute::new(
                                        "l".to_string(),
                                        ConverterUtil::bool_xml_flag(&is_leaf),
                                    ));
                                }
                                if let Some(formula_type) = calculation.formula_type.as_ref() {
                                    attributes.push(XmlAttribute::new(
                                        "t".to_string(),
                                        formula_type.to_string(),
                                    ));
                                }
                                if let Some(share_formula) = calculation.share_formula {
                                    attributes.push(XmlAttribute::new(
                                        "s".to_string(),
                                        ConverterUtil::bool_xml_flag(&share_formula),
                                    ));
                                }
                                if let Some(array_formula) = calculation.array_formula {
                                    attributes.push(XmlAttribute::new(
                                        "a".to_string(),
                                        ConverterUtil::bool_xml_flag(&array_formula),
                                    ));
                                }
                                xml_document_mut
                                    .append_child_element_mut(root_id, "c", Some(attributes))
                                    .context(
                                        "draviavemal-openxml_office::Failed To Add Child Item",
                                    )?;
                            }
                        }
                        office_document_ref
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to Borrow Share Tree")?
                            .close_xml_document(&self.file_path)?;
                    } else if let Some(relationship_part) = self.parent_relationship_part.upgrade()
                    {
                        relationship_part
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed To pull parent relation ship part of Calc Chain")?
                            .delete_relationship_mut(&self.file_path);
                        office_document_ref
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to Borrow Share Tree")?
                            .delete_document_mut(&self.file_path);
                    }
                }
                Ok(())
            },
            "Close Calculation Chain"
        )
    }
}

impl XmlDocumentPartInitializing for CalculationChainPart {
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION
            .get("calc_chain")
            .context("Failed to read Excel type collection")?;
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_element_ns_mut(
                "calcChain",
                &NamespaceDeclaration {
                    default_alias: SPREADSHEET_NS.default_alias,
                    uri: SPREADSHEET_NS.uri,
                    alias_override: Some(""),
                },
                None,
            )
            .context("draviavemal-openxml_office::Create XML Root Element Failed")?;
        Ok((
            xml_document,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}
