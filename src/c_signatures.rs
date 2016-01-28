//! The signatures of the `c` functions we will need

use libc::{c_char, c_void, c_uint};


extern "C" {
    pub fn sennaCreate(opt_path: *const c_char) -> *mut c_void;
    pub fn sennaFree(senna: *mut c_void);

    pub fn sennaParseSentence(senna: *mut c_void, sentence: *const c_char, options: c_uint);

    pub fn sennaGetNumberOfWords(senna: *const c_void) -> c_uint;
    pub fn sennaGetStartOffset(senna: *const c_void, token: c_uint) -> c_uint;
    pub fn sennaGetEndOffset(senna: *const c_void, token: c_uint) -> c_uint;
    pub fn sennaGetPOS(senna: *const c_void, token: c_uint) -> *const c_char;
    pub fn sennaGetPSGStr(senna: *mut c_void) -> *const c_char;
}

