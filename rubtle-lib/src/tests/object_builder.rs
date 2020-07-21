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

#[test]
fn create_object() {
    let _builder: ObjectBuilder<Userdata> = ObjectBuilder::new();
}

struct Userdata {
    value: i32,
}

#[test]
fn create_builder_with_userdata() {
    let mut builder: ObjectBuilder<Userdata> = ObjectBuilder::new();

    let user_data = Userdata { value: 4 };

    builder.set_user_data(user_data);
}

#[test]
fn create_builder_with_ctor() {
    let mut builder: ObjectBuilder<Userdata> = ObjectBuilder::new();

    let user_data = Userdata { value: 4 };

    builder.set_user_data(user_data);
    builder.set_constructor(|mut user_data| {
        user_data.value += 1;
    });
}

#[test]
fn create_builder_with_method() {
    let mut builder: ObjectBuilder<Userdata> = ObjectBuilder::new();

    let user_data = Userdata { value: 4 };

    builder.set_user_data(user_data);
    builder.set_constructor(|mut user_data| {
        user_data.value = 1;
    });

    builder.add_method("increment", |mut user_data| {
        user_data.value += 1;
    });
}
