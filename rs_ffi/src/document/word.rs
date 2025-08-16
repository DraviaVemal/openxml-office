use crate::{chain_error, openxml_office_fbs, StatusCode};
use draviavemal_openxml_office::document_2007::{Word, WordPropertiesModel};
use std::{
    ffi::{c_char, c_void, CStr, CString},
    slice::from_raw_parts,
};

#[no_mangle]
/// Creates a new Word object.
///
/// Returns a pointer to the newly created Word object.
/// If an error occurs, returns a null pointer.
pub extern "C" fn word_create(
    file_name: *const c_char,
    buffer: *const u8,
    buffer_size: usize,
    out_word: *mut *mut c_void,
    out_error: *mut *const c_char,
) -> i8 {
    let file_name = if file_name.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(file_name) }
                .to_string_lossy()
                .into_owned(),
        )
    };
    if buffer.is_null() || buffer_size == 0 {
        return StatusCode::InvalidArgument as i8;
    }
    let buffer_slice = unsafe { from_raw_parts(buffer, buffer_size) };
    match flatbuffers::root::<openxml_office_fbs::presentation_2007::PresentationPropertiesModel>(
        buffer_slice,
    ) {
        Ok(fbs_word_properties) => {
            let word_properties = WordPropertiesModel {
                is_editable: fbs_word_properties.is_editable(),
            };
            let word = if let Some(file_name) = file_name {
                Word::new(Some(file_name), word_properties)
            } else {
                Word::new(None, word_properties)
            };
            match word {
                Ok(word) => {
                    unsafe {
                        *out_word = Box::into_raw(Box::new(word)) as *mut c_void;
                    }
                    StatusCode::Success as i8
                }
                Err(e) => {
                    unsafe { *out_error = chain_error(&e) };
                    StatusCode::UnknownError as i8
                }
            }
        }
        Err(e) => {
            unsafe { *out_error = chain_error(&e.into()) };
            StatusCode::FlatBufferError as i8
        }
    }
}

#[no_mangle]
///Save the Word File in provided file path
pub extern "C" fn word_save_as(
    word_ptr: *const c_void,
    file_name: *const c_char,
    out_error: *mut *const c_char,
) -> i8 {
    if word_ptr.is_null() || file_name.is_null() {
        eprintln!("Received null pointer");
        return StatusCode::InvalidArgument as i8;
    }
    let file_name = unsafe { CStr::from_ptr(file_name) }
        .to_string_lossy()
        .into_owned();
    let word_ptr = word_ptr as *mut Word;
    let word = unsafe { Box::from_raw(word_ptr) };
    match word.save_as(&file_name) {
        Result::Ok(_full_path) => StatusCode::Success as i8,
        Err(e) => match CString::new(format!("Flat Buffer Parse Error. {}", e)) {
            Result::Ok(str) => {
                unsafe { *out_error = str.into_raw() };
                StatusCode::Success as i8
            }
            Err(e) => {
                eprintln!("Error String send Error. {}", e);
                StatusCode::IoError as i8
            }
        },
    }
}
