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
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Boolean(bool),
    Number(f64),
    Str(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {

    ///
    /// Check whether value is a string
    ///
    /// Returns
    ///
    /// `true` if the value is a string; otherwise `false`
    ///

    pub fn is_none(&self) -> bool {
        if let Value::None = self {
            true
        } else {
            false
        }
    }

    ///
    /// Check whether value is a boolean
    ///
    /// Returns
    ///
    /// `true` if the value is a bool; otherwise `false`
    ///

    pub fn is_boolean(&self) -> bool {
        if let Value::Boolean(_) = *self {
            true
        } else {
            false
        }
    }

    ///
    /// Check whether value is a number
    ///
    /// Returns
    ///
    /// `true` if the value is a number; otherwise `false`
    ///

    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = *self {
            true
        } else {
            false
        }
    }

    ///
    /// Check whether value is a string
    ///
    /// Returns
    ///
    /// `true` if the value is a string; otherwise `false`
    ///

    pub fn is_string(&self) -> bool {
        if let Value::Str(_) = *self {
            true
        } else {
            false
        }
    }

    ///
    /// Check whether value is an array
    ///
    /// Returns
    ///
    /// `true` if the value is an array; otherwise `false`
    ///

    pub fn is_array(&self) -> bool {
        if let Value::Array(_) = *self {
            true
        } else {
            false
        }
    }

    ///
    /// Check whether value is an object
    ///
    /// Returns
    ///
    /// `true` if the value is an object; otherwise `false`
    ///

    pub fn is_object(&self) -> bool {
        if let Value::Object(_) = *self {
            true
        } else {
            false
        }
    }

    ///
    /// Return inner none value
    ///
    /// Returns
    ///
    /// `Option` either with value or without
    ///

    pub fn as_none(&self) -> Option<()> {
        if let Value::None = *self {
            Some(())
        } else {
            None
        }
    }

    ///
    /// Return inner boolean value
    ///
    /// Returns
    ///
    /// `Option` either with value or without
    ///

    pub fn as_boolean(&self) -> Option<bool> {
        if let Value::Boolean(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    ///
    /// Return inner number value
    ///
    /// Returns
    ///
    /// `Option` either with value or without
    ///

    pub fn as_number(&self) -> Option<f64> {
        if let Value::Number(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    ///
    /// Return inner string value
    ///
    /// Returns
    ///
    /// `Option` either with value or without
    ///

    pub fn as_string(&self) -> Option<&String> {
        if let Value::Str(ref value) = *self {
            Some(value)
        } else {
            None
        }
    }

    ///
    /// Coerce value to string
    ///
    /// Returns
    ///
    /// Coerced `String`
    ///

    pub fn coerce_string(&self) -> Option<String> {
        match self {
            Value::None => Some(String::from("None")),
            Value::Number(val) => Some(val.to_string()),
            Value::Boolean(val) => Some(val.to_string()),
            Value::Str(val) => Some(val.clone()),
            Value::Array(_val) => unimplemented!(),
            Value::Object(_val) => unimplemented!(),
        }
    }
}

///
/// Empty tuple
///

impl From<Value> for () {
    fn from(src: Value) -> () {
        if let Value::None = src {
            ()
        } else {
            unimplemented!();
        }
    }
}


impl From<()> for Value {
    fn from(_src: ()) -> Self {
        Value::None
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
    };
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

///
/// Array
///

macro_rules! convert_array_type {
    ($array_type: ty) => {
        impl<'rubtle> From<Value> for Vec<$array_type> {
            fn from(src: Value) -> Vec<$array_type> {
                if let Value::Array(val) = src {
                    let mut ary = Vec::new();

                    for v in val {
                        ary.push(Value::into(v))
                    }

                    ary
                } else {
                    unimplemented!();
                }
            }
        }

        impl<'rubtle> From<&Vec<$array_type>> for Value {
            fn from(src: &Vec<$array_type>) -> Self {
                let mut ary = Vec::new();

                for var in src {
                    ary.push(Value::from(*var))
                }

                Value::Array(ary)
            }
        }
    };
}

convert_array_type!(bool);
convert_array_type!(i32);
convert_array_type!(f64);
convert_array_type!(&'rubtle str);

///
/// Object
///

macro_rules! convert_object_type {
    ($obj_type: ty) => {
        impl<'rubtle> From<Value> for HashMap<String, $obj_type> {
            fn from(src: Value) -> HashMap<String, $obj_type> {
                if let Value::Object(val) = src {
                    let mut hash: HashMap<String, $obj_type> = HashMap::new();

                    for (k, v) in val {
                        hash.insert(k, v.into());
                    }

                    hash
                } else {
                    unimplemented!();
                }
            }
        }

        impl<'rubtle> From<&HashMap<&'rubtle str, $obj_type>> for Value {
            fn from(src: &HashMap<&'rubtle str, $obj_type>) -> Self {
                let mut hash = HashMap::new();

                for (k, v) in src {
                    hash.insert(k.to_string(), Value::from(*v));
                }

                Value::Object(hash)
            }
        }
    }
}

convert_object_type!(bool);
convert_object_type!(i32);
convert_object_type!(f64);