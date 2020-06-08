use rubtle_duktape as rub_duk;

use std::ptr;

pub struct Rubtle {
    pub(crate) ctx: *mut rub_duk::duk_context,
    pub(crate) is_top: bool,
}

impl Rubtle {
    pub fn new() -> Rubtle {
        Rubtle {
            ctx: unsafe { Self::create_heap() },
            is_top: true
        }
    }

    unsafe fn create_heap() -> *mut rub_duk::duk_context {
        let ctx = rub_duk::duk_create_heap(None, None, None,
            ptr::null_mut(), None);

        ctx
    }
}

impl Drop for Rubtle {
    fn drop(&mut self) {
        unsafe {
            rub_duk::duk_destroy_heap(self.ctx);
        }
    }
}