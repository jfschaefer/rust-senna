//! Contains data structures for the representation of a sentence
//! which can be tokenized, POS tagged, etc.

/// A tokenized and possibly annotated word

use pos::POS;

pub struct Word<'t> {
    offset_start : usize,
    offset_end : usize,
    string: &'t str,
    pos: POS,
    n: u32,    // nth word in sentence
}

impl<'t> Word<'t> {
    /// Constructor for a new word. A POS tag can later be added using `set_pos`.
    pub fn new(offset_start: usize, offset_end: usize, string: &'t str, n: u32) -> Word<'t> {
        Word {
            offset_start: offset_start,
            offset_end: offset_end,
            string: string,
            pos: POS::NOT_SET,
            n: n,
        }
    }

    /// Sets the POS tag
    pub fn set_pos(&mut self, tag: POS) {
        self.pos = tag;
    }

    /// returns the POS tag (empty string if not set)
    pub fn get_pos(&self) -> POS {
        self.pos
    }

    /// Returns the string representation of the word
    pub fn get_string(&self) -> &str {
        self.string
    }

    /// Returns the offset of the first character of the word in the sentence
    pub fn get_offset_start(&self) -> usize {
        self.offset_start
    }

    /// Returns the offset of the end of the word in the sentence
    pub fn get_offset_end(&self) -> usize {
        self.offset_end
    }

    /// Returns the position of the word in the sentence (n-th word)
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
    /// Constructs a new sentence. Tokenized words can be added with `push_word`
    pub fn new(string: &'t str) -> Sentence<'t> {
        Sentence {
            string: string,
            words: vec![],
            psgroot: None,
        }
    }

    /// Adds a `Word` to the sentence
    pub fn push_word(&mut self, word: Word<'t>) {
        self.words.push(word);
    }

    /// Sets the root node for the PSG tree
    pub fn set_psgroot(&mut self, root: PSGNode) {
        self.psgroot = Some(root);
    }

    /// Returns the root of the PSG tree if set
    pub fn get_psgroot(&'t self) -> Option<&PSGNode> {
        self.psgroot.as_ref()
    }

    /// returns the tokenized words
    pub fn get_words(&self) -> &Vec<Word> {
        &self.words
    }

    /// returns the string representation of the sentence
    pub fn get_string(&self) -> &str {
        self.string
    }
}

/// A phrase node in the PSG tree
pub struct PSGPhrase {
    label: String,
    children: Vec<PSGNode>,
}

impl PSGPhrase {
    /// constructs a new PSG phrase with a label (e.g. `NP` or `VP`)
    pub fn new(label: &str) -> PSGPhrase {
        PSGPhrase {
            label: label.to_string(),
            children: vec![],
        }
    }

    /// returns the label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// adds another child node
    pub fn push_child(&mut self, child: PSGNode) {
        self.children.push(child);
    }

    /// returns all the children
    pub fn get_children(&self) -> &Vec<PSGNode> {
        &self.children
    }
}

        

/// A node in the PSG tree (which can be either a leaf, referring to a word, or a phrase)
pub enum PSGNode {
    Leaf(usize /* <- index of word */),
    Parent(Box<PSGPhrase>),
}


