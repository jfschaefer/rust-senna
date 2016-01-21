//! The signatures of the `c` functions we will need

use libc::{c_char, c_void, c_uint};


extern "C" {
    pub fn sennaCreate(opt_path: *const c_char) -> *mut c_void;
    pub fn sennaFree(senna: *mut c_void);

    pub fn sennaParseSentence(senna: *mut c_void, sentence: *const c_char, options: c_uint);

    pub fn sennaGetNumberOfWords(senna: *const c_void) -> c_uint;
}
