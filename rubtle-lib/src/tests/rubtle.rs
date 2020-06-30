///
/// @package Rubtle-Lib
///
/// @file Rubtle tests
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::{Rubtle, Value};

#[test]
fn push_and_pop_bool_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from(true);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value();

    println!("{:?}", rval);

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_number_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from(4);

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value();

    println!("{:?}", rval);

    assert_eq!(rval, rval2);
}

#[test]
fn push_and_pop_string_value() {
    let rubtle = Rubtle::new();

    let rval = Value::from("Test");

    rubtle.push_value(&rval);
    let rval2 = rubtle.pop_value();

    println!("{:?}", rval);

    assert_eq!(rval, rval2);
}

#[test]
fn evil_eval_test() {
    let rubtle = Rubtle::new();

    rubtle.eval(r#"
        var rubtle = 'yeah';
    "#);
}