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
    pub(crate) ctx: *mut ffi::duk_context,
}

impl Rubtle {
    pub fn new() -> Rubtle {
        Rubtle {
            ctx: unsafe { Self::create_heap() },
        }
    }

    pub unsafe fn push_str(&self, str_val: &str) {
        let cstr = CString::new(to_cesu8(str_val));

        match cstr {
            Ok(val) => {
                ffi::duk_push_lstring( self.ctx, val.as_ptr(),
                    val.as_bytes().len() as u64);
            },
            Err(e) => eprintln!("couldn't push str {}: {}", str_val, e),
        }
    }

    pub unsafe fn pop_str(&self, idx: ffi::duk_idx_t) -> String {
        let mut len = 0;

        let string = ffi::duk_get_lstring_default(self.ctx,
            idx, &mut len, cstr!(""), 0);

        if string.is_null() {
            return String::new();
        }

        let bytes = slice::from_raw_parts(string as *const u8,
            len as usize);

        match from_cesu8(bytes) {
            Ok(string) => string.into_owned(),
            Err(_) => String::new()
        }
    }

    pub unsafe fn eval(&self, str_val: &str) {
        let cstr = CString::new(str_val);

        match cstr {
            Ok(val) => {
                ffi::duk_eval_raw(self.ctx, val.as_ptr(),
                    val.into_bytes().len() as u64, 0);
            },
            Err(e) => eprintln!("couldn't eval str {}: {}", str_val, e),
        }
    }

    unsafe fn create_heap() -> *mut ffi::duk_context {
        let ctx = ffi::duk_create_heap(None, None, None,
            ptr::null_mut(), Some(fatal_handler));

        ctx
    }
}

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