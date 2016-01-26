//! Some helper functions

use std::ffi::{CString, CStr};
use std::str;

use senna::{Senna, ParseOption};
use c_signatures::*;

/// Parses one sentence
pub fn senna_parse(senna: &mut Senna, sentence: &str, option: ParseOption) {
    let c_string = CString::new(sentence).unwrap().as_ptr();
    unsafe {
        sennaParseSentence(senna.senna_ptr, c_string, option.convert());
    }
}

/// Converts C's char * to &str
pub fn const_cptr_to_rust<'t>(cptr: *const i8) -> &'t str {
    let cstr = unsafe { CStr::from_ptr(cptr) };
    str::from_utf8(cstr.to_bytes()).unwrap()
}

