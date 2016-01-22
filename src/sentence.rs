//! Contains data structures for the representation of a sentence
//! which can be tokenized, POS tagged, etc.



pub struct Word<'t> {
    offset_start : usize,
    offset_end : usize,
    string: &'t str,

}

impl<'t> Word<'t> {
    pub fn new(offset_start: usize, offset_end: usize, string: &'t str) -> Word<'t> {
        Word {
            offset_start: offset_start,
            offset_end: offset_end,
            string: string,
        }
    }

    pub fn get_string(&self) -> &str {
        self.string
    }
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

