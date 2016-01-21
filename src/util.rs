//! Some helper functioniality

use std::ffi::CString;

use senna::Senna;
use c_signatures::{sennaParseSentence, sennaGetNumberOfWords};

/// Specifies which information shall be generated when parsing a sentence
pub enum ParseOption {
    TokenizeOnly,
    GeneratePOS,
    GeneratePSG,      // includes POS generation
}

impl ParseOption {
    /// Convert to the corresponding values in `c_wrapper.h`
    fn convert(&self) -> u32 {
        match *self {
            ParseOption::TokenizeOnly => 0,
            ParseOption::GeneratePOS  => 1,
            ParseOption::GeneratePSG  => 2,
        }
    }
}

/// Parses one sentence
pub fn senna_parse(senna: &mut Senna, sentence: &str, option: ParseOption) {
    let c_string = CString::new(sentence).unwrap().as_ptr();
    unsafe {
        sennaParseSentence(senna.senna_ptr, c_string, option.convert());
    }
}

pub fn get_last_number_of_words(senna: &Senna) -> u32{
    unsafe {
        sennaGetNumberOfWords(senna.senna_ptr)
    }
}

