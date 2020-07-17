///
/// @package Rubtle-Lib
///
/// @file Object functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//
use std::fmt;

#[derive(Debug)]
pub struct Object {}

impl Object {
    pub fn new() -> Object {
        Object {}
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object")
    }
}
