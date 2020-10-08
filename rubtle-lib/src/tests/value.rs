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

///
/// Create value
///

#[test]
fn create_boolean_value() {
    let val: bool = true;
    let rval = Value::from(val);

    assert!(rval.is_boolean());
    assert_eq!(val, rval.into());
}

#[test]
fn create_number_value() {
    let val: i32 = 4;
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

#[test]
fn create_none_value() {
    let rval = Value::from(());

    assert!(rval.is_none());

    assert_eq!((), rval.into());
}

///
/// Convert values
///

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

#[test]
fn convert_none() {
    let val = ();
    let rval = Value::from(val);

    assert_eq!(val, rval.as_none().unwrap());
}

///
/// Coerce to string
///

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

#[test]
fn coerce_none_to_string() {
    let strval = "None";
    let val = ();
    let rval = Value::from(val);

    assert_eq!(strval, rval.coerce_string().unwrap());
}