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

#[test]
fn create_bool_value() {
    let val: bool = true;
    let rval = Value::from(val);

    println!("{:?}", rval);

    assert!(rval.is_bool());
    assert_eq!(val, rval.into());
}

#[test]
fn create_number_value() {
    let val: i32 = 4;
    let rval = Value::from(val);

    println!("{:?}", rval);

    assert!(rval.is_number());
    assert_eq!(val, rval.into());
}