use std::ffi::{c_char, CString};

use anyhow::Error as AnyError;

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
