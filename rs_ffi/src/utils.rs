use std::{
    ffi::{c_char, CString},
    slice::from_raw_parts,
};

use anyhow::{anyhow, Error as AnyError};
use flatbuffers::{Follow, Verifiable};

use crate::StatusCode;

pub(crate) fn chain_error(error: &AnyError) -> *mut c_char {
    let mut message = String::new();
    for (i, chain) in error.chain().enumerate() {
        if i > 0 {
            message.push_str(" -> ");
        }
        message.push_str(&chain.to_string());
    }
    match CString::new(message) {
        Result::Ok(str) => str.into_raw(),
        Err(e) => {
            eprintln!("Error String send Error. {}", e);
            std::ptr::null_mut()
        }
    }
}

/// Validates the incoming FFI buffer and parses it as the given FlatBuffers root type.
///
/// On success returns the parsed root. On failure sets `out_error` (for parse errors)
/// and returns the `StatusCode` (as `i8`) that the caller should return.
///
/// # Safety
/// `in_buffer` must point to `in_buffer_size` valid bytes for the duration of the call,
/// and `out_error` must be a valid pointer.
pub(crate) unsafe fn root_from_raw<'a, T>(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_error: *mut *const c_char,
) -> Result<T::Inner, i8>
where
    T: Follow<'a> + Verifiable + 'a,
{
    if in_buffer.is_null() || in_buffer_size == 0 {
        return Err(set_error(
            out_error,
            &AnyError::msg("draviavemal-openxml_office::Input buffer is null or empty"),
            StatusCode::InvalidArgument,
        ));
    }
    let buffer_slice = from_raw_parts(in_buffer, in_buffer_size);
    match flatbuffers::root::<T>(buffer_slice) {
        Ok(root) => Ok(root),
        Err(e) => Err(set_error(out_error, &e.into(), StatusCode::FlatBufferError)),
    }
}

/// Writes a serialized FlatBuffers payload to the FFI output pointers.
///
/// The payload is copied into a freshly allocated heap buffer whose ownership is
/// transferred to the caller. The caller must release it by calling `free_buffer`
/// with the returned pointer and size once the bytes have been consumed.
///
/// # Safety
/// `out_buffer` and `out_buffer_size` must be valid pointers.
pub(crate) unsafe fn write_buffer(
    buf: &[u8],
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
) {
    let boxed: Box<[u8]> = Box::from(buf);
    *out_buffer_size = boxed.len();
    *out_buffer = Box::into_raw(boxed) as *mut u8;
}

#[no_mangle]
/// Releases a buffer previously handed to the caller through an FFI output pointer.
///
/// # Safety
/// `buffer` must be a pointer returned by an FFI call together with its matching
/// `buffer_size`, and it must not have been freed already.
pub unsafe extern "C" fn free_buffer(buffer: *mut u8, buffer_size: usize) {
    if buffer.is_null() {
        return;
    }
    let slice = std::slice::from_raw_parts_mut(buffer, buffer_size);
    drop(Box::from_raw(slice as *mut [u8]));
}

/// Sets the FFI error output from an error chain and returns the given status code.
///
/// # Safety
/// `out_error` must be a valid pointer.
pub(crate) unsafe fn set_error(
    out_error: *mut *const c_char,
    error: &AnyError,
    status_code: StatusCode,
) -> i8 {
    *out_error = chain_error(error);
    status_code as i8
}
