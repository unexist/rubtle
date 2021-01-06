///
/// @package Rubtle
///
/// @file Rubtle main entry
/// @copyright 2020-2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
extern crate rubtle_lib as rubtle;

use rubtle::{Rubtle, ObjectBuilder, Invocation, Value, CallbackResult};

use std::{env, fs};

///
/// Helper
///

fn js_printer(inv: Invocation<i8>) -> CallbackResult<Value> {
    let args = inv.args.unwrap();

    for val in args.iter() {
        match val.coerce_string() {
            Some(s) => println!("<JS> {:?}", s),
            None => eprintln!("Error unwrap value"),
        }
    }

    Ok(Value::from(true))
}

fn js_assert_eq(inv: Invocation<i8>) -> CallbackResult<Value> {
    let args = inv.args.unwrap();
    let assert_a = args.first().unwrap();
    let assert_b = args.get(1).unwrap();
    let assert_mesg = args.last().unwrap().coerce_string().unwrap();

    assert_eq!(assert_a, assert_b, "<ASSERT> {}", assert_mesg);

    /* Make compiler happy */
    Ok(Value::from(true))
}

///
/// Init
///

fn init_global(rubtle: &Rubtle) {
    rubtle.set_global_function("print", js_printer);
    rubtle.set_global_function("assert", js_assert_eq);
}

fn init_rubtle(rubtle: &Rubtle) {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .with_method("inc", |inv| -> CallbackResult<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .with_method("set", |inv| -> CallbackResult<Value> {
            let mut udata = inv.udata.as_mut().unwrap();
            let args = inv.args.as_ref().unwrap();

            match args.first() {
                Some(val) => udata.value = val.as_number().unwrap() as i32,
                None => udata.value = 1,
            }

            Ok(Value::from(udata.value))
        })
        .with_method("get", |inv| -> CallbackResult<Value> {
            let udata = inv.udata.as_mut().unwrap();

            Ok(Value::from(udata.value))
        })
        .build();

    rubtle.set_global_object("Rubtle", &mut object);
}

///
/// Main
///

fn main() {
    let args: Vec<String> = env::args().collect();

    if 1 < args.len() {
        let contents = fs::read_to_string(&args[1]);
        let rubtle = Rubtle::new();

        init_global(&rubtle);
        init_rubtle(&rubtle);

        match contents {
            Ok(val) => rubtle.eval(&val),
            Err(_) => eprintln!("File read failed"),
        }
    } else {
        println!("Usage: {}: <JSFile>", args[0]);
    }
}
