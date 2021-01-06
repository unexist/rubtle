///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - helper
/// @copyright 2020-2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::{Invocation, CallbackResult, Value};

///
/// Helper functions
///

pub fn js_printer(inv: Invocation<i8>) -> CallbackResult<Value> {
    let args = inv.args.unwrap();

    for val in args.iter() {
        match val.coerce_string() {
            Some(s) => println!("{:?}", s),
            None => eprintln!("Error unwrap value"),
        }
    }

    Ok(Value::from(true))
}

pub fn js_assert(inv: Invocation<i8>) -> CallbackResult<Value> {
    let args = inv.args.unwrap();
    let assert_val = args.first().unwrap().as_boolean().unwrap();
    let assert_mesg = args.last().unwrap().coerce_string().unwrap();

    assert_eq!(true, assert_val, "{}", assert_mesg);

    /* Make compiler happy */
    Ok(Value::from(true))
}
