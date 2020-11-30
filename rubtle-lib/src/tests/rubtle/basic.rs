///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - basic
/// @copyright 2020 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::{Rubtle, Value};

///
/// Stack primitives
///

#[test]
fn push_and_pop_bool_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from(true);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_number_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from(4);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_string_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from("rubtle");

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_none_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from(());

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value().unwrap();

    assert_eq!(rval, rval2);
}

///
/// Global
///

#[test]
fn set_global_bool_value() {
    let rubtle = Rubtle::new();
    let rval = Value::from(true);

    rubtle.set_global_value("rubtle", &rval);
}

#[test]
fn set_global_number_value() {
    let rubtle = Rubtle::new();
    let rval = Value::from(4);

    rubtle.set_global_value("rubtle", &rval);
}

#[test]
fn set_global_string_value() {
    let rubtle = Rubtle::new();
    let rval = Value::from("rubtle");

    rubtle.set_global_value("rubtle", &rval);
}

#[test]
fn get_global_bool_value() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = true;
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();
    let rval2 = Value::from(true);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_number_value_i32() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = 4;
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();
    let rval2 = Value::from(4);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_number_value_f64() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = 4.0;
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();
    let rval2 = Value::from(4.0);

    assert_eq!(rval, rval2);
}

#[test]
fn get_global_string_value() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = 'test';
    "#,
    );

    let rval = rubtle.get_global_value("rubtle").unwrap();
    let rval2 = Value::from("test");

    assert_eq!(rval, rval2);
}