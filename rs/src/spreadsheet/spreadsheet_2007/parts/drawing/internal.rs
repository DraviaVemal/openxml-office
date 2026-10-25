use crate::{
    element_dictionary::Content,
    files::OfficeDocument,
    global_2007::{
        parts::{DrawingPartGlobal, RelationsPart},
        traits::XmlDocumentPartInitializing,
    },
    log_elapsed,
    spreadsheet_2007::{services::CommonServices, DrawingPart},
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use phf::Map;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

impl DrawingPart {
    pub(crate) fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        worksheet_relationship_part: Weak<RefCell<RelationsPart>>,
        common_service: Weak<RefCell<CommonServices>>,
        type_collection: &Map<&'static str, &'static Content>,
    ) -> AnyResult<DrawingPart, AnyError> {
        let file_path =
            DrawingPart::get_drawing_file_name(&worksheet_relationship_part, type_collection)
                .context("draviavemal-openxml_office::Failed to pull worksheet file name")?;
        // Detect whether this drawing already exists (loaded) before the xml document
        // handle is created, since fetching the handle moves it out of the archive.
        let is_new = if let Some(office_document) = office_document.upgrade() {
            !office_document
                .try_borrow()
                .context("draviavemal-openxml_office::Failed to borrow office document")?
                .check_file_exist(file_path.clone())
        } else {
            true
        };
        let xml_document = DrawingPart::get_xml_document(&office_document, &file_path)?;
        let drawing_relationship_part = Rc::new(RefCell::new(
            RelationsPart::new(
                office_document.clone(),
                &format!(
                    "{}/_rels/{}.rels",
                    &file_path[..file_path.rfind('/').unwrap()],
                    file_path.rsplit('/').next().unwrap()
                ),
            )
            .context(
                "draviavemal-openxml_office::Creating Relation ship part for workbook failed.",
            )?,
        ));
        let anchor_collection = log_elapsed!(
            || {
                DrawingPart::deserialize_drawing(&xml_document)
                    .context("draviavemal-openxml_office::Failed to open drawing part and deserialize anchors")
            },
            "drawing Initialize Time"
        )?;
        let drawing_global = DrawingPartGlobal::new(drawing_relationship_part.clone())
            .context("Failed to create drawing global")?;
        Ok(DrawingPart {
            drawing_global,
            office_document,
            xml_document,
            common_service,
            worksheet_relationship_part,
            drawing_relationship_part,
            anchor_collection,
            is_new,
            file_path: file_path.to_string(),
        })
    }

    /// A drawing is considered empty when it was newly created this session and has
    /// not received any anchor/content. Loaded drawings are never treated as empty so
    /// they always round-trip.
    pub(crate) fn is_empty(&self) -> bool {
        self.is_new
            && self
                .anchor_collection
                .as_ref()
                .map(|anchors| anchors.is_empty())
                .unwrap_or(true)
    }

    fn get_drawing_file_name(
        worksheet_relationship_part: &Weak<RefCell<RelationsPart>>,
        type_collection: &Map<&'static str, &'static Content>,
    ) -> AnyResult<String, AnyError> {
        let drawing_content = type_collection
            .get("drawing")
            .context("Failed to read drawing type collection")?;
        if let Some(worksheet_relationship_part) = worksheet_relationship_part.upgrade() {
            Ok(worksheet_relationship_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &drawing_content.schemas_type,
                    drawing_content,
                    Some(format!("xl/{}", drawing_content.default_path)),
                    None,
                )
                .context("draviavemal-openxml_office::Pull Path From Existing File Failed")?)
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to upgrade relation part",
            ))
        }
    }
}
