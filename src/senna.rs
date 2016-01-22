//! The initialization functionality

use c_signatures::*;

use libc::c_void;
use std::ffi::CString;

use util::*;   // helper functionality
use sentence::{Sentence, Word};



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
        unsafe { sennaGetNumberOfWords(self.senna_ptr) }
    }


    pub fn parse(&mut self, sentence: &'a str, options: ParseOption) -> Sentence<'a> {
        senna_parse(self, sentence, options);
        let n = unsafe { sennaGetNumberOfWords(self.senna_ptr) };
        let mut words: Vec<Word> = Vec::with_capacity(n as usize);

        for i in 0..n {
            let start = unsafe { sennaGetStartOffset(self.senna_ptr, i) } as usize;
            let end = unsafe { sennaGetEndOffset(self.senna_ptr, i) } as usize;
            words.push(Word::new(start, end, &sentence[start..end]));
        }

        Sentence::new(sentence, words)
    }
}


impl Drop for Senna {
    fn drop(&mut self) {
        unsafe { sennaFree(self.senna_ptr); }
    }
}


