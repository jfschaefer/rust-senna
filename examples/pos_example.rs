//! A simple example how you can use `rustsenna` for
//! part-of-speech tagging of sentences.

extern crate senna;

use senna::sennapath::SENNA_PATH;
use senna::senna::{Senna, SennaParseOptions};

fn main() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let sentence = senna.parse("This is not a sentence.", SennaParseOptions { pos: true, psg: false, });
    for word in sentence.get_words() {
        print!("\"{}\" - {}\n", word.get_string(), word.get_pos().to_str());
    }
}

