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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Str(String)
}

impl Value {

    ///
    /// Check whether valie is a boolean
    ///
    /// * `idx` - Stack index
    //

    pub fn is_bool(&self) -> bool {
        if let Value::Boolean(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Value::Str(_) = *self {
            true
        } else {
            false
        }
    }
}

///
/// Boolean
///

impl From<Value> for bool {
    fn from(src: Value) -> bool {
        if let Value::Boolean(val) = src {
            val
        } else {
            false
        }
    }
}

impl From<bool> for Value {
    fn from(src: bool) -> Self {
        Value::Boolean(src)
    }
}

///
/// Number
///

macro_rules! convert_num_type {
    ($num_type: ty) => {
        impl From<Value> for $num_type {
            fn from(src: Value) -> $num_type {
                if let Value::Number(val) = src {
                    val as $num_type
                } else {
                    -1 as $num_type
                }
            }
        }

        impl From<$num_type> for Value {
            fn from(src: $num_type) -> Self {
                Value::Number(src as f64)
            }
        }
    }
}

convert_num_type!(i32);
convert_num_type!(f64);

///
/// String
///

impl From<Value> for String {
    fn from(src: Value) -> String {
        if let Value::Str(val) = src {
            val
        } else {
            unimplemented!();
        }
    }
}

impl From<String> for Value {
    fn from(src: String) -> Self {
        Value::Str(src)
    }
}

impl From<Value> for &str {
    fn from(src: Value) -> &'static str {
        if let Value::Str(val) = src {
            Box::leak(val.into_boxed_str())
        } else {
            unimplemented!();
        }
    }
}

impl From<&str> for Value {
    fn from(src: &str) -> Self {
        Value::Str(src.into())
    }
}