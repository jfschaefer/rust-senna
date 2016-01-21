//! The tokenization functionality

use senna::Senna;
use util::ParseOption;
use util::{senna_parse, get_last_number_of_words};   // helper functionality


pub trait Tokenizer {
    fn get_number_of_words(&mut self, sentence: &str) -> u32;
}


impl Tokenizer for Senna {
    fn get_number_of_words(&mut self, sentence: &str) -> u32 {
        senna_parse(self, sentence, ParseOption::TokenizeOnly);
        get_last_number_of_words(self)
    }
}
