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

use crate::Rubtle;

#[test]
fn create_string_value() {
    let literal = "String Test";
    let rval = Value::new(literal);

    println!("{:?}", rval);

    assert!(rval.is_string());
    assert_eq!(literal, rval.)
}