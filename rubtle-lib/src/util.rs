///
/// @package Rubtle-Lib
///
/// @file Util functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::os::raw::c_char;

macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0")
            as *const str
            as *const [::std::os::raw::c_char]
            as *const ::std::os::raw::c_char
    )
}

macro_rules! assert_stack {
    ($ctx:expr, $diff:expr, $body:block) => {
        {
            let initial_stack_height = $crate::ffi::duk_get_top($ctx);
            let result = $body;
            assert_eq!(initial_stack_height + $diff, $crate::ffi::duk_get_top($ctx));
            result
        }
    }
}