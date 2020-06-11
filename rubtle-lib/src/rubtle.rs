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

use crate::Value;

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
    /// Push value onto duktape stack
    ///
    /// * `rval` - String value to push
    ///

    pub(crate) fn push_value(&self, rval: Value) {
        unsafe {
            match rval {
                Value::Boolean(val) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_boolean(self.ctx,
                        if val { 1 } else { 0 });
                },
                Value::Number(val) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_number(self.ctx, val);
                },
                _ => {
                    unimplemented!();
                },
            }
        }
    }

    ///
    /// Pop most recent value from duktape stack
    ///

    pub(crate) fn pop_value(&self) -> Value {
        self.pop_value_at(-1)
    }

    ///
    /// Pop value on given index from duktape stack
    ///
    /// * `idx` - Stack index
    //

    pub(crate) fn pop_value_at(&self, idx: ffi::duk_idx_t) -> Value {
        unsafe {
            match ffi::duk_get_type(self.ctx, idx) as u32 {
               ffi::DUK_TYPE_BOOLEAN => {
                    let dval = ffi::duk_get_boolean(self.ctx, idx);

                    ffi::duk_remove(self.ctx, idx);

                    Value::Boolean(0 != dval)
                },
                ffi::DUK_TYPE_NUMBER => {
                    let dval = ffi::duk_get_number(self.ctx, idx);

                    ffi::duk_remove(self.ctx, idx);

                    Value::Number(dval)
                },
                _ => {
                    unimplemented!();
                },
            }
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

    pub fn create_global(&self) {

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