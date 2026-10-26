// TODO: Move archive I/O to background tasks.

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument, XmlSerializer};
use log::{debug, warn};
use rayon::prelude::*;
use zip::{write::SimpleFileOptions, CompressionMethod, DateTime, ZipArchive, ZipWriter};

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    env,
    fmt::{Debug, Formatter},
    fs::{canonicalize, File},
    io::{BufReader, BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

use crate::{
    files::reorganize::{plan_layout, LayoutPlan},
    global_2007::parts::ContentTypesPart,
};

const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
const COMPRESSION_LEVEL: i64 = 4;
/// Limit initial allocation from untrusted zip metadata.
const MAX_PREALLOCATION: u64 = 64 * 1024 * 1024;

#[derive(Debug, Clone)]
pub(crate) struct ExtractedDocument {
    document_handle: Rc<RefCell<XmlDocument>>,
    content_type: Option<String>,
    file_extension: String,
    extension_type: String,
}

/// Indexed package part with an optional edited payload.
#[derive(Debug, Clone)]
struct Entry {
    source_index: Option<usize>,
    data: Option<Vec<u8>>,
}

struct SourceArchive {
    path: PathBuf,
    archive: ZipArchive<BufReader<File>>,
}

impl Debug for SourceArchive {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceArchive")
            .field("path", &self.path)
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct OfficeDocument {
    /// Original archive for lazy part reads.
    source: Option<SourceArchive>,
    /// Part name to source index or edited bytes.
    entries: BTreeMap<String, Entry>,
    /// Open XML documents.
    xml_document_collection: HashMap<String, ExtractedDocument>,
    /// Part-number counters.
    part_counter: RefCell<HashMap<String, usize>>,
    /// Default content types by extension.
    defaults: BTreeMap<String, String>,
    /// Override content types by part name.
    overrides: BTreeMap<String, String>,
    /// Renumber supported part families on save.
    reorganize_parts: bool,
}

fn extension_of(part_name: &str) -> &str {
    let file_name = part_name.rsplit('/').next().unwrap_or(part_name);
    file_name
        .rfind('.')
        .map(|index| &file_name[index + 1..])
        .unwrap_or("")
}

fn validate_part_name(name: &str) -> AnyResult<(), AnyError> {
    if name.starts_with('/')
        || name.contains('\\')
        || name.split('/').any(|segment| segment == "..")
    {
        return Err(AnyError::msg(format!(
            "draviavemal-openxml_office::Unsafe part name in archive : {}",
            name
        )));
    }
    Ok(())
}

fn part_options(size: usize) -> SimpleFileOptions {
    SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(COMPRESSION_LEVEL))
        .last_modified_time(DateTime::default())
        .large_file(size as u64 > u32::MAX as u64)
}

/// Deflate one entry for raw copying into the output archive.
fn deflate_single(name: &str, data: &[u8]) -> AnyResult<Vec<u8>, AnyError> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::with_capacity(data.len() / 2 + 128)));
    writer
        .start_file(name, part_options(data.len()))
        .context("draviavemal-openxml_office::Zip File Write Start Fail")?;
    writer
        .write_all(data)
        .context("draviavemal-openxml_office::Writing data to Zip")?;
    Ok(writer
        .finish()
        .context("draviavemal-openxml_office::Zip Entry Close Failed")?
        .into_inner())
}

enum Payload<'a> {
    /// Source entry copied without recompression.
    Raw(usize),
    Bytes(&'a [u8]),
}

impl OfficeDocument {
    /// Create or clone a document.
    pub(crate) fn new(file_path: Option<String>) -> AnyResult<OfficeDocument, AnyError> {
        let mut document = OfficeDocument {
            source: None,
            entries: BTreeMap::new(),
            xml_document_collection: HashMap::new(),
            part_counter: RefCell::new(HashMap::new()),
            defaults: BTreeMap::new(),
            overrides: BTreeMap::new(),
            reorganize_parts: true,
        };
        if let Some(file_path) = file_path {
            document.open_source(&file_path).context(
                "draviavemal-openxml_office::Load OpenXML Archive Into deserializing Failed",
            )?;
        }
        Ok(document)
    }

