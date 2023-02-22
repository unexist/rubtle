///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - arrays
/// @copyright 2020-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::{Rubtle, Value};

///
/// Stack arrays
///

#[test]
fn push_and_pop_array_value_i32() {
    let rubtle = Rubtle::new();

    let ary: Vec<i32> = vec![1, 2];
    let rval = Value::from(&ary);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_array_value_f64() {
    let rubtle = Rubtle::new();

    let ary: Vec<f64> = vec![1.0, 2.0];
    let rval = Value::from(&ary);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_array_value_bool() {
    let rubtle = Rubtle::new();

    let ary: Vec<bool> = vec![true, false];
    let rval = Value::from(&ary);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

///
/// Global arrays
///

#[test]
fn get_global_array_value_bool() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = [true, false];
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let vec = vec![true, false];
    let rval2 = Value::from(&vec);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_array_value_i32() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = [1, 2];
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let vec = vec![1, 2];
    let rval2 = Value::from(&vec);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_array_value_f64() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = [1.0, 2.0];
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let vec = vec![1.0, 2.0];
    let rval2 = Value::from(&vec);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_array_value_str() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = ["rubtle", "rubtle"];
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();

    let vec = vec!["rubtle", "rubtle"];
    let rval2 = Value::from(&vec);

    assert_eq!(rval, rval2);
}