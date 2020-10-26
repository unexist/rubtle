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

///
/// Helper functions
///

fn js_printer(inv: Invocation<i8>) -> Result<Value> {
    let args = inv.args.unwrap();

    for val in args.iter() {
        match val.coerce_string() {
            Some(s) => println!("{:?}", s),
            None => eprintln!("Error unwrap value"),
        }
    }

    Ok(Value::from(true))
}

fn js_assert(inv: Invocation<i8>) -> Result<Value> {
    let args = inv.args.unwrap();
    let assert_val = args.first().unwrap().as_boolean().unwrap();
    let assert_mesg = args.last().unwrap().coerce_string().unwrap();

    assert_eq!(true, assert_val, "{}", assert_mesg);

    /* Make compiler happy */
    Ok(Value::from(true))
}

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

#[test]
fn push_and_pop_none_value() {
    let rubtle = Rubtle::new();
    let rval = Value::from(());

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

///
/// Global functions
///

#[test]
fn set_global_function_as_closure() {
    let rubtle = Rubtle::new();

    rubtle.set_global_function("square", |inv| -> Result<Value> {
        let args = inv.args.unwrap();

        let i = args.first().unwrap().as_number().unwrap();

        Ok(Value::from(i * i))
    });
}

#[test]
fn set_and_run_global_printer() {
    let rubtle = Rubtle::new();

    rubtle.set_global_function("print", js_printer);

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
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("assert", js_assert);

    rubtle.eval(
        r#"
        var counter = new Counter();

        assert(typeof counter != 'undefined', "Damn!");
    "#,
    );
}

#[test]
fn set_global_object_with_ctor_and_method() {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .with_method("inc", |inv| -> Result<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("assert", js_assert);

    rubtle.eval(
        r#"
        var counter = new Counter();

        assert(typeof counter != 'undefined', "Damn!");

        counter.inc();
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
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .with_method("inc", |inv| -> Result<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .with_method("print", |inv| -> Result<Value> {
            let udata = inv.udata.as_ref().unwrap();

            println!("Value={}", udata.value);

            Ok(Value::from(udata.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("assert", js_assert);

    rubtle.eval(
        r#"
        var counter = new Counter();

        assert(typeof counter != 'undefined', "Damn!");

        counter.inc();
        counter.inc();

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
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .with_method("inc", |inv| -> Result<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("print", js_printer);
    rubtle.set_global_function("assert", js_assert);

    rubtle.eval(
        r#"
        var counter = new Counter();

        assert(typeof counter != 'undefined', "Damn!");

        counter.inc();

        var value = counter.inc();

        print(value);

        assert(3 == value, "Damn!");
    "#,
    );
}

#[test]
fn set_global_object_with_ctor_with_arguments_and_method_with_return_value() {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();
            let args = inv.args.as_ref().unwrap();

            match args.first() {
                Some(val) => udata.value = val.as_number().unwrap() as i32,
                None => udata.value = 1,
            }
        })
        .with_method("inc", |inv| -> Result<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("assert", js_assert);

    rubtle.eval(
        r#"
        var counter = new Counter(5);

        assert(typeof counter != 'undefined', "Damn!");

        var value = counter.inc();

        assert(6 == value, "Damn!");
    "#,
    );
}

#[test]
fn set_global_object_with_ctor_with_arguments_and_method_with_arguments_and_return_value() {
    #[derive(Default)]
    struct UserData {
        value: i32,
    };

    let mut object = ObjectBuilder::<UserData>::new()
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();
            let args = inv.args.as_ref().unwrap();

            match args.first() {
                Some(val) => udata.value = val.as_number().unwrap() as i32,
                None => udata.value = 1,
            }
        })
        .with_method("inc", |inv| -> Result<Value> {
            let mut udata = inv.udata.as_mut().unwrap();
            let args = inv.args.as_ref().unwrap();

            match args.first() {
                Some(val) => udata.value += val.as_number().unwrap() as i32,
                None => udata.value += 1,
            }

            Ok(Value::from(udata.value))
        })
        .build();

    let rubtle = Rubtle::new();

    rubtle.set_global_object("Counter", &mut object);

    rubtle.set_global_function("assert", js_assert);

    rubtle.eval(
        r#"
        var counter = new Counter(2);

        assert(typeof counter != 'undefined', "Damn!");

        var value = counter.inc(8);

        assert(10 == value, "Damn!");
    "#,
    );
}