    /// Index metadata and load content types.
    fn open_source(&mut self, file_path: &str) -> AnyResult<(), AnyError> {
        let file = File::open(file_path)
            .context("draviavemal-openxml_office::Open Existing archive File")?;
        let mut archive = ZipArchive::new(BufReader::new(file))
            .context("draviavemal-openxml_office::Archive read Failed")?;
        let mut content_types = Vec::new();
        archive
            .by_name(CONTENT_TYPES_PART)
            .context("draviavemal-openxml_office::Read [Content_Types].xml Failed")?
            .read_to_end(&mut content_types)
            .context("draviavemal-openxml_office::Failed to uncompress [Content_Types].xml")?;
        let (defaults, overrides) = ContentTypesPart::new(content_types)
            .context("draviavemal-openxml_office::Decoding Content Type Failed")?
            .read_all()
            .context("draviavemal-openxml_office::Failed to read Content Types")?;
        for (extension, content_type) in defaults {
            if !extension.is_empty() {
                self.defaults
                    .insert(extension.to_ascii_lowercase(), content_type);
            }
        }
        self.overrides.extend(overrides);
        for index in 0..archive.len() {
            let name = archive
                .by_index_raw(index)
                .context("draviavemal-openxml_office::Zip entry read Failed")?
                .name()
                .to_string();
            if name.ends_with('/') || name == CONTENT_TYPES_PART {
                continue;
            }
            validate_part_name(&name)?;
            self.entries.insert(
                name,
                Entry {
                    source_index: Some(index),
                    data: None,
                },
            );
        }
        self.source = Some(SourceArchive {
            path: PathBuf::from(file_path),
            archive,
        });
        Ok(())
    }

    /// Configure save-time part renumbering.
    #[allow(dead_code)]
    pub(crate) fn set_reorganize_parts(&mut self, reorganize_parts: bool) {
        self.reorganize_parts = reorganize_parts;
    }

    fn read_entry_bytes(&mut self, name: &str) -> AnyResult<Option<Vec<u8>>, AnyError> {
        let Some(entry) = self.entries.get(name) else {
            return Ok(None);
        };
        if let Some(data) = &entry.data {
            return Ok(Some(data.clone()));
        }
        let index = entry
            .source_index
            .context("draviavemal-openxml_office::Part has no content")?;
        let source = self
            .source
            .as_mut()
            .context("draviavemal-openxml_office::Source archive is closed")?;
        let mut file = source
            .archive
            .by_index(index)
            .context("draviavemal-openxml_office::Zip file extract failed")?;
        let mut content = Vec::with_capacity(file.size().min(MAX_PREALLOCATION) as usize);
        file.read_to_end(&mut content)
            .context("draviavemal-openxml_office::File Uncompressed failed")?;
        Ok(Some(content))
    }

    pub(crate) fn check_file_exist(&self, file_path: String) -> bool {
        self.entries.contains_key(&file_path)
    }

    pub(crate) fn get_next_part_number(
        &self,
        dir_path: &str,
        base_name: &str,
        extension: &str,
    ) -> usize {
        let key = format!("{}/{}.{}", dir_path, base_name, extension);
        let mut part_counter = self.part_counter.borrow_mut();
        let counter = part_counter.entry(key).or_insert(0);
        loop {
            *counter += 1;
            let candidate = format!("{}/{}{}.{}", dir_path, base_name, *counter, extension);
            if !self.entries.contains_key(&candidate)
                && !self.xml_document_collection.contains_key(&candidate)
            {
                return *counter;
            }
        }
    }

    pub(crate) fn delete_document_mut(&mut self, file_name: &str) {
        self.xml_document_collection.remove(file_name);
        self.entries.remove(file_name);
        self.overrides.remove(&format!("/{}", file_name));
    }

    pub(crate) fn get_xml_document_ref(
        &mut self,
        file_name: &str,
        content_type: Option<String>,
        file_extension: String,
        extension_type: String,
        xml_tree: XmlDocument,
    ) -> AnyResult<Weak<RefCell<XmlDocument>>, AnyError> {
        let ref_xml_document = Rc::new(RefCell::new(xml_tree));
        let weak_xml_document = Rc::downgrade(&ref_xml_document);
        self.xml_document_collection.insert(
            file_name.to_string(),
            ExtractedDocument {
                document_handle: ref_xml_document,
                content_type,
                file_extension,
                extension_type,
            },
        );
        Ok(weak_xml_document)
    }

