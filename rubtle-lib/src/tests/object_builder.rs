///
/// @package Rubtle-Lib
///
/// @file Value tests
/// @copyright 2020-2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::{Value, ObjectBuilder};
use crate::types::CallbackResult;

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
fn create_object_builder_object_take_method() {
    let mut object = ObjectBuilder::<UserData>::new().build();

    assert!(object.take_method("test").is_none());
}

#[test]
fn create_object_builder_object_take_ctor() {
    let mut object = ObjectBuilder::<UserData>::new().build();

    assert!(object.take_constructor().is_none());
}

#[test]
fn create_object_builder_object_iter() {
    let object = ObjectBuilder::<UserData>::new()
        .with_method("print1", |inv| -> CallbackResult<Value> {
            let udata = inv.udata.as_ref().unwrap();

            Ok(Value::from(udata.value))
        })
        .with_method("print2", |inv| -> CallbackResult<Value> {
            let udata = inv.udata.as_ref().unwrap();

            Ok(Value::from(udata.value))
        })
        .build();

    let mut i = 0;

    for (_name, _meth) in object {
        i += 1;
    }

    assert_eq!(i, 2);
}

#[test]
fn create_builder_with_ctor() {
    let _object = ObjectBuilder::<UserData>::new()
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .build();
}

#[test]
fn create_builder_with_method() {
    let _object = ObjectBuilder::<UserData>::new()
        .with_constructor(|inv| {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value = 1;
        })
        .with_method("increment", |inv| -> CallbackResult<Value> {
            let mut udata = inv.udata.as_mut().unwrap();

            udata.value += 1;

            Ok(Value::from(udata.value))
        })
        .build();
}
