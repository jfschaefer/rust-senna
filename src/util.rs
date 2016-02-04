//! Some helper functions

use std::ffi::{CString, CStr};
use std::str;
use std::collections::HashMap;

use senna::{Senna, SennaParseOptions};
use sentence::*;
use c_signatures::*;
use phrase::Phrase;

/// Parses one sentence
pub fn senna_parse(senna: &mut Senna, sentence: &str, option: SennaParseOptions) {
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

/// recursively parses a PSG string and returns the root node
pub fn parse_psg(psgstr: &[u8], word_count: &mut usize, index: &mut usize,
                 psg_map: &HashMap<&str, Phrase>) -> PSGNode {
    if *index >= psgstr.len() {
        panic!("rust-senna: Fatal: PSG string ended unexpectedly: \"{}\"",
                String::from_utf8(psgstr.to_vec()).unwrap());
    }
    match psgstr[*index] {
        b'*' => {
            let node = PSGNode::Leaf(*word_count);
            *word_count = *word_count + 1;
            *index = *index + 1;
            return node;
        },
        b'(' => {
            *index = *index + 1;
            let mut offset = 0usize;
            loop {
                let c = psgstr[*index + offset];
                if c < b'A' || c > b'Z' { break; }  // end of phrase label
                offset += 1;
            }
            let label = str::from_utf8(&psgstr[*index..(*index + offset)]).unwrap();
            *index = *index + offset;
            let mut phrase = Box::new(PSGPhrase::new(*psg_map.get(&label).unwrap()));
            while psgstr[*index] != b')' {
                (*phrase).push_child(parse_psg(psgstr, word_count, index, psg_map));
            }
            *index = *index + 1;
            return PSGNode::Parent(phrase);
        },
        _ => panic!("rust-senna: Fatal: unexpected character in PSG string: \"{}\"",
                    String::from_utf8(psgstr.to_vec()).unwrap()),
    }
}

