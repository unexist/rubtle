///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - basic
/// @copyright 2020-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::{Rubtle, Value, CallbackResult, ObjectBuilder};

use crate::tests::rubtle::helper::js_assert;
use crate::tests::rubtle::helper::js_printer;

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
        .with_method("inc", |inv| -> CallbackResult<Value> {
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
        .with_method("inc", |inv| -> CallbackResult<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .with_method("print", |inv| -> CallbackResult<Value> {
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
        var value = counter.inc();
        assert(3 == value, "Damn!");
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
        .with_method("inc", |inv| -> CallbackResult<Value> {
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
        .with_method("inc", |inv| -> CallbackResult<Value> {
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
        .with_method("inc", |inv| -> CallbackResult<Value> {
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