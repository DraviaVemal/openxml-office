use std::{cell::RefCell, rc::Weak};

use anyhow::{Context, Error as AnyError, Result as AnyResult};

use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION, global_2007::parts::RelationsPart,
    spreadsheet_2007::WorkbookPart,
};

impl WorkbookPart {
    pub(crate) fn get_workbook_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let relationship_content = EXCEL_TYPE_COLLECTION.get("workbook").unwrap();
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &relationship_content.schemas_type,
                    relationship_content,
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

    pub(crate) fn list_sheet_names(&self) -> AnyResult<Vec<String>, AnyError> {
        Ok(self
            .sheet_collection
            .try_borrow()
            .context("draviavemal-openxml_office::Failed to pull Sheet Name Collection")?
            .iter()
            .map(|(sheet_name, _, _)| sheet_name.to_string())
            .collect::<Vec<String>>())
    }

    pub(crate) fn has_sheets(&self) -> AnyResult<bool, AnyError> {
        Ok(!self
            .sheet_collection
            .try_borrow()
            .context("draviavemal-openxml_office::Failed to pull Sheet Name Collection")?
            .is_empty())
    }

    pub(crate) fn for_each_sheet_name(
        &self,
        mut callback: impl FnMut(&str),
    ) -> AnyResult<(), AnyError> {
        let sheet_collection = self
            .sheet_collection
            .try_borrow()
            .context("draviavemal-openxml_office::Failed to pull Sheet Name Collection")?;
        for (sheet_name, _, _) in sheet_collection.iter() {
            callback(sheet_name);
        }
        Ok(())
    }
}
