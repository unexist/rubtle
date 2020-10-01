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

use crate::{Result, Value, ObjectBuilder};

#[derive(Default)]
struct UserData {
    value: i32,
}

#[test]
fn create_object_builder() {
    let _object = ObjectBuilder::<UserData>::new().build();
}

#[test]
fn create_object_builder_object_has_method() {
    let object = ObjectBuilder::<UserData>::new().build();

    assert_eq!(object.has_method("test"), false);
}

#[test]
fn create_object_builder_object_get_method() {
    let mut object = ObjectBuilder::<UserData>::new().build();

    assert!(object.take_method("test").is_none());
}

#[test]
fn create_object_builder_object_iter() {
    let object = ObjectBuilder::<UserData>::new()
        .with_method("print1", |user_data| -> Result<Value> {
            println!("{}", user_data.value);

            Ok(Value::from(user_data.value))
        })
        .with_method("print2", |user_data| -> Result<Value> {
            println!("{}", user_data.value);

            Ok(Value::from(user_data.value))
        })
        .build();

    let mut i = 0;

    for (name, _meth) in object {
        i += 1;
        println!("{}", name);
    }

    assert_eq!(i, 2);
}

#[test]
fn create_builder_with_ctor() {
    let _object = ObjectBuilder::<UserData>::new()
        .with_constructor(|mut user_data| {
            user_data.value = 1;
        })
        .build();
}

#[test]
fn create_builder_with_method() {
    let _object = ObjectBuilder::<UserData>::new()
        .with_constructor(|mut user_data| {
            user_data.value = 1;
        })
        .with_method("increment", |mut user_data| -> Result<Value> {
            user_data.value += 1;

            Ok(Value::from(user_data.value))
        })
        .build();
}
