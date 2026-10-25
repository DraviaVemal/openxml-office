use std::{
    ffi::{c_char, CString},
    slice::from_raw_parts,
};

use anyhow::Error as AnyError;
use flatbuffers::{FlatBufferBuilder, Follow};
use log::error;

use crate::ffi::StatusCode;

pub(crate) fn chain_error(error: &AnyError) -> *mut c_char {
    let mut message = String::new();
    for (index, chain) in error.chain().enumerate() {
        if index > 0 {
            message.push_str(" -> ");
        }
        message.push_str(&chain.to_string());
    }
    match CString::new(message) {
        Result::Ok(error_string) => error_string.into_raw(),
        Err(error) => {
            error!("Error String send Error. {}", error);
            std::ptr::null_mut()
        }
    }
}

/// Parse trusted FlatBuffers input without structural verification.
///
/// # Safety
/// Input and error pointers must be valid for this call.
pub(crate) unsafe fn root_from_raw<'a, T>(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> Result<T::Inner, i8>
where
    T: Follow<'a> + 'a,
{
    if in_buffer.is_null() || in_buffer_size == 0 {
        return Err(set_error(
            out_error,
            &AnyError::msg("draviavemal-openxml_office::Input buffer is null or empty"),
            StatusCode::InvalidArgument,
        ));
    }
    let buffer_slice = from_raw_parts(in_buffer, in_buffer_size);
    Ok(flatbuffers::root_unchecked::<T>(buffer_slice))
}

/// Transfer a finished FlatBuffer allocation to the caller.
///
/// # Safety
/// Output pointers must be valid.
pub(crate) unsafe fn write_buffer(
    builder: FlatBufferBuilder<'_>,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
) {
    let (mut buffer, start) = builder.collapse();
    let buffer_size = buffer.len() - start;
    buffer.copy_within(start.., 0);
    buffer.truncate(buffer_size);
    let boxed: Box<[u8]> = buffer.into_boxed_slice();
    *out_buffer_size = boxed.len();
    *out_buffer = Box::into_raw(boxed) as *mut u8;
}

#[no_mangle]
/// Free a buffer returned by this crate.
///
/// # Safety
/// The pointer and size must match a live FFI result.
pub unsafe extern "C" fn free_buffer(buffer: *mut u8, buffer_size: usize) {
    if buffer.is_null() {
        return;
    }
    let slice = std::slice::from_raw_parts_mut(buffer, buffer_size);
    drop(Box::from_raw(slice as *mut [u8]));
}

/// Set the caller's error output and return a status code.
///
/// # Safety
/// `out_error` must be valid.
pub(crate) unsafe fn set_error(
    out_error: *mut *const c_char,
    error: &AnyError,
    status_code: StatusCode,
) -> i8 {
    *out_error = chain_error(error);
    status_code as i8
}
