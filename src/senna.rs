//! The initialization functionality

use c_signatures::{sennaCreate, sennaFree};

use libc::c_void;
use std::ffi::CString;

/// Handle to all the hashes etc.
pub struct Senna {
    /// pointer to the corresponding c struct
    pub senna_ptr : *mut c_void,
}

impl Senna {
    /// Initializes senna based on the data files in `opt_path`
    pub fn new(opt_path : &str) -> Senna {
        let c_path = CString::new(opt_path).unwrap().as_ptr();
        unsafe {
            Senna {
                senna_ptr : sennaCreate(c_path),
            }
        }
    }
}


impl Drop for Senna {
    fn drop(&mut self) {
        unsafe { sennaFree(self.senna_ptr); }
    }
}


