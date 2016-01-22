//! A few random tests.
//! Knowing myself, the coverage will be insignificant ;)

extern crate rustsenna;

use rustsenna::senna::*;

#[test]
/// Tokenization returns correct number of words
fn test_token_count() {
    let mut senna = Senna::new("senna/");
    assert_eq!(2, senna.get_number_of_words("Hello world"));
}


#[test]
/// Test words in tokenization are okay
fn test_word_strings_in_tokenization() {
    let mut senna = Senna::new("senna/");
    let sentence = senna.parse("How are you", ParseOption::TokenizeOnly);
    assert_eq!("are", sentence.get_words()[1].get_string());
}
