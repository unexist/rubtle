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
use crate::{Invocation, ObjectBuilder, Result, Rubtle, Value};
use crate::debug::*;

///
/// Stack
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

///
/// Eval
///

#[test]
fn evil_eval_test() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = 'yeah';
    "#,
    );
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
fn get_global_number_value() {
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

///
/// Global functions
///

#[test]
fn set_global_function() {
    let rubtle = Rubtle::new();

    rubtle.set_global_function("square", |inv: Invocation| -> Result<Value> {
        let i = inv.args.first().unwrap();

        Ok(Value::from(i.as_number().unwrap() * i.as_number().unwrap()))
    });
}

#[test]
fn set_and_run_global_printer() {
    let rubtle = Rubtle::new();

    rubtle.set_global_function("print", |inv: Invocation| -> Result<Value> {
        for val in inv.args.iter() {
            match val.coerce_string() {
                Some(s) => println!("{:?}", s),
                None => eprintln!("Error unwrap value"),
            }
        }

        Ok(Value::from(true))
    });

    rubtle.eval(
        r#"
        print('Test');
    "#,
    );
}

///
/// Global objects
///

#[test]
fn set_global_object_with_ctor() {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|mut user_data| {
            user_data.value = 1;
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.eval(
        r#"
        var counter = new Counter();
    "#,
    );
}

#[test]
fn set_global_object_with_ctor_and_methods() {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|mut user_data| {
            user_data.value = 1;
        })
        .with_method("count", |mut user_data| -> Result<Value> {
            user_data.value += 1;

            Ok(Value::from(user_data.value))
        })
        .with_method("print", |user_data| -> Result<Value> {
            println!("Value={}", user_data.value);

            Ok(Value::from(user_data.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.eval(
        r#"
        var counter = new Counter();

        counter.count();
        counter.count()

        counter.print();
    "#,
    );
}

#[test]
fn set_global_object_with_ctor_and_method_with_return_value() {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|mut user_data| {
            user_data.value = 1;
        })
        .with_method("count", |mut user_data| -> Result<Value> {
            user_data.value += 1;

            Ok(Value::from(user_data.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("print", |inv: Invocation| -> Result<Value> {
        for val in inv.args.iter() {
            match val.coerce_string() {
                Some(s) => println!("{:?}", s),
                None => eprintln!("Error unwrap value"),
            }
        }

        Ok(Value::from(true))
    });

    rubtle.set_global_function("assert", |inv: Invocation| -> Result<Value> {
        let assert_val = inv.args.first().unwrap().as_boolean().unwrap();

        if true != assert_val {
            eprintln!("Assertion failed: {}", inv.args.last().unwrap().coerce_string().unwrap());
        }

        assert_eq!(true, assert_val);

        /* Make compiler happy */
        Ok(Value::from(true))
    });

    rubtle.eval(
        r#"
        var counter = new Counter();

        counter.count();

        var value = counter.count();

        print(value);

        assert(3 == value, "Damn!");
    "#,
    );
}