///
/// @package Rubtle-Lib
///
/// @file Value functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::convert::From;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Number(i32)
}

impl Value {
    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = *self {
            true
        } else {
            false
        }
    }
}

impl From<Value> for i32 {
    fn from(src: Value) -> i32 {
        if let Value::Number(val) = src {
            val
        } else {
            -1
        }
    }
}

impl From<i32> for Value {
    fn from(src: i32) -> Self {
        Value::Number(src)
    }
}