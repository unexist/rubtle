///
/// @package Rubtle-Lib
///
/// @file Rubtle lib entry
/// @copyright 2020 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
extern crate cesu8;
extern crate rubtle_duktape as ffi;

#[macro_use]
mod util;

mod debug;
mod error;
mod invocation;
mod object_builder;
mod rubtle;
mod types;
mod value;

#[cfg(test)]
mod tests;

pub use error::Error;
pub use invocation::Invocation;
pub use object_builder::{Object, ObjectBuilder};
pub use rubtle::Rubtle;
pub use types::{Callback, Result};
pub use value::Value;
