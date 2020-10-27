///
/// @package Rubtle-Lib
///
/// @file Value tests
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::Value;

use std::collections::HashMap;

///
/// Create value
///

#[test]
fn create_none_value() {
    let rval = Value::from(());

    assert!(rval.is_none());

    assert_eq!((), rval.into());
}

#[test]
fn create_boolean_value() {
    let val: bool = true;
    let rval = Value::from(val);

    assert!(rval.is_boolean());
    assert_eq!(val, rval.into());
}

#[test]
fn create_number_value_i32() {
    let val: i32 = 4;
    let rval = Value::from(val);

    assert!(rval.is_number());
    assert_eq!(val, rval.into());
}

#[test]
fn create_number_value_f64() {
    let val: f64 = 4.0;
    let rval = Value::from(val);

    assert!(rval.is_number());
    assert_eq!(val, rval.into());
}

#[test]
fn create_string_value() {
    let val = "Test";
    let rval = Value::from(val);

    assert!(rval.is_string());

    let rval2: &str = rval.into();
    assert_eq!(val, rval2);
}

///
/// Create arrays
///

#[test]
fn create_array_value_bool() {
    let val = vec![true, false];
    let rval = Value::from(&val);

    assert!(rval.is_array());

    let ary: Vec<bool> = rval.into();

    assert_eq!(val, ary.as_slice());
}

#[test]
fn create_array_value_i32() {
    let val = vec![1, 2];
    let rval = Value::from(&val);

    assert!(rval.is_array());

    let ary: Vec<i32> = rval.into();

    assert_eq!(val, ary.as_slice());
}

#[test]
fn create_array_value_f64() {
    let val = vec![1.0, 2.0];
    let rval = Value::from(&val);

    assert!(rval.is_array());

    let ary: Vec<f64> = rval.into();

    assert_eq!(val, ary.as_slice());
}

#[test]
fn create_array_value_str() {
    let val = vec!["rubtle", "rubtle"];
    let rval = Value::from(&val);

    assert!(rval.is_array());

    let ary: Vec<&str> = rval.into();

    assert_eq!(val, ary.as_slice());
}

///
/// Create hash
///

#[test]
fn create_object_bool() {
    let mut val = HashMap::new();

    val.insert("rubtle1", true);
    val.insert("rubtle2", false);

    let rval = Value::from(&val);

    assert!(rval.is_object());

    let hash: HashMap<String, bool> = rval.into();

    for (k, v) in val {
        assert!(hash.contains_key(k));
        assert_eq!(v, *hash.get(k).unwrap());
    }
}

#[test]
fn create_object_i32() {
    let mut val = HashMap::new();

    val.insert("rubtle1", 1);
    val.insert("rubtle2", 2);

    let rval = Value::from(&val);

    assert!(rval.is_object());

    let hash: HashMap<String, i32> = rval.into();

    for (k, v) in val {
        assert!(hash.contains_key(k));
        assert_eq!(v, *hash.get(k).unwrap());
    }
}

#[test]
fn create_object_f64() {
    let mut val = HashMap::new();

    val.insert("rubtle1", 1.0);
    val.insert("rubtle2", 2.0);

    let rval = Value::from(&val);

    assert!(rval.is_object());

    let hash: HashMap<String, f64> = rval.into();

    for (k, v) in val {
        assert!(hash.contains_key(k));
        assert_eq!(v, *hash.get(k).unwrap());
    }
}

///
/// Convert values
///

#[test]
fn convert_none() {
    let val = ();
    let rval = Value::from(val);

    assert_eq!(val, rval.as_none().unwrap());
}

#[test]
fn convert_boolean() {
    let val: bool = true;
    let rval = Value::from(val);

    assert_eq!(val, rval.as_boolean().unwrap());
}

#[test]
fn convert_number() {
    let val: f64 = 4.0;
    let rval = Value::from(val);

    assert_eq!(val, rval.as_number().unwrap());
}

#[test]
fn convert_string() {
    let val = "Test";
    let rval = Value::from(val);

    assert_eq!(val, rval.as_string().unwrap());
}

///
/// Coerce to string
///

#[test]
fn coerce_none_to_string() {
    let strval = "None";
    let val = ();
    let rval = Value::from(val);

    assert_eq!(strval, rval.coerce_string().unwrap());
}

#[test]
fn coerce_bool_to_string() {
    let strval = "true";
    let val: bool = true;
    let rval = Value::from(val);

    assert_eq!(strval, rval.coerce_string().unwrap());
}

#[test]
fn coerce_number_to_string() {
    let strval = "4";
    let val: f64 = 4.0;
    let rval = Value::from(val);

    assert_eq!(strval, rval.coerce_string().unwrap());
}

#[test]
fn coerce_string_to_string() {
    let strval = "Test";
    let val = "Test";
    let rval = Value::from(val);

    assert_eq!(strval, rval.coerce_string().unwrap());
}
