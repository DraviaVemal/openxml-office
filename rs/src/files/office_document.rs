use crate::{
    file_handling::{compress_content, decompress_content},
    files::{XmlSerializer, XmlDocument, XmlDeSerializer},
    global_2007::parts::ContentTypesPart,
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::{metadata, remove_file, File},
    io::{Cursor, Read, Write},
    rc::{Rc, Weak},
};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

#[derive(Debug)]
pub(crate) struct OfficeDocument {
    /// Key : File_Name -> Value : (File Handle, Content Type, File Extension, Extension Type)
    xml_document_collection:
        HashMap<String, (Rc<RefCell<XmlDocument>>, Option<String>, String, String)>,
    /// Key : File_name -> Value : (File_Extension,Extension_Type,Content_type,Compressed_Size,Uncompressed_Size,Compression_Level,File_Content)
    archive_collection: HashMap<
        String,
        (
            String,
            String,
            Option<String>,
            usize,
            usize,
            usize,
            Option<Vec<u8>>,
        ),
    >,
}

impl OfficeDocument {
    /// Create or Clone existing document to start with
    pub(crate) fn new(file_path: Option<String>) -> AnyResult<Self, AnyError> {
        let mut archive_collection = HashMap::new();
        if let Some(file_path) = file_path {
            // Load existing file to our system
            archive_collection = Self::load_archive_into_database(&file_path)
                .context("Load OpenXML Archive Into Database Failed")?;
        }
        Ok(Self {
            xml_document_collection: HashMap::new(),
            archive_collection,
        })
    }

    pub(crate) fn check_file_exist(&self, file_path: String) -> bool {
        self.archive_collection.contains_key(&file_path)
    }

