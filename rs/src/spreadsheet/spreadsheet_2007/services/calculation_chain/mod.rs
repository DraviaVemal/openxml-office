use crate::{
    converters::ConverterUtil,
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{
            XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush,
            XmlDocumentPartInitializing,
        },
    },
    log_elapsed,
    namespaces::SPREADSHEET_NS,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlAttribute, XmlDocument};
use std::{cell::RefCell, rc::Weak};

mod deserialize;
mod edit;
mod internal;
mod model;

pub(crate) use model::{CalculationChain, CalculationChainPart};

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
                            let mut xml_doc_mut = xml_document.try_borrow_mut().context(
                                "draviavemal-openxml_office::Failed to pull document handle",
                            )?;
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
                                    .context(
                                        "draviavemal-openxml_office::Failed To Add Child Item",
                                    )?;
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
