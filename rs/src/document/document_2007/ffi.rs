use std::ffi::c_char;

use crate::{
    document::document_2007::{Word, WordPropertiesModel},
    ffi::{
        openxml_office_fbs::document::{
            Word_create, Word_create_return, Word_create_returnArgs, Word_save_as,
            Word_save_as_return, Word_save_as_returnArgs,
        },
        root_from_raw, set_error, write_buffer, StatusCode,
    },
};

#[no_mangle]
/// Create a Word object.
pub extern "C" fn Word_create(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_word_create =
        match unsafe { root_from_raw::<Word_create>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let file_name = fbs_word_create.file_name().map(|item| item.to_string());
    let word_settings = fbs_word_create.word_settings();
    let word_properties = WordPropertiesModel {
        is_editable: word_settings.is_editable(),
    };
    match Word::new(file_name, word_properties) {
        Ok(word) => {
            let word_ptr = Box::into_raw(Box::new(word)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let word_create_return = Word_create_return::create(
                &mut builder,
                &Word_create_returnArgs { word_ptr: word_ptr },
            );
            builder.finish(word_create_return, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::UnknownError) },
    }
}

#[no_mangle]
/// Save and consume the Word object.
pub extern "C" fn Word_save_as(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_save_as =
        match unsafe { root_from_raw::<Word_save_as>(in_buffer, in_buffer_size, out_error) } {
            Ok(root) => root,
            Err(status) => return status,
        };
    let word_ptr = fbs_save_as.word_ptr() as *mut Word;
    let word = unsafe { *Box::from_raw(word_ptr) };
    match word.save_as(fbs_save_as.file_name().unwrap_or_default()) {
        Ok(full_path) => {
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let full_path_offset = builder.create_string(&full_path);
            let word_save_as_return = Word_save_as_return::create(
                &mut builder,
                &Word_save_as_returnArgs {
                    full_path: Some(full_path_offset),
                },
            );
            builder.finish(word_save_as_return, None);
            unsafe { write_buffer(builder, out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(error) => unsafe { set_error(out_error, &error, StatusCode::IoError) },
    }
}
