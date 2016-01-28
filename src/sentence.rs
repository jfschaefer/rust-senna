//! Contains data structures for the representation of a sentence
//! which can be tokenized, POS tagged, etc.

/// A tokenized and possibly annotated word
pub struct Word<'t> {
    offset_start : usize,
    offset_end : usize,
    string: &'t str,
    pos: String,
    n: u32,    // nth word in sentence
}

impl<'t> Word<'t> {
    pub fn new(offset_start: usize, offset_end: usize, string: &'t str, n: u32) -> Word<'t> {
        Word {
            offset_start: offset_start,
            offset_end: offset_end,
            string: string,
            pos: String::new(),
            n: n,
        }
    }

    pub fn set_pos(&mut self, tag: &str) {
        self.pos = tag.to_string();
    }

    pub fn get_pos(&self) -> &str {
        &self.pos
    }

    pub fn get_string(&self) -> &str {
        self.string
    }

    pub fn get_offset_start(&self) -> usize {
        self.offset_start
    }

    pub fn get_offset_end(&self) -> usize {
        self.offset_end
    }

    pub fn get_n(&self) -> u32 {
        self.n
    }
}


/// A tokenized sentence
pub struct Sentence<'t> {
    string: &'t str,
    words: Vec<Word<'t>>,
    psgroot: Option<PSGNode>,
}

impl<'t> Sentence<'t> {
    pub fn new(string: &'t str) -> Sentence<'t> {
        Sentence {
            string: string,
            words: vec![],
            psgroot: None,
        }
    }

    pub fn push_word(&mut self, word: Word<'t>) {
        self.words.push(word);
    }

    pub fn set_psgroot(&mut self, root: PSGNode) {
        self.psgroot = Some(root);
    }

    pub fn get_psgroot(&'t self) -> Option<&PSGNode> {
        self.psgroot.as_ref()
    }

    pub fn get_words(&self) -> &Vec<Word> {
        &self.words
    }

    pub fn get_string(&self) -> &str {
        self.string
    }
}

pub struct PSGPhrase {
    label: String,
    children: Vec<PSGNode>,
}

impl PSGPhrase {
    pub fn new(label: &str) -> PSGPhrase {
        PSGPhrase {
            label: label.to_string(),
            children: vec![],
        }
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn push_child(&mut self, child: PSGNode) {
        self.children.push(child);
    }

    pub fn get_children(&self) -> &Vec<PSGNode> {
        &self.children
    }
}

        

pub enum PSGNode {
    Leaf(usize /* <- index of word */),
    Parent(Box<PSGPhrase>),
}