    /// Load and parse one XML part.
    pub(crate) fn get_xml_tree_mut(
        &mut self,
        file_path: &str,
    ) -> AnyResult<Option<(XmlDocument, Option<String>, String, String)>, AnyError> {
        if self.xml_document_collection.contains_key(file_path) {
            return Err(AnyError::msg(
                "draviavemal-openxml_office::Please close the Existing object before creating new handle",
            ));
        }
        let Some(content) = self.read_entry_bytes(file_path)? else {
            return Ok(None);
        };
        let xml_tree = XmlDeserializer::vec_to_xml_doc_tree(content)
            .context("draviavemal-openxml_office::Xml Struct Serializer Failed")?;
        let file_extension = extension_of(file_path).to_string();
        let extension_type = self
            .defaults
            .get(&file_extension.to_ascii_lowercase())
            .cloned()
            .unwrap_or_default();
        let content_type = self.overrides.get(&format!("/{}", file_path)).cloned();
        Ok(Some((
            xml_tree,
            content_type,
            file_extension,
            extension_type,
        )))
    }

    /// Serialize and close one XML part.
    pub(crate) fn close_xml_document(&mut self, file_path: &str) -> AnyResult<(), AnyError> {
        if let Some(extracted_document) = self.xml_document_collection.remove(file_path) {
            let data = {
                let mut xml_doc_mut = extracted_document
                    .document_handle
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to get document handle")?;
                XmlSerializer::xml_tree_to_vec(&mut xml_doc_mut).context(format!(
                    "draviavemal-openxml_office::Failed Xml Tree to String content, File : {}",
                    file_path
                ))?
            };
            if !extracted_document.file_extension.is_empty()
                && !extracted_document.extension_type.is_empty()
            {
                self.defaults.insert(
                    extracted_document.file_extension.to_ascii_lowercase(),
                    extracted_document.extension_type,
                );
            }
            let override_key = format!("/{}", file_path);
            match extracted_document.content_type {
                Some(content_type) => {
                    self.overrides.insert(override_key, content_type);
                }
                None => {
                    self.overrides.remove(&override_key);
                }
            }
            let entry = self.entries.entry(file_path.to_string()).or_insert(Entry {
                source_index: None,
                data: None,
            });
            entry.data = Some(data);
        }
        Ok(())
    }

    /// Plan output order and relationship changes.
    fn plan_layout(&mut self) -> LayoutPlan {
        let names: Vec<String> = self.entries.keys().cloned().collect();
        let renumber = self.reorganize_parts;
        let result = plan_layout(&names, renumber, &mut |name| {
            self.read_entry_bytes(name)?
                .with_context(|| format!("draviavemal-openxml_office::Missing part : {}", name))
        });
        match result {
            Ok(plan) => plan,
            Err(error) => {
                warn!(
                    "draviavemal-openxml_office::Part reorganizing skipped. {:#}",
                    error
                );
                let mut order = names;
                order.sort_by_key(|name| (name != "_rels/.rels", name.clone()));
                LayoutPlan {
                    order,
                    ..LayoutPlan::default()
                }
            }
        }
    }

    /// Build content types for output names.
    fn build_content_types(&self, plan: &LayoutPlan) -> AnyResult<Vec<u8>, AnyError> {
        let mut used_defaults: BTreeMap<String, String> = BTreeMap::new();
        let mut final_overrides: BTreeMap<String, String> = BTreeMap::new();
        for old_name in &plan.order {
            let new_name = plan.rename.get(old_name).unwrap_or(old_name);
            let extension = extension_of(new_name).to_ascii_lowercase();
            let default_type = if extension.is_empty() {
                None
            } else {
                self.defaults.get(&extension)
            };
            match self.overrides.get(&format!("/{}", old_name)) {
                Some(content_type) if Some(content_type) != default_type => {
                    final_overrides.insert(format!("/{}", new_name), content_type.clone());
                }
                Some(content_type) => {
                    used_defaults.insert(extension, content_type.clone());
                }
                None => match default_type {
                    Some(content_type) => {
                        used_defaults.insert(extension, content_type.clone());
                    }
                    None => warn!(
                        "draviavemal-openxml_office::No content type found for part : {}",
                        new_name
                    ),
                },
            }
        }
        ContentTypesPart::create_xml_file(
            used_defaults.into_iter().collect(),
            final_overrides.into_iter().collect(),
        )
        .context("draviavemal-openxml_office::Creating Content Type XML Failed")
    }

