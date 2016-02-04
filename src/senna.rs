//! The initialization functionality

use libc::c_void;
use std::ffi::CString;
use std::collections::HashMap;
use std::io::{self, Write};

use c_signatures::*;
use util::*;   // helper functionality
use sentence::{Sentence, Word};
use pos::POS;
use phrase::Phrase;


#[derive(PartialEq, Clone, Copy)]
pub struct SennaParseOptions {
    pub pos: bool,
    pub psg: bool,
}

impl Default for SennaParseOptions {
    fn default() -> SennaParseOptions {
        SennaParseOptions {
            pos: true,
            psg: false,   // expensive
        }
    }
}

impl SennaParseOptions {
    pub fn convert(&self) -> u32 {
        0 + if self.pos { 1<<0 } else { 0 } + if self.psg { 1<<1 } else { 0 }
    }
}


// /// Specifies which information shall be generated when parsing a sentence
// #[derive(PartialEq, Clone, Copy)]
// pub enum ParseOption {
//     Tokenize,
//     GeneratePOS,
//     GeneratePSG,      // includes POS generation
// }
// 
// impl ParseOption {
//     /// Convert to the corresponding values in `c_wrapper.h`
//     pub fn convert(&self) -> u32 {
//         match *self {
//             ParseOption::Tokenize     => 0,
//             ParseOption::GeneratePOS  => 1<<0,
//             ParseOption::GeneratePSG  => 1<<1,
//         }
//     }
// }





/// Handle to all the hashes etc.
pub struct Senna {
    /// pointer to the corresponding C struct
    pub senna_ptr : *mut c_void,
    /// HashMap for converting string pos tags to `pos::POS`
    /// (unfortunately, there seem to be no static HashMaps in Rust yet)
    pub pos_map : HashMap<&'static str, POS>,
    /// HashMap for converting string psg tags to `phrase::Phrase`
    /// (unfortunately, there seem to be no static HashMaps in Rust yet)
    pub psg_map : HashMap<&'static str, Phrase>,
}

impl <'a> Senna {
    /// Initializes senna based on the data files in `opt_path`
    pub fn new(opt_path : String) -> Senna {
        let c_path = CString::new(opt_path).unwrap().as_ptr();
        unsafe {
            Senna {
                senna_ptr : sennaCreate(c_path),
                pos_map : POS::generate_str_to_pos_map(),
                psg_map : Phrase::generate_str_to_phrase_map(),
            }
        }
    }

    /// Returns the number of words contained in a string
    /// (warning: Does full tokenization, i.e. relatively costly)
    pub fn get_number_of_words(&mut self, sentence: &str) -> u32 {
        senna_parse(self, sentence, SennaParseOptions {
            pos: false, psg: false, });
        unsafe { sennaGetNumberOfWords(self.senna_ptr) }
    }


    /// Parses a sentence and returns the results as a `Sentence` structure.
    pub fn parse(&mut self, sentence: &'a str, options: SennaParseOptions) -> Sentence<'a> {
        senna_parse(self, sentence, options);
        let n = unsafe { sennaGetNumberOfWords(self.senna_ptr) };
        let mut sen = Sentence::new(sentence);

        for i in 0..n {
            let start = unsafe { sennaGetStartOffset(self.senna_ptr, i) } as usize;
            let end = unsafe { sennaGetEndOffset(self.senna_ptr, i) } as usize;
            let mut word = Word::new(start, end, &sentence[start..end], i);
            if options.pos || options.psg {
                let pos = unsafe { sennaGetPOS(self.senna_ptr, i) };
                let pospos = match self.pos_map.get(const_cptr_to_rust(pos)) {
                    None => {
                        writeln!(&mut io::stderr(),
                            "Warning: rust-senna: senna: unknown POS tag: \"{}\"",
                            const_cptr_to_rust(pos)).unwrap();
                        POS::NOT_SET
                    },
                    Some(x) => *x,
                };

                word.set_pos(pospos);
            }
            sen.push_word(word);
        }

        if options.psg {
            let psgstr = const_cptr_to_rust( unsafe { sennaGetPSGStr(self.senna_ptr) } );
            let psgroot = parse_psg(psgstr.as_bytes(), &mut 0, &mut 0, &self.psg_map);
            sen.set_psgroot(psgroot);
        }
        sen
    }
}


impl Drop for Senna {
    /// Senna's hash tables etc. must be freed
    fn drop(&mut self) {
        unsafe { sennaFree(self.senna_ptr); }
    }
}


