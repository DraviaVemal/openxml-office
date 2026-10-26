use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use anyhow::{Context, Error as AnyError, Result as AnyResult};

use crate::{
    global_2007::files::OfficeDocument,
    global_2007::{
        parts::{RelationsPart, ThemePart},
        traits::{XmlDocumentPart, XmlDocumentPartInitializing},
    },
    log_elapsed,
    spreadsheet_2007::{
        services::{
            CalculationChainPart, CommonServices, ShareStringPart, SpreadsheetRelationshipRules,
            StylePart,
        },
        WorkbookPart,
    },
};

/// Trait implementation of XML Part - Only accessible within crate
impl XmlDocumentPart for WorkbookPart {
    /// Create workbook
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<WorkbookPart, AnyError> {
        log_elapsed!(
            || {
                let file_name = WorkbookPart::get_workbook_file_name(&parent_relationship_part)
                    .context("draviavemal-openxml_office::Failed to pull workbook file name")?
                    .to_string();
                let mut file_tree = WorkbookPart::get_xml_document(&office_document, &file_name)?;
                let workbook_relationship_part = Rc::new(RefCell::new(
                    RelationsPart::with_rules(
                        office_document.clone(),
                        &format!(
                            "{}/_rels/workbook.xml.rels",
                            &file_name[..file_name.rfind("/").unwrap()]
                        ),
                        Box::new(SpreadsheetRelationshipRules),
                    )
                    .context("draviavemal-openxml_office::Creating Relation ship part for workbook failed.")?,
                ));
                // Theme
                let theme_part = ThemePart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("draviavemal-openxml_office::Loading Theme Part Failed")?;
                // Share String
                let share_string = ShareStringPart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("draviavemal-openxml_office::Loading Share String Failed")?;
                // Calculation chain
                let calculation_chain = CalculationChainPart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("draviavemal-openxml_office::Loading Calculation Chain Failed")?;
                // Style
                let style = StylePart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("draviavemal-openxml_office::Loading Style Part Failed")?;
                let common_service = Rc::new(RefCell::new(CommonServices::new(
                    calculation_chain,
                    share_string,
                    style,
                )));
                let (sheet_collection, workbook_view) =
                    WorkbookPart::load_sheet_names(&mut file_tree)
                        .context("draviavemal-openxml_office::Loading Sheet Names Failed")?;
                Ok(WorkbookPart {
                    office_document,
                    xml_document: file_tree,
                    file_path: file_name,
                    common_service,
                    workbook_relationship_part,
                    theme_part,
                    sheet_collection: Rc::new(RefCell::new(sheet_collection)),
                    workbook_view,
                })
            },
            "Create New Workbook"
        )
    }
}
