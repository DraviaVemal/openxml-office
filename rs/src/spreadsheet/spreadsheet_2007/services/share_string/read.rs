use crate::spreadsheet_2007::services::ShareStringPart;
use anyhow::{Context, Error as AnyError, Result as AnyResult};

impl ShareStringPart {
    pub(crate) fn get_string_id_value(&self, id: String) -> AnyResult<String, AnyError> {
        let actual_string = self
            .share_string_collection
            .get(
                id.parse::<usize>()
                    .context("draviavemal-openxml_office::Failed to parse Share String Id")?,
            )
            .context("draviavemal-openxml_office::Failed to find the value in the Vec")?;
        Ok(actual_string.to_string())
    }
    pub(crate) fn get_string_id_value_ref(&self, id: &str) -> AnyResult<&str, AnyError> {
        let index = id
            .parse::<usize>()
            .context("draviavemal-openxml_office::Failed to parse Share String Id")?;
        self.share_string_collection
            .get(index)
            .map(String::as_str)
            .context("draviavemal-openxml_office::Failed to find the value in the Vec")
    }
}