    pub(crate) fn delete_document_mut(&mut self, file_name: &str) {
        self.xml_document_collection.remove(file_name);
        self.archive_collection.remove(file_name);
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
            (
                ref_xml_document.clone(),
                content_type.clone(),
                file_extension.clone(),
                extension_type.clone(),
            ),
        );
        Ok(weak_xml_document)
    }

    /// Get Xml tree from content if not already open
    pub(crate) fn get_xml_tree_mut(
        &mut self,
        file_path: &str,
    ) -> AnyResult<Option<(XmlDocument, Option<String>, String, String)>, AnyError> {
        // Check is the file path object already exist
        if self.xml_document_collection.get(file_path).is_some() {
            return Err(anyhow!(
                "Please close the Existing object before creating new handle"
            ));
        }
        if let Some((file_extension, extension_type, content_type, _, _, _, file_content)) =
            self.archive_collection.remove(file_path)
        {
            let content = file_content.ok_or(anyhow!("Failed To Get content vec"))?;
            let decompressed_data =
                decompress_content(&content).context("Raw Content Decompression Failed")?;
            let xml_tree: XmlDocument =
                XmlDeSerializer::vec_to_xml_doc_tree(decompressed_data, file_path)
                    .context("Xml Serializer Failed")?;
            Ok(Some((
                xml_tree,
                content_type.clone(),
                file_extension.clone(),
                extension_type.clone(),
            )))
        } else {
            Ok(None)
        }
    }

    /// Update the XML tree data to database and close the refCell
    pub(crate) fn close_xml_document(&mut self, file_path: &str) -> AnyResult<(), AnyError> {
        if let Some((xml_document, content_type, file_extension, extension_type)) =
            self.xml_document_collection.remove(file_path)
        {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("Failed to get document handle")?;
            let uncompressed_data = XmlSerializer::xml_tree_to_vec(&mut xml_doc_mut, file_path)
                .context(format!(
                "Failed Xml Tree to String content, File : {}",
                file_path
            ))?;
            let compression_level = 4;
            let compressed = compress_content(&uncompressed_data, compression_level)
                .context("Recompressing in GZip Failed")?;
            self.archive_collection
                .entry(file_path.to_string())
                .and_modify(|value| {
                    value.0 = file_extension.clone();
                    value.1 = extension_type.clone();
                    value.2 = content_type.clone();
                    value.3 = compressed.len();
                    value.4 = uncompressed_data.len();
                    value.5 = compression_level;
                    value.6 = Some(compressed.clone());
                })
                .or_insert((
                    file_extension,
                    extension_type,
                    content_type,
                    compressed.len(),
                    uncompressed_data.len(),
                    compression_level,
                    Some(compressed),
                ));
        }
        Ok(())
    }

    /// Save Current Document to final result
    pub(crate) fn save_as(&mut self, file_path: &str) -> AnyResult<(), AnyError> {
        // Save the live content update object to database
        let keys = self
            .xml_document_collection
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        for key_file_path in keys {
            self.close_xml_document(&key_file_path)
                .context(" Saving open object content failed")?;
        }
        let file_content: Vec<u8> = self
            .save_database_into_archive()
            .context("Save Archive Data into Database")?;
        if metadata(file_path).is_ok() {
            remove_file(file_path).map_err(|e| anyhow!("Remove Save File Target Failed. {}", e))?;
        }
        let mut file = File::create(file_path).context("Create Save File Failed")?;
        file.write_all(&file_content)
            .context("Save File Write Failed")
    }

    /// Save the database content into file archive
    fn save_database_into_archive(&self) -> AnyResult<Vec<u8>, AnyError> {
        let mut extensions: Vec<(String, String)> = Vec::new();
        let mut overrides: Vec<(String, String)> = Vec::new();
        let mut buffer = Cursor::new(Vec::new());
        let mut zip_writer: ZipWriter<&mut Cursor<Vec<u8>>> = ZipWriter::new(&mut buffer);
        let zip_option = SimpleFileOptions::default().compression_level(Some(4));
        // Load Files into Archive and add Override for content types
        {
            for (
                file_name,
                (
                    file_extension,
                    extension_type,
                    content_type,
                    _compress_size,
                    _uncompress_size,
                    _compression_level,
                    file_content,
                ),
            ) in self.archive_collection.to_owned()
            {
                extensions.push((file_extension, extension_type));
                if let Some(content_type) = content_type {
                    overrides.push((format!("/{}", file_name), content_type));
                }
                zip_writer
                    .start_file(file_name, zip_option)
                    .context("Zip File Write Start Fail")?;
                if let Some(xml_content_compressed) = file_content {
                    let uncompressed =
                        decompress_content(&xml_content_compressed).context("Decompress Error")?;
                    zip_writer
                        .write_all(&uncompressed)
                        .context("Writing compressed data to ZIp")?;
                }
            }
        }
        // Insert Content Type Details into Archive
        zip_writer
            .start_file("[Content_Types].xml", zip_option)
            .context("Zip File Write Start Fail")?;
        let content_type_file = ContentTypesPart::create_xml_file(
            extensions
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            overrides,
        )
        .context("Creating Content Type XML Failed")?;
        zip_writer
            .write_all(&content_type_file)
            .context("Writing compressed data to ZIp")?;
        zip_writer.finish().context("Zip Close Failed")?;
        Ok(buffer.into_inner())
    }

    /// Read Zip file and load it into database after compression
    fn load_archive_into_database(
        file_path: &str,
    ) -> AnyResult<
        HashMap<
            String,
            (
                String,
                String,
                Option<String>,
                usize,
                usize,
                usize,
                Option<Vec<u8>>,
            ),
        >,
        AnyError,
    > {
        let file: File = File::open(file_path).context("Open Existing archive File")?;
        let mut archive_collection = HashMap::new();
        let mut zip_read: ZipArchive<File> =
            ZipArchive::new(file).context("Archive read Failed")?;
        let mut uncompressed_file = Vec::new();
        zip_read
            .by_name("[Content_Types].xml")
            .context("Read [Content_Types].xml Failed")?
            .read_to_end(&mut uncompressed_file)
            .context("Failed to uncompress [Content_Types].xml")?;
        let file_names = zip_read
            .file_names()
            .filter(|name| "[Content_Types].xml" != *name)
            .map(|item| item.to_string())
            .collect::<Vec<_>>();
        let mut content_types_part =
            ContentTypesPart::new(uncompressed_file).context("Decoding Content Type Failed")?;
        let extension_collection = content_types_part
            .get_extensions()
            .context("Failed to pull extensions list")?;
        // Load File Content To DB
        for file_name in file_names {
            let mut file_extension = String::new();
            let mut extension_type = String::new();
            let mut uncompressed_data = Vec::new();
            let f_extension = file_name.rsplit('.').next();
            if let Some(f_extension) = f_extension {
                if let Some(extensions) = extension_collection.clone() {
                    let item = extensions.iter().find(|item| item.0 == f_extension);
                    if let Some(extension) = item {
                        file_extension = extension.0.to_string();
                        extension_type = extension.1.to_string();
                    }
                }
            }
            zip_read
                .by_name(&file_name)
                .context("Zip file extract failed")?
                .read_to_end(&mut uncompressed_data)
                .context("File Uncompressed failed")?;
            let content_type = content_types_part
                .get_override_content_type(&file_name)
                .context("Failed to extract Content Type")?;
            let compression_level = 4;
            let compressed = compress_content(&uncompressed_data, compression_level)
                .context("Recompressing in GZip Failed")?;
            archive_collection.insert(
                file_name,
                (
                    file_extension,
                    extension_type,
                    content_type,
                    compressed.len(),
                    uncompressed_data.len(),
                    compression_level,
                    Some(compressed),
                ),
            );
        }
        Ok(archive_collection)
    }
}
