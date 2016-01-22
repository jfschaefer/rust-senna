//! Some helper functions

use std::ffi::CString;

use senna::{Senna, ParseOption};
use c_signatures::*;

/// Parses one sentence
pub fn senna_parse(senna: &mut Senna, sentence: &str, option: ParseOption) {
    let c_string = CString::new(sentence).unwrap().as_ptr();
    unsafe {
        sennaParseSentence(senna.senna_ptr, c_string, option.convert());
    }
}

