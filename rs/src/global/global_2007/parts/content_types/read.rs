use crate::global_2007::parts::content_types::ContentTypesPart;
use anyhow::{Context, Error as AnyError, Result as AnyResult};

impl ContentTypesPart {
    /// Read every `Default` (extension, type) and `Override` (part name, type) entry.
    pub(crate) fn read_all(
        &mut self,
    ) -> AnyResult<(Vec<(String, String)>, Vec<(String, String)>), AnyError> {
        let root_id = self.xml_document.get_root_id();
        let mut defaults: Vec<(String, String)> = Vec::new();
        let mut overrides: Vec<(String, String)> = Vec::new();
        for (tag, key_attribute, collection) in [
            ("Default", "Extension", &mut defaults),
            ("Override", "PartName", &mut overrides),
        ] {
            if let Some(ids) = self
                .xml_document
                .find_all_child(root_id, tag)
                .context("draviavemal-openxml_office::Failed to find content type elements")?
            {
                for id in ids {
                    let element = self
                        .xml_document
                        .get_element(id)
                        .context("draviavemal-openxml_office::Element not Found")?;
                    if let (Some(key), Some(content_type)) = (
                        element.get_attribute(key_attribute),
                        element.get_attribute("ContentType"),
                    ) {
                        collection.push((
                            key.get_value().to_string(),
                            content_type.get_value().to_string(),
                        ));
                    }
                }
            }
        }
        Ok((defaults, overrides))
    }
}
