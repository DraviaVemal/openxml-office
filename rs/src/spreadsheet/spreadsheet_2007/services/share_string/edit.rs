use anyhow::{Error as AnyError, Result as AnyResult};

use crate::spreadsheet_2007::services::ShareStringPart;

impl ShareStringPart {
    pub(crate) fn get_string_id_mut(&mut self, value: String) -> AnyResult<String, AnyError> {
        Ok(
            if let Some(position) = self
                .share_string_collection
                .iter()
                .position(|predicate| predicate == &value)
            {
                position.to_string()
            } else {
                self.share_string_collection.push(value);
                (self.share_string_collection.len() - 1).to_string()
            },
        )
    }
}
