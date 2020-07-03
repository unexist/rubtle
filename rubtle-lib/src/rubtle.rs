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
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::{Value, Invocation, Result};

pub struct Rubtle {
    /// Duktape context
    pub(crate) ctx: *mut ffi::duk_context,
}

const FUNC: [i8; 6] = hidden_i8str!('f', 'u', 'n', 'c');

pub(crate) type Callback = Box<dyn Fn(Invocation) -> Result<Value>>;

impl Rubtle {

    ///
    /// Create a new Rubtle instance
    ///
    /// # Example
    ///
    ///     use rubtle_lib::Rubtle;
    ///
    ///     let rubtle = Rubtle::new();
    ///

    pub fn new() -> Rubtle {
        Rubtle {
            ctx: unsafe { Self::create_heap() },
        }
    }

    ///
    /// Push value onto duktape stack
    ///
    /// # Arguments
    ///
    /// * `rval` - String value to push
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Value};
    ///
    ///     let rubtle = Rubtle::new();
    ///     let rval = Value::from(4);
    ///
    ///     rubtle.push_value(&rval);
    ///

    pub fn push_value(&self, rval: &Value) {
        unsafe {
            match rval {
                Value::Boolean(val) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_boolean(self.ctx,
                        if *val { 1 } else { 0 });
                },

                Value::Number(val) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_number(self.ctx, *val);
                },

                Value::Str(val) => {
                    let cstr = CString::new(to_cesu8(&val[..]));

                    match cstr {
                        Ok(cval) => {
                            ffi::duk_require_stack(self.ctx, 1);
                            ffi::duk_push_lstring(self.ctx,
                                cval.as_ptr(), cval.as_bytes().len() as u64);
                        },
                        Err(_) => unimplemented!()
                    }
                },
            }
        }
    }

    ///
    /// Set value to context and assign a global reachable name
    ///
    /// # Arguments
    ///
    /// `name`- Name of the value
    /// `rval` - The actual value
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Value};
    ///
    ///     let rubtle = Rubtle::new();
    ///     let rval = Value::from(4);
    ///
    ///     rubtle.set_global_value("rubtle", &rval);
    ///

    pub fn set_global_value(&self, name: &str, rval: &Value) {
        unsafe {
            let cstr = CString::new(to_cesu8(name));

            match cstr {
                Ok(cval) => {
                    self.push_value(rval);

                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_put_global_lstring(self.ctx,
                        cval.as_ptr(), cval.as_bytes().len() as u64);
                },
                Err(_) => unimplemented!()
            }
        }
    }

    ///
    /// Get value from context for given global reachable name
    ///
    /// # Arguments
    ///
    /// `name`- Name of the value
    ///
    /// # Returns
    ///
    /// Any value on top of the stack as `Option<Value>`
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Value};
    ///
    ///     let rubtle = Rubtle::new();
    ///
    ///     rubtle.get_global_value("rubtle");
    ///

    pub fn get_global_value(&self, name: &str) -> Option<Value> {
        unsafe {
            let cstr = CString::new(to_cesu8(name));

            match cstr {
                Ok(cval) => {
                    ffi::duk_get_global_lstring(self.ctx,
                        cval.as_ptr(), cval.as_bytes().len() as u64);

                    self.pop_value()
                },
                Err(_) => None
            }
        }
    }

    pub fn set_global_function<F>(&self, name: &str, func: F)
        where
            F: Fn(Invocation) -> Result<Value>
    {
        unsafe extern "C" fn wrapper(ctx: *mut ffi::duk_context) ->
            ffi::duk_ret_t
        {
            let rubtle = Rubtle { ctx };
            let nargs = ffi::duk_get_top(ctx) as usize;
            let mut args = Vec::with_capacity(nargs);

            //ffi::duk_push_this(ctx);
            //let this = ducc.pop_value();

            for i in 0..nargs {
                ffi::duk_dup(ctx, i as ffi::duk_idx_t);
                args.push(rubtle.pop_value().unwrap());
            }

            let invocation = Invocation {
                rubtle: &rubtle,
                this: Value::from(1),
                args: args,
            };

            ffi::duk_get_prop_string(ctx, -1, FUNC.as_ptr() as *const _);
            let func_ptr = ffi::duk_get_pointer(ctx, -1) as *mut Callback;

            //let inner = || (*func)(&rubtle, (), args);
            let inner = move || (* func_ptr)(invocation);

            let result = match catch_unwind(AssertUnwindSafe(inner)) {
                Ok(result) => result,
                Err(_) => {
                    ffi::duk_fatal_raw(ctx, cstr!("panic occurred during script execution"));
                    unreachable!();
                },
            };

            match result {
                Ok(value) => {
                    rubtle.push_value(&value);
                    1
                },
                Err(_) => {
                    -1
                },
            }
        }

        unsafe extern "C" fn finalizer(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t {
            ffi::duk_require_stack(ctx, 1);

            0
        }

        unsafe {
            let cstr = CString::new(to_cesu8(name));

            match cstr {
                Ok(cval) => {
                    ffi::duk_require_stack(self.ctx, 2);
                    ffi::duk_push_c_function(self.ctx, Some(wrapper), -1); //< (DUK_VARARGS)
                    ffi::duk_push_pointer(self.ctx, Box::into_raw(Box::new(func)) as *mut _);
                    ffi::duk_push_c_function(self.ctx, Some(finalizer), 1);
                    ffi::duk_set_finalizer(self.ctx, -2);
                    ffi::duk_put_global_lstring(self.ctx,
                        cval.as_ptr(), cval.as_bytes().len() as u64);
                },
                Err(_) => unimplemented!()
            }
        }
    }

    ///
    /// Pop most recent value from duktape stack
    ///
    /// # Returns
    ///
    /// Any value on top of the stack as `Value`
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Value};
    ///
    ///     let rubtle = Rubtle::new();
    ///     let rval = Value::from(4);
    ///
    ///     rubtle.push_value(&rval);
    ///
    ///     let rval2 = rubtle.pop_value().unwrap();
    ///

    pub fn pop_value(&self) -> Option<Value> {
        self.pop_value_at(-1)
    }

    ///
    /// Pop value on given index from duktape stack
    ///
    /// # Arguments
    ///
    /// * `idx` - Stack index; -1 for top
    ///
    /// # Returns
    ///
    /// Any value on top of the stack as `Option<Value>`
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Value};
    ///
    ///     let rubtle = Rubtle::new();
    ///     let rval = Value::from(4);
    ///
    ///     rubtle.push_value(&rval);
    ///
    ///     let rval2 = rubtle.pop_value_at(-1).unwrap();
    //

    pub fn pop_value_at(&self, idx: ffi::duk_idx_t) -> Option<Value> {
        unsafe {
            match ffi::duk_get_type(self.ctx, idx) as u32 {
               ffi::DUK_TYPE_BOOLEAN => {
                    let dval = ffi::duk_get_boolean(self.ctx, idx);

                    ffi::duk_remove(self.ctx, idx);

                    Some(Value::Boolean(0 != dval))
                },

                ffi::DUK_TYPE_NUMBER => {
                    let dval = ffi::duk_get_number(self.ctx, idx);

                    ffi::duk_remove(self.ctx, idx);

                    Some(Value::Number(dval))
                },

                ffi::DUK_TYPE_STRING => {
                    let mut len = 0;

                    let dval = ffi::duk_get_lstring(self.ctx,
                        idx, &mut len);

                        if dval.is_null() {
                            unimplemented!();
                        }

                        let bytes = slice::from_raw_parts(dval as *const u8,
                            len as usize);

                        match from_cesu8(bytes) {
                            Ok(string) => {
                                Some(Value::Str(string.into_owned()))
                            },
                            Err(_) => None
                        }
                },

                _ => {
                    None
                },
            }
        }
    }

    ///
    /// Eval given string
    ///
    /// # Arugments
    ///
    /// * `str_val` - String to eval
    ///
    /// # Example
    ///
    ///     use rubtle_lib::Rubtle;
    ///
    ///     let rubtle = Rubtle::new();
    ///
    ///     rubtle.eval(r#"
    ///         var rubtle = 4;
    ///     "#)
    ///

    pub fn eval(&self, str_val: &str) {
        let cstr = CString::new(str_val);

        match cstr {
            Ok(val) => {
                unsafe {
                    ffi::duk_eval_raw(self.ctx, val.as_ptr(),
                        val.into_bytes().len() as u64,
                            ffi::DUK_COMPILE_EVAL|
                            ffi::DUK_COMPILE_NOSOURCE|
                            ffi::DUK_COMPILE_NORESULT|
                            ffi::DUK_COMPILE_NOFILENAME);
                }
            },
            Err(e) => eprintln!("couldn't eval str {}: {}", str_val, e),
        }
    }

    ///
    /// Create and init duktape context
    ///
    /// # Returns
    ///
    /// A new duktape heap context
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
/// # Arguments
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