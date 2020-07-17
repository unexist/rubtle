///
/// @package Rubtle-Lib
///
/// @file Rubtle lib entry
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

extern crate cesu8;
extern crate rubtle_duktape as ffi;

#[macro_use] mod util;

mod rubtle;
mod value;
mod invocation;
mod error;
mod types;
mod debug;

#[cfg(test)] mod tests;

pub use rubtle::Rubtle;
pub use value::Value;
pub use invocation::Invocation;
pub use error::Error;
pub use types::{Callback, Result};