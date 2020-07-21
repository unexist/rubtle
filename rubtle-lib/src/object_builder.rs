use std::collections::HashMap;
///
/// @package Rubtle-Lib
///
/// @file ObjectBuilder functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//

type ObjectBuilderCall<T> = Box<dyn FnMut(T)>;

pub struct ObjectBuilder<T> {
    pub user_data: Option<T>,
    pub constructor: Option<ObjectBuilderCall<T>>,
    pub methods: HashMap<&'static str, ObjectBuilderCall<T>>,
}

impl<T> ObjectBuilder<T> {
    pub fn new() -> ObjectBuilder<T> {
        ObjectBuilder {
            user_data: None,
            constructor: None,
            methods: HashMap::new(),
        }
    }

    pub fn set_constructor<F>(&mut self, func: F)
    where
        F: 'static + FnMut(T),
    {
        self.constructor = Some(Box::new(func) as ObjectBuilderCall<T>);
    }

    pub fn set_user_data(&mut self, user_data: T) {
        self.user_data = Some(user_data);
    }

    pub fn add_method<F>(&mut self, name: &'static str, func: F)
    where
        F: 'static + FnMut(T),
    {
        self.methods.insert(name, Box::new(func) as ObjectBuilderCall<T>);
    }
}
