//! # A wrapper for senna
//! **Please ensure that you agree to senna's [license](http://ronan.collobert.com/senna/download.html)**!
//! This library is a simple rust wrapper for the POS and PSG functionality of senna.


extern crate libc;

mod c_signatures;
mod util;
pub mod senna;
pub mod tokenizer;

