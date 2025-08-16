use anyhow::{Error as AnyError, Result as AnyResult};
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::{
    io::{Read, Write},
    path::Path,
};

pub(crate) fn compress_content(
    uncompressed_data: &[u8],
    compression_level: usize,
) -> AnyResult<Vec<u8>, AnyError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(compression_level as u32));
    encoder.write_all(uncompressed_data)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub(crate) fn decompress_content(compressed_data: &[u8]) -> AnyResult<Vec<u8>, AnyError> {
    let mut decoder: GzDecoder<&[u8]> = GzDecoder::new(compressed_data);
    let mut decompressed_data: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

pub(crate) fn clean_file_path(path: &str) -> String {
    let mut result_path = Vec::new();
    for part in Path::new(&path).components() {
        match part {
            std::path::Component::ParentDir => {
                result_path.pop();
            }
            std::path::Component::CurDir => {}
            _ => {
                result_path.push(part.as_os_str().to_string_lossy().to_string());
            }
        };
    }
    result_path.join("/")
}
