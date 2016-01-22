//! A few random tests.
//! Knowing myself, the coverage will be insignificant ;)

extern crate rustsenna;

use rustsenna::senna::Senna;

#[test]
/// Tokenization returns correct number of words
fn test_token_count() {
    let mut senna = Senna::new("senna/");
    assert_eq!(2, senna.get_number_of_words("Hello world"));
}

