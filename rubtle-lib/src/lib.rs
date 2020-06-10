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

#[cfg(test)] mod tests;
mod rubtle;
mod value;

pub use rubtle::Rubtle;
pub use rubtle::Value;
