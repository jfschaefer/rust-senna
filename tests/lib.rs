//! A few random tests.
//! Knowing myself, the coverage will be insignificant ;)

extern crate senna;

use senna::senna::*;
use senna::sentence::PSGNode;
use senna::sennapath::*;
use senna::pos::POS;
use senna::phrase::Phrase;

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
/// test very long sentence
fn test_very_long() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let sentence = senna.parse("Let mathformula be a finitely generated group of subexponential growth (e.g., mathformula); let mathformula be a finite set of generators for mathformula (e.g., the von Neumann or Moore neighborhood) and let mathformula be the set of reduced words on mathformula having at most length mathformula; let mathformula, and let mathformula be the quotient of mathformula with respect to the equivalence relation mathformula endowed with the topology induced by the distance mathformula Let mathformula be a cellular automaton over mathformula having set of states mathformula.  1.  mathformula induces in a natural way a Lipschitz continuous mathformula.  2.  mathformula is surjective if and only if mathformula is surjective.  3.  mathformula is injective if and only if it is invertible.", SennaParseOptions { pos: true, psg: true, });
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

