use crate::{
    files::OfficeDocument,
    global_2007::{
        parts::{CorePropertiesPart, RelationsPart},
        traits::{XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush},
    },
};
use anyhow::{Context, Error as AnyError, Ok, Result as AnyResult};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Word {
    office_document: Rc<RefCell<OfficeDocument>>,
    root_relations: Rc<RefCell<RelationsPart>>,
    core_properties: CorePropertiesPart,
}

#[derive(Debug)]
pub struct WordPropertiesModel {
    pub is_editable: bool,
}

impl Default for WordPropertiesModel {
    fn default() -> WordPropertiesModel {
        WordPropertiesModel { is_editable: true }
    }
}

impl Word {
    /// Create new or clone source file to start working on Word
    pub fn new(
        file_name: Option<String>,
        _word_setting: WordPropertiesModel,
    ) -> AnyResult<Self, AnyError> {
        let office_document = Rc::new(RefCell::new(
            OfficeDocument::new(file_name.clone())
                .context("draviavemal-openxml_office::Creating Office Document Struct Failed")?,
        ));
        let root_relations = Rc::new(RefCell::new(
            RelationsPart::new(Rc::downgrade(&office_document), "_rels/.rels")
                .context("draviavemal-openxml_office::Initialize Root Relation Part failed")?,
        ));
        let core_properties = CorePropertiesPart::new(
            Rc::downgrade(&office_document),
            Rc::downgrade(&root_relations),
        )
        .context("draviavemal-openxml_office::Creating Core Property Part Failed.")?;
        Ok(Self {
            office_document,
            root_relations,
            core_properties,
        })
    }

    /// Save/Replace the current file into target destination
    pub fn save_as(self, file_name: &str) -> AnyResult<String, AnyError> {
        self.core_properties.flush()?;
        self.root_relations
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Failed To Pull Relation Handle")?
            .close_document()?;
        self.office_document
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Save Office Document handle Failed")?
            .save_as(file_name)
            .context("draviavemal-openxml_office::File Save Failed for the target path.")
    }
}
