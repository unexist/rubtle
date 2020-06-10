///
/// @package Rubtle-Lib
///
/// @file Rubtle functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use cesu8::{to_cesu8, from_cesu8};

use std::{process, ptr, slice};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0")
            as *const str
            as *const [::std::os::raw::c_char]
            as *const ::std::os::raw::c_char
    )
}

pub struct Rubtle {
    /// Duktape context
    pub(crate) ctx: *mut ffi::duk_context,
}

impl Rubtle {

    ///
    /// Create a new Rubtle instance
    ///

    pub fn new() -> Rubtle {
        Rubtle {
            ctx: unsafe { Self::create_heap() },
        }
    }

    ///
    /// Push string onto duktape stack
    ///
    /// * `str_val` - String value to push
    ///

    pub(crate) fn push_str(&self, str_val: &str) {
        let cstr = CString::new(to_cesu8(str_val));

        match cstr {
            Ok(val) => {
                unsafe {
                    ffi::duk_push_lstring( self.ctx, val.as_ptr(),
                        val.as_bytes().len() as u64);
                }
            },
            Err(e) => eprintln!("couldn't push str {}: {}", str_val, e),
        }
    }

    ///
    /// Pop most recent string from duktape stack
    ///

    pub(crate) fn pop_str(&self) -> String {
        self.pop_str_idx(-1)
    }

    ///
    /// Pop string with given index from duktape stack
    ///
    /// * `idx` - Stack index
    //

    pub(crate) fn pop_str_idx(&self, idx: ffi::duk_idx_t) -> String {
        let mut len = 0;
        let lstring;
        let bytes;

        unsafe {
            lstring = ffi::duk_get_lstring_default(self.ctx,
                idx, &mut len, cstr!(""), 0);

            if lstring.is_null() {
                return String::new();
            }

            bytes = slice::from_raw_parts(lstring as *const u8,
            len as usize);
        }

        match from_cesu8(bytes) {
            Ok(string) => string.into_owned(),
            Err(_) => String::new()
        }
    }

    ///
    /// Eval given string
    ///
    /// * `str_val` - String to eval
    //

    pub fn eval(&self, str_val: &str) {
        let cstr = CString::new(str_val);

        match cstr {
            Ok(val) => {
                unsafe {
                    ffi::duk_eval_raw(self.ctx, val.as_ptr(),
                        val.into_bytes().len() as u64, 0);
                }
            },
            Err(e) => eprintln!("couldn't eval str {}: {}", str_val, e),
        }
    }

    ///
    /// Create and init duktape context
    ///

    unsafe fn create_heap() -> *mut ffi::duk_context {
        let ctx = ffi::duk_create_heap(None, None, None,
            ptr::null_mut(), Some(fatal_handler));

        ctx
    }
}

///
/// Handle duktape fatals errors - print the error and abort
///
/// * `data` - Userdata supplied to context
/// * `msg` - Error message
///

unsafe extern "C" fn fatal_handler(_udata: *mut c_void,
    msg: *const c_char)
{
    let msg = from_cesu8(CStr::from_ptr(msg).to_bytes())
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| "failed to decode message".to_string());

    eprintln!("fatal error from duktape: {}", msg);

    process::abort();
}

impl Drop for Rubtle {
    fn drop(&mut self) {
        unsafe {
            ffi::duk_destroy_heap(self.ctx);
        }
    }
}