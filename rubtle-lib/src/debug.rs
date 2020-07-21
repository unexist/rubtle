///
/// @package Rubtle-Lib
///
/// @file Debug functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//
use std::slice;

use cesu8::from_cesu8;

#[allow(dead_code)]
pub fn debug_stack(ctx: *mut ffi::duk_context, filename: &str, lineno: u32) {
    unsafe {
        ffi::duk_push_context_dump(ctx);

        let mut len: ffi::duk_size_t = 0;
        let data = ffi::duk_safe_to_lstring(ctx, -1, &mut len);

        if !data.is_null() {
            let bytes = slice::from_raw_parts(data as *const u8, len as usize);

            match from_cesu8(bytes) {
                Ok(string) => println!("\n{}@{}: {}", filename, lineno, string),
                Err(_) => unreachable!(),
            }
        }

        ffi::duk_pop(ctx);
    }
}