    /// Save the current document.
    pub(crate) fn save_as(&mut self, file_path: &str) -> AnyResult<String, AnyError> {
        if !self.xml_document_collection.is_empty() {
            for file in self.xml_document_collection.keys() {
                debug!("draviavemal-openxml_office::Document still open : {file}");
            }
            return Err(AnyError::msg(
                "draviavemal-openxml_office::Please close all the open document before saving the file",
            ));
        }
        let mut target = Path::new(file_path).to_path_buf();
        if !target.is_absolute() {
            target = env::current_dir()?.join(target);
        }
        let parent = target
            .parent()
            .context("draviavemal-openxml_office::Save File Target has no parent folder")?
            .to_path_buf();

        let plan = self.plan_layout();
        let content_types = self.build_content_types(&plan)?;

        let mut parts: Vec<(String, Payload)> = Vec::with_capacity(plan.order.len() + 1);
        parts.push((
            CONTENT_TYPES_PART.to_string(),
            Payload::Bytes(&content_types),
        ));
        for old_name in &plan.order {
            let entry = self.entries.get(old_name).with_context(|| {
                format!("draviavemal-openxml_office::Missing part : {}", old_name)
            })?;
            let new_name = plan.rename.get(old_name).unwrap_or(old_name).clone();
            let payload = if let Some(rewritten) = plan.rewritten_rels.get(old_name) {
                Payload::Bytes(rewritten)
            } else if let Some(data) = &entry.data {
                Payload::Bytes(data)
            } else {
                Payload::Raw(entry.source_index.with_context(|| {
                    format!(
                        "draviavemal-openxml_office::Part has no content : {}",
                        old_name
                    )
                })?)
            };
            parts.push((new_name, payload));
        }

        // Keep replacement atomic.
        let mut temp = tempfile::NamedTempFile::new_in(&parent)
            .context("draviavemal-openxml_office::Create Save File Failed")?;
        let mut zip_writer = ZipWriter::new(BufWriter::new(
            temp.as_file()
                .try_clone()
                .context("draviavemal-openxml_office::Create Save File Failed")?,
        ));
        // Bound parallel compression memory.
        let batch_size = (rayon::current_num_threads() * 4).max(8);
        for batch in parts.chunks(batch_size) {
            let compressed: Vec<Option<Vec<u8>>> = batch
                .par_iter()
                .map(|(name, payload)| match payload {
                    Payload::Bytes(data) => deflate_single(name, data).map(Some),
                    Payload::Raw(_) => Ok(None),
                })
                .collect::<AnyResult<Vec<_>, AnyError>>()?;
            for ((name, payload), packed) in batch.iter().zip(compressed) {
                match (payload, packed) {
                    (Payload::Raw(index), _) => {
                        let source = self
                            .source
                            .as_mut()
                            .context("draviavemal-openxml_office::Source archive is closed")?;
                        let file = source
                            .archive
                            .by_index_raw(*index)
                            .context("draviavemal-openxml_office::Zip entry read Failed")?;
                        zip_writer
                            .raw_copy_file_rename(file, name.as_str())
                            .context("draviavemal-openxml_office::Zip entry copy Failed")?;
                    }
                    (Payload::Bytes(_), Some(packed)) => {
                        let mut single = ZipArchive::new(Cursor::new(packed))
                            .context("draviavemal-openxml_office::Zip entry read Failed")?;
                        let file = single
                            .by_index_raw(0)
                            .context("draviavemal-openxml_office::Zip entry read Failed")?;
                        zip_writer
                            .raw_copy_file(file)
                            .context("draviavemal-openxml_office::Zip entry copy Failed")?;
                    }
                    (Payload::Bytes(_), None) => {
                        return Err(AnyError::msg(
                            "draviavemal-openxml_office::Compressed content missing",
                        ));
                    }
                }
            }
        }
        zip_writer
            .finish()
            .context("draviavemal-openxml_office::Zip Close Failed")?
            .flush()
            .context("draviavemal-openxml_office::Save File Write Failed")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&target)
                .map(|existing| existing.permissions().mode() & 0o7777)
                .unwrap_or(0o644);
            temp.as_file_mut()
                .set_permissions(std::fs::Permissions::from_mode(mode))
                .context("draviavemal-openxml_office::Save File permission Failed")?;
        }

        // Release the source before replacement on Windows.
        if cfg!(windows) {
            let same_file = match (
                self.source
                    .as_ref()
                    .and_then(|source| canonicalize(&source.path).ok()),
                canonicalize(&target).ok(),
            ) {
                (Some(source), Some(destination)) => source == destination,
                _ => false,
            };
            if same_file {
                self.source = None;
            }
        }
        temp.persist(&target).map_err(|error| {
            AnyError::msg(format!(
                "draviavemal-openxml_office::Replace Save File Target Failed. {}",
                error
            ))
        })?;
        Ok(target.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REL_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
    const SHEET_TYPE: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
    const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
    const BOOK_TYPE: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

    fn rels(items: &[(&str, &str, Option<&str>)]) -> String {
        let mut xml = format!(
            "<?xml version=\"1.0\"?><Relationships xmlns=\"{}\">",
            REL_NS
        );
        for (id, target, mode) in items {
            match mode {
                Some(mode) => xml.push_str(&format!(
                    "<Relationship Id=\"{}\" Type=\"t\" Target=\"{}\" TargetMode=\"{}\"/>",
                    id, target, mode
                )),
                None => xml.push_str(&format!(
                    "<Relationship Id=\"{}\" Type=\"t\" Target=\"{}\"/>",
                    id, target
                )),
            }
        }
        xml.push_str("</Relationships>");
        xml
    }

    /// Workbook with gaps and nonnumeric tab order.
    fn sample_parts() -> Vec<(String, String)> {
        let content_types = format!(
            "<?xml version=\"1.0\"?><Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\
<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
<Default Extension=\"xml\" ContentType=\"application/xml\"/>\
<Override PartName=\"/xl/workbook.xml\" ContentType=\"{BOOK_TYPE}\"/>\
<Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{SHEET_TYPE}\"/>\
<Override PartName=\"/xl/worksheets/sheet3.xml\" ContentType=\"{SHEET_TYPE}\"/>\
<Override PartName=\"/xl/worksheets/sheet7.xml\" ContentType=\"{SHEET_TYPE}\"/>\
<Override PartName=\"/xl/drawings/drawing5.xml\" ContentType=\"{DRAWING_TYPE}\"/></Types>"
        );
        vec![
            (CONTENT_TYPES_PART.into(), content_types),
            (
                "_rels/.rels".into(),
                rels(&[("rId1", "xl/workbook.xml", None)]),
            ),
            (
                "xl/workbook.xml".into(),
                "<workbook xmlns:r=\"r\"><sheets><sheet name=\"A\" r:id=\"rId3\"/><sheet name=\"B\" r:id=\"rId1\"/><sheet name=\"C\" r:id=\"rId2\"/></sheets></workbook>".into(),
            ),
            (
                "xl/_rels/workbook.xml.rels".into(),
                rels(&[
                    ("rId1", "worksheets/sheet1.xml", None),
                    ("rId2", "/xl/worksheets/sheet3.xml", None),
                    ("rId3", "worksheets/sheet7.xml", None),
                ]),
            ),
            ("xl/worksheets/sheet1.xml".into(), "<s>one</s>".into()),
            ("xl/worksheets/sheet3.xml".into(), "<s>three</s>".into()),
            ("xl/worksheets/sheet7.xml".into(), "<s>seven</s>".into()),
            (
                "xl/worksheets/_rels/sheet7.xml.rels".into(),
                rels(&[
                    ("rId1", "../drawings/drawing5.xml#frag", None),
                    ("rId2", "https://example.com/sheet7.xml", Some("External")),
                ]),
            ),
            ("xl/drawings/drawing5.xml".into(), "<d/>".into()),
        ]
    }

    fn write_zip(path: &Path, parts: &[(String, String)]) {
        let mut writer = ZipWriter::new(File::create(path).unwrap());
        for (name, content) in parts {
            writer
                .start_file(name, SimpleFileOptions::default())
                .unwrap();
            writer.write_all(content.as_bytes()).unwrap();
        }
        writer.finish().unwrap();
    }

    fn read_zip(path: &Path) -> (Vec<String>, HashMap<String, String>) {
        let mut archive = ZipArchive::new(File::open(path).unwrap()).unwrap();
        let mut names = Vec::new();
        let mut content = HashMap::new();
        for index in 0..archive.len() {
            let mut file = archive.by_index(index).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            names.push(file.name().to_string());
            content.insert(file.name().to_string(), text);
        }
        (names, content)
    }

    fn saved(
        source: &Path,
        output: &Path,
        reorganize: bool,
    ) -> (Vec<String>, HashMap<String, String>) {
        let mut document = OfficeDocument::new(Some(source.to_string_lossy().into())).unwrap();
        document.set_reorganize_parts(reorganize);
        document.save_as(&output.to_string_lossy()).unwrap();
        read_zip(output)
    }

    #[test]
    fn open_is_lazy_and_untouched_parts_survive_byte_identical() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("in.xlsx");
        write_zip(&source, &sample_parts());

        let document = OfficeDocument::new(Some(source.to_string_lossy().into())).unwrap();
        assert!(document.entries.values().all(|entry| entry.data.is_none()));
        assert_eq!(document.entries.len(), sample_parts().len() - 1);

        let (names, content) = saved(&source, &dir.path().join("out.xlsx"), false);
        assert_eq!(names[0], CONTENT_TYPES_PART);
        assert_eq!(names[1], "_rels/.rels");
        for (name, text) in sample_parts()
            .iter()
            .filter(|(name, _)| name != CONTENT_TYPES_PART)
        {
            assert_eq!(content.get(name), Some(text), "part {name} changed");
        }
        assert!(content[CONTENT_TYPES_PART].contains("/xl/worksheets/sheet7.xml"));
    }

    #[test]
    fn reorganize_renumbers_in_tab_order_and_keeps_relationships_valid() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("in.xlsx");
        write_zip(&source, &sample_parts());

        let (names, content) = saved(&source, &dir.path().join("out.xlsx"), true);
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            vec![
                "[Content_Types].xml",
                "_rels/.rels",
                "xl/_rels/workbook.xml.rels",
                "xl/drawings/drawing1.xml",
                "xl/workbook.xml",
                "xl/worksheets/_rels/sheet1.xml.rels",
                "xl/worksheets/sheet1.xml",
                "xl/worksheets/sheet2.xml",
                "xl/worksheets/sheet3.xml",
            ]
        );
        // Renumber in workbook tab order.
        assert_eq!(content["xl/worksheets/sheet1.xml"], "<s>seven</s>");
        assert_eq!(content["xl/worksheets/sheet2.xml"], "<s>one</s>");
        assert_eq!(content["xl/worksheets/sheet3.xml"], "<s>three</s>");
        assert_eq!(content["xl/workbook.xml"], sample_parts()[2].1);

        let book_rels = &content["xl/_rels/workbook.xml.rels"];
        assert!(book_rels.contains("Id=\"rId3\" Type=\"t\" Target=\"worksheets/sheet1.xml\""));
        assert!(book_rels.contains("Id=\"rId1\" Type=\"t\" Target=\"worksheets/sheet2.xml\""));
        assert!(book_rels.contains("Id=\"rId2\" Type=\"t\" Target=\"/xl/worksheets/sheet3.xml\""));

        let sheet_rels = &content["xl/worksheets/_rels/sheet1.xml.rels"];
        assert!(sheet_rels.contains("Target=\"../drawings/drawing1.xml#frag\""));
        assert!(sheet_rels
            .contains("Target=\"https://example.com/sheet7.xml\" TargetMode=\"External\""));

        let types = &content[CONTENT_TYPES_PART];
        for part in ["sheet1", "sheet2", "sheet3"] {
            assert!(types.contains(&format!("/xl/worksheets/{part}.xml")));
        }
        assert!(types.contains("/xl/drawings/drawing1.xml"));
        assert!(!types.contains("sheet7") && !types.contains("drawing5"));
    }

    #[test]
    fn save_is_deterministic_and_replaces_existing_target() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("in.xlsx");
        write_zip(&source, &sample_parts());
        let output = dir.path().join("out.xlsx");
        std::fs::write(&output, b"old").unwrap();

        saved(&source, &output, true);
        let first = std::fs::read(&output).unwrap();
        saved(&source, &output, true);
        assert_eq!(first, std::fs::read(&output).unwrap());
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
    }

    #[test]
    fn saving_over_the_source_file_works() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("in.xlsx");
        write_zip(&source, &sample_parts());
        let (names, _) = saved(&source, &source, true);
        assert_eq!(names.len(), sample_parts().len());
    }

    #[test]
    fn unsafe_part_names_are_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("bad.xlsx");
        let mut parts = sample_parts();
        parts.push(("../evil.xml".into(), "x".into()));
        write_zip(&source, &parts);
        assert!(OfficeDocument::new(Some(source.to_string_lossy().into())).is_err());
    }
}
