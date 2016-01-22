//! Contains data structures for the representation of a sentence
//! which can be tokenized, POS tagged, etc.



pub struct Word<'t> {
    offset_start : usize,
    offset_end : usize,
    string: &'t str,
}


pub struct Sentence<'t> {
    string: &'t str,
    words: Vec<Word<'t>>,
}

impl<'t> Sentence<'t> {
    pub fn new(string: &'t str, words: Vec<Word<'t>>) -> Sentence<'t> {
        Sentence {
            string: string,
            words: words,
        }
    }
    pub fn get_words(&self) -> &Vec<Word> {
        &self.words
    }
}

