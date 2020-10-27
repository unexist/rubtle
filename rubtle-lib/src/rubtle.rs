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
use std::{process, ptr, slice};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::collections::HashMap;

use cesu8::{from_cesu8, to_cesu8};

use crate::object_builder::Object;
use crate::types::{Callback, ObjectBuilderCall, ObjectBuilderCtor};
use crate::{Invocation, Result, Value};

#[allow(unused_imports)]
use crate::debug::*;

const FUNC: [i8; 6] = hidden_i8str!('f', 'u', 'n', 'c');
const CTOR: [i8; 6] = hidden_i8str!('c', 't', 'o', 'r');
const METH: [i8; 6] = hidden_i8str!('m', 'e', 't', 'h');
const UDATA: [i8; 7] = hidden_i8str!('u', 'd', 'a', 't', 'a');

pub struct Rubtle {
    /// Duktape context
    pub(crate) ctx: *mut ffi::duk_context,

    /// Whether to keep duktape heap during a drop; we share the
    /// heap between multiple instances of it with a single duktape heap
    pub(crate) drop_ctx: bool,
}

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
            drop_ctx: true,
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
                    ffi::duk_push_boolean(self.ctx, if *val { 1 } else { 0 });
                }

                Value::Number(val) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_number(self.ctx, *val);
                }

                Value::Str(val) => {
                    let cstr = CString::new(to_cesu8(&val[..]));

                    match cstr {
                        Ok(cval) => {
                            ffi::duk_require_stack(self.ctx, 1);
                            ffi::duk_push_lstring(
                                self.ctx,
                                cval.as_ptr(),
                                cval.as_bytes().len() as u64,
                            );
                        }
                        Err(_) => unimplemented!(),
                    }
                },

                Value::None => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_undefined(self.ctx);
                },

                Value::Array(_val) => {
                    unimplemented!();
                },

                Value::Object(_val) => {
                    unimplemented!();
                }
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

    fn handle_objects(&self) -> Option<Value> {
        unsafe {
            /* Handle arrays */
            if 1 == ffi::duk_is_array(self.ctx, -1) {
                let mut vec: Vec<Value> = Vec::new();

                ffi::duk_enum(self.ctx, -1, ffi::DUK_ENUM_ARRAY_INDICES_ONLY);

                while 0 != ffi::duk_next(self.ctx, -1, 1) {
                    match self.pop_value_at(-1) {
                        Some(val) => vec.push(Value::from(val)),
                        None => ffi::duk_pop(self.ctx),
                    }

                    /* Remove iter */
                    ffi::duk_pop(self.ctx);
                }

                /* Remove enum */
                ffi::duk_pop(self.ctx);

                Some(Value::Array(vec))
            } else if 1 == ffi::duk_is_object(self.ctx, -1) {
                let mut hash = HashMap::new();

                ffi::duk_enum(self.ctx, -1, 0);

                while 0 != ffi::duk_next(self.ctx, -1, 1) {
                    /* Pop value and key in reverse */
                    let value = self.pop_value_at(-1);
                    let key = self.pop_value_at(-1);

                    if !key.is_none() && !value.is_none() {
                        hash.insert(key.unwrap().as_string().unwrap().clone(), value.unwrap());
                    } else {
                        ffi::duk_pop_2(self.ctx);
                    }
                }

                /* Remove enum */
                ffi::duk_pop(self.ctx);

                Some(Value::Object(hash))
            } else {
                Some(Value::None)
            }
        }
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

                    let dval = ffi::duk_get_lstring(self.ctx, idx, &mut len);

                    assert!(!dval.is_null(), "string is null");

                    ffi::duk_remove(self.ctx, idx);

                    let bytes = slice::from_raw_parts(dval as *const u8, len as usize);

                    match from_cesu8(bytes) {
                        Ok(string) => Some(Value::Str(string.into_owned())),
                        Err(_) => None,
                    }
                },

                ffi::DUK_TYPE_OBJECT => self.handle_objects(),

                ffi::DUK_TYPE_UNDEFINED => {
                    Some(Value::None)
                },

                _ => None,
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
                    ffi::duk_put_global_lstring(
                        self.ctx,
                        cval.as_ptr(),
                        cval.as_bytes().len() as u64,
                    );
                }
                Err(_) => unimplemented!(),
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
                    ffi::duk_get_global_lstring(
                        self.ctx,
                        cval.as_ptr(),
                        cval.as_bytes().len() as u64,
                    );

                    self.pop_value()
                }
                Err(_) => None,
            }
        }
    }

    ///
    /// Set closure/function as a global function to call from JS
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the global
    /// * `func`- Closure/function to call
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Value, Invocation, Result};
    ///
    ///     let rubtle = Rubtle::new();
    ///
    ///     let printer = |inv: Invocation<i8>| -> Result<Value> {
    ///         let args = inv.args.unwrap();
    ///         let s = args.first().unwrap();
    ///
    ///         println!("{:?}", s.as_string().unwrap());
    ///
    ///         Ok(Value::from(true))
    ///     };
    ///
    ///     rubtle.set_global_function("print", printer);
    ///

    pub fn set_global_function<F>(&self, name: &str, func: F)
    where
        F: 'static + Fn(Invocation<i8>) -> Result<Value>,
    {
        unsafe extern "C" fn wrapper<T>(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t {
            /* Get arguments from stack */
            let rubtle = Rubtle {
                ctx: ctx,
                drop_ctx: false,
            };
            let nargs = ffi::duk_get_top(ctx) as usize;
            let mut args = Vec::with_capacity(nargs);

            for i in 0..nargs {
                ffi::duk_dup(ctx, i as ffi::duk_idx_t);

                match rubtle.pop_value() {
                    Some(val) => args.push(val),
                    None => eprintln!("Unwrap of None value"),
                }
            }

            /* Assemble invocation */
            let invocation = Invocation::<i8> {
                rubtle: &rubtle,
                args: Some(args),
                udata: None,
            };

            /* Fetch pointer from duktape */
            ffi::duk_push_current_function(ctx);
            ffi::duk_get_prop_string(ctx, -1, FUNC.as_ptr() as *const _);
            let func_ptr = ffi::duk_get_pointer(ctx, -1) as *mut Callback<i8>;
            ffi::duk_pop_n(ctx, 2);

            /* Wrap function and finally call it */
            let wrapped_func = || (*func_ptr)(invocation);
            let result = match catch_unwind(AssertUnwindSafe(wrapped_func)) {
                Ok(result) => result,
                Err(_) => {
                    ffi::duk_fatal_raw(ctx, cstr!("fatal error on func call"));
                    unreachable!();
                }
            };

            match result {
                Ok(value) => {
                    rubtle.push_value(&value);
                    1
                }
                Err(_) => -1,
            }
        }

        unsafe extern "C" fn finalizer<T>(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t {
            ffi::duk_require_stack(ctx, 1);
            ffi::duk_get_prop_string(ctx, 0, FUNC.as_ptr() as *const _);

            /* Get box and drop it */
            let callback = Box::from_raw(ffi::duk_get_pointer(ctx, -1) as *mut Callback<i8>);

            drop(callback);

            ffi::duk_pop(ctx);
            ffi::duk_push_undefined(ctx);
            ffi::duk_put_prop_string(ctx, 0, FUNC.as_ptr() as *const _);

            0
        }

        unsafe {
            let cstr = CString::new(to_cesu8(name));

            match cstr {
                Ok(cval) => {
                    ffi::duk_require_stack(self.ctx, 2);
                    ffi::duk_push_c_function(self.ctx, Some(wrapper::<i8>), -1); //< (DUK_VARARGS)

                    /* Store wrapper */
                    let boxed_func = Box::into_raw(Box::new(Box::new(func) as Callback<i8>));

                    assert!(!boxed_func.is_null(), "Null function pointer");

                    ffi::duk_push_pointer(self.ctx, boxed_func as *mut _);
                    ffi::duk_put_prop_string(self.ctx, -2, FUNC.as_ptr() as *const _);

                    /* Store finalizer */
                    ffi::duk_push_c_function(self.ctx, Some(finalizer::<i8>), 1);
                    ffi::duk_set_finalizer(self.ctx, -2);

                    /* Finally store as global function */
                    ffi::duk_put_global_lstring(
                        self.ctx,
                        cval.as_ptr(),
                        cval.as_bytes().len() as u64,
                    );
                }
                Err(_) => unimplemented!(),
            }
        }
    }

    ///
    /// Create a global object for JS
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the global
    /// * `object`- Object from ObjectBuilder
    ///
    /// # Example
    ///
    ///     use rubtle_lib::{Rubtle, Result, Value, ObjectBuilder};
    ///
    ///     let rubtle = Rubtle::new();
    ///
    ///     #[derive(Default)]
    ///     struct UserData {
    ///         value: i32,
    ///     };
    ///
    ///     let mut object = ObjectBuilder::<UserData>::new()
    ///         .with_constructor(|inv| {
    ///             let mut udata = inv.udata.as_mut().unwrap();
    ///
    ///             udata.value = 0;
    ///          })
    ///         .build();
    ///
    ///     let rubtle = Rubtle::new();
    ///
    ///     rubtle.set_global_object("Printer", &mut object);
    ///

    pub fn set_global_object<T>(&self, name: &str, object: &mut Object<T>)
    where
        T: Default + 'static,
    {
        unsafe extern "C" fn ctor_wrapper<T>(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t
        where
            T: Default + 'static,
        {
            /* Verify this is a constrcutor call */
            if 0 == ffi::duk_is_constructor_call(ctx) {
                return -1;
            }

            /* Get arguments from stack */
            let rubtle = Rubtle {
                ctx: ctx,
                drop_ctx: false,
            };
            let nargs = ffi::duk_get_top(ctx) as usize;
            let mut args = Vec::with_capacity(nargs);

            for i in 0..nargs {
                ffi::duk_dup(ctx, i as ffi::duk_idx_t);

                match rubtle.pop_value() {
                    Some(val) => args.push(val),
                    None => eprintln!("Unwrap of None value"),
                }
            }

            /* Create invocation data */
            let mut inv = Invocation {
                rubtle: &rubtle,
                args: Some(args),
                udata: Some(T::default()),
            };

            /* Fetch pointer from duktape */
            ffi::duk_push_current_function(ctx);
            ffi::duk_get_prop_string(ctx, -1, CTOR.as_ptr() as *const _);
            let func_ptr = ffi::duk_get_pointer(ctx, -1) as *mut ObjectBuilderCtor<T>;
            ffi::duk_pop_n(ctx, 2);

            assert!(!func_ptr.is_null(), "Null function pointer");

            /* Wrap function and finally call it */
            let wrapped_func = || (*func_ptr)(&mut inv);

            let _result = match catch_unwind(AssertUnwindSafe(wrapped_func)) {
                Ok(res) => res,
                Err(_) => {
                    ffi::duk_fatal_raw(ctx, cstr!("Fatal error on func call"));
                    unreachable!();
                }
            };

            let boxed_udata = Box::into_raw(Box::new(inv));

            assert!(!boxed_udata.is_null(), "Null user data pointer");

            /* Store user data */
            ffi::duk_push_this(ctx);
            ffi::duk_push_pointer(ctx, boxed_udata as *mut _);
            ffi::duk_put_prop_string(ctx, -2, UDATA.as_ptr() as *const _);

            ffi::duk_pop(ctx);

            0
        }

        unsafe extern "C" fn meth_wrapper<T>(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t
        where
            T: Default + 'static,
        {
            /* Get arguments from stack */
            let rubtle = Rubtle {
                ctx: ctx,
                drop_ctx: false,
            };
            let nargs = ffi::duk_get_top(ctx) as usize;
            let mut args = Vec::with_capacity(nargs);

            for i in 0..nargs {
                ffi::duk_dup(ctx, i as ffi::duk_idx_t);

                match rubtle.pop_value() {
                    Some(val) => args.push(val),
                    None => eprintln!("Unwrap of None value"),
                }
            }

            /* Fetch pointer from duktape */
            ffi::duk_push_current_function(ctx);
            ffi::duk_get_prop_string(ctx, -1, METH.as_ptr() as *const _);
            let func_ptr = ffi::duk_get_pointer(ctx, -1) as *mut ObjectBuilderCall<T>;
            ffi::duk_pop_n(ctx, 2);

            assert!(!func_ptr.is_null(), "Null function pointer");

            /* Fetch user data from duktape */
            ffi::duk_push_this(ctx);
            ffi::duk_get_prop_string(ctx, -1, UDATA.as_ptr() as *const _);
            let inv_ptr = ffi::duk_get_pointer(ctx, -1) as *mut Invocation<T>;
            ffi::duk_pop_n(ctx, 2);

            assert!(!inv_ptr.is_null(), "Null user data pointer");

            (*inv_ptr).args = Some(args);

            /* Wrap function and finally call it */
            let wrapped_func = || (*func_ptr)(&mut *inv_ptr);
            let result = match catch_unwind(AssertUnwindSafe(wrapped_func)) {
                Ok(res) => res,
                Err(_) => {
                    ffi::duk_fatal_raw(ctx, cstr!("Fatal error on func call"));
                    unreachable!();
                }
            };

            match result {
                Ok(val) => {
                    rubtle.push_value(&val);

                    1
                }
                Err(_) => 0,
            }
        }

        unsafe {
            let cstr = CString::new(to_cesu8(name));

            match cstr {
                Ok(cval) => {
                    ffi::duk_push_c_function(self.ctx, Some(ctor_wrapper::<T>), -1);

                    /* Store ctor wrapper */
                    match object.take_constructor() {
                        Some(ctor) => {
                            let boxed_func = Box::into_raw(Box::new(ctor));

                            ffi::duk_push_pointer(self.ctx, boxed_func as *mut _);
                            ffi::duk_put_prop_string(self.ctx, -2, CTOR.as_ptr() as *const _);
                        }
                        None => {
                            ffi::duk_fatal_raw(self.ctx, cstr!("No constructor"));
                            unreachable!();
                        }
                    }

                    ffi::duk_push_object(self.ctx);

                    /* Store method wrapper */
                    for (name, meth) in object {
                        let cstr = CString::new(to_cesu8(name));

                        match cstr {
                            Ok(cval) => {
                                let boxed_func = Box::into_raw(Box::new(meth));

                                ffi::duk_push_c_function(self.ctx, Some(meth_wrapper::<T>), -1); //< (DUK_VARARGS)

                                ffi::duk_push_pointer(self.ctx, boxed_func as *mut _);
                                ffi::duk_put_prop_string(self.ctx, -2, METH.as_ptr() as *const _);

                                ffi::duk_put_prop_lstring(
                                    self.ctx,
                                    -2,
                                    cval.as_ptr(),
                                    cval.as_bytes().len() as u64,
                                );
                            }
                            Err(_) => unimplemented!(),
                        }
                    }

                    ffi::duk_put_prop_string(self.ctx, -2, cstr!("prototype"));
                    ffi::duk_put_global_lstring(
                        self.ctx,
                        cval.as_ptr(),
                        cval.as_bytes().len() as u64,
                    );
                }
                Err(_) => unimplemented!(),
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
            Ok(val) => unsafe {
                ffi::duk_eval_raw(
                    self.ctx,
                    val.as_ptr(),
                    val.into_bytes().len() as u64,
                    ffi::DUK_COMPILE_EVAL
                        | ffi::DUK_COMPILE_NOSOURCE
                        | ffi::DUK_COMPILE_NORESULT
                        | ffi::DUK_COMPILE_NOFILENAME,
                );
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
        let ctx = ffi::duk_create_heap(None, None, None, ptr::null_mut(), Some(fatal_handler));

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

unsafe extern "C" fn fatal_handler(_udata: *mut c_void, msg: *const c_char) {
    let msg = from_cesu8(CStr::from_ptr(msg).to_bytes())
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| "failed to decode message".to_string());

    eprintln!("fatal error from duktape: {}", msg);

    process::abort();
}

impl Drop for Rubtle {
    fn drop(&mut self) {
        /* Check wether heap needs to be kept alive */
        if self.drop_ctx {
            unsafe {
                ffi::duk_destroy_heap(self.ctx);
            }
        }
    }
}
