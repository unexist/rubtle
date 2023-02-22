///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - objects
/// @copyright 2020-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::{Rubtle, Value};

use std::collections::HashMap;

///
/// Stack objects
///

#[test]
fn push_and_pop_object_i32() {
    let rubtle = Rubtle::new();

    let mut hash: HashMap<&str, i32> = HashMap::new();

    hash.insert("rubtle", 1);
    hash.insert("rubtle", 2);

    let rval = Value::from(&hash);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_object_f64() {
    let rubtle = Rubtle::new();

    let mut hash: HashMap<&str, f64> = HashMap::new();

    hash.insert("rubtle", 1.0);
    hash.insert("rubtle", 2.0);

    let rval = Value::from(&hash);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_object_bool() {
    let rubtle = Rubtle::new();

    let mut hash: HashMap<&str, bool> = HashMap::new();

    hash.insert("rubtle", true);
    hash.insert("rubtle", false);

    let rval = Value::from(&hash);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

///
/// Global objects
///

#[test]
fn get_global_object_value_bool() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = { "rubtle1": true, "rubtle2": false };
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let mut hash: HashMap<&str, bool> = HashMap::new();

    hash.insert("rubtle1", true);
    hash.insert("rubtle2", false);

    let rval2 = Value::from(&hash);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_object_value_i32() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = { "rubtle1": 1, "rubtle2": 2 };
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let mut hash: HashMap<&str, i32> = HashMap::new();

    hash.insert("rubtle1", 1);
    hash.insert("rubtle2", 2);

    let rval2 = Value::from(&hash);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_object_value_f64() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = { "rubtle1": 1.0, "rubtle2": 2.0 };
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let mut hash: HashMap<&str, f64> = HashMap::new();

    hash.insert("rubtle1", 1.0);
    hash.insert("rubtle2", 2.0);

    let rval2 = Value::from(&hash);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_object_value_str() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = { "rubtle1": "rubtle", "rubtle2": "rubtle" };
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let mut hash: HashMap<&str, &str> = HashMap::new();

    hash.insert("rubtle1", "rubtle");
    hash.insert("rubtle2", "rubtle");

    let rval2 = Value::from(&hash);

    assert_eq!(rval, rval2);
}