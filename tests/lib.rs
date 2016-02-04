//! A few random tests.
//! Knowing myself, the coverage will be insignificant ;)

extern crate rustsenna;

use rustsenna::senna::*;
use rustsenna::sentence::PSGNode;
use rustsenna::sennapath::*;
use rustsenna::pos::POS;
use rustsenna::phrase::Phrase;

#[test]
/// Tokenization returns correct number of words
fn test_token_count() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    assert_eq!(2, senna.get_number_of_words("Hello world"));
}


#[test]
/// Test words in tokenization are okay
fn test_word_strings_in_tokenization() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let sentence = senna.parse("How are you", SennaParseOptions { pos: false, psg: false, });
    assert_eq!("are", sentence.get_words()[1].get_string());
}


#[test]
/// test pos tags
fn test_pos_tagging() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let sentence = senna.parse("This is not a sentence", SennaParseOptions { pos: true, psg: false, });
    let a = &sentence.get_words()[3];
    assert_eq!("a", a.get_string());
    assert_eq!(POS::DT, a.get_pos());
}

#[test]
/// test psg tags
fn test_psg_tagging() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let sentence = senna.parse("it works", SennaParseOptions { pos: true, psg: true, });
    // remark: psg should be (S(NP*)(VP*))
    let root = sentence.get_psgroot().unwrap();
    match root {
        &PSGNode::Leaf(_) => { panic!("wrong psg node A") }
        &PSGNode::Parent(ref s) => {
            assert_eq!(s.get_label(), Phrase::S);
            let children = s.get_children();
            assert_eq!(children.len(), 2);
            match children[0] {
                PSGNode::Leaf(_) => { panic!("wrong psg node B") }
                PSGNode::Parent(ref t) => { assert_eq!(t.get_label(), Phrase::NP) }
            }
            match children[1] {
                PSGNode::Leaf(_) => { panic!("wrong psg node C") }
                PSGNode::Parent(ref t) => {
                    assert_eq!(t.get_label(), Phrase::VP);
                    assert_eq!(t.get_children().len(), 1);
                    match t.get_children()[0] {
                        PSGNode::Leaf(1) => { } /* Correct :-) */,
                        PSGNode::Leaf(_) => { panic!("wrong psg node D") },
                        _ => { panic!("wrong psg node D") }
                    }
                }
            }
        }
    }
}

