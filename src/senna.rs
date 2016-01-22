//! The initialization functionality

use c_signatures::{sennaCreate, sennaFree};

use libc::c_void;
use std::ffi::CString;

use util::{senna_parse, get_last_number_of_words};   // helper functionality
use sentence::Sentence;



/// Specifies which information shall be generated when parsing a sentence
pub enum ParseOption {
    TokenizeOnly,
    GeneratePOS,
    GeneratePSG,      // includes POS generation
}

impl ParseOption {
    /// Convert to the corresponding values in `c_wrapper.h`
    pub fn convert(&self) -> u32 {
        match *self {
            ParseOption::TokenizeOnly => 0,
            ParseOption::GeneratePOS  => 1,
            ParseOption::GeneratePSG  => 2,
        }
    }
}





/// Handle to all the hashes etc.
pub struct Senna {
    /// pointer to the corresponding c struct
    pub senna_ptr : *mut c_void,
}

impl <'a> Senna {
    /// Initializes senna based on the data files in `opt_path`
    pub fn new(opt_path : &str) -> Senna {
        let c_path = CString::new(opt_path).unwrap().as_ptr();
        unsafe {
            Senna {
                senna_ptr : sennaCreate(c_path),
            }
        }
    }

    /// Returns the number of words contained in a string
    /// (warning: Does full tokenization, i.e. relatively costly)
    pub fn get_number_of_words(&mut self, sentence: &str) -> u32 {
        senna_parse(self, sentence, ParseOption::TokenizeOnly);
        get_last_number_of_words(self)
    }


    pub fn parse(&mut self, sentence: &'a str, options: ParseOption) -> Sentence<'a> {
        senna_parse(self, sentence, options);

        Sentence::new(sentence, vec![])
    }
}


impl Drop for Senna {
    fn drop(&mut self) {
        unsafe { sennaFree(self.senna_ptr); }
    }
}


