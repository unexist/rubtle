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
use crate::ObjectBuilder;

#[derive(Default, Copy, Clone)]
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

    assert!(object.get_method("test").is_none());
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
        .with_method("increment", |mut user_data| {
            user_data.value += 1;
        })
        .build();
}
