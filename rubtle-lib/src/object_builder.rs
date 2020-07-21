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
    pub constructor: Option<ObjectBuilderCall<T>>,
    pub methods: HashMap<&'static str, ObjectBuilderCall<T>>,
}

impl<T> ObjectBuilder<T>
where
    T: Default + 'static,
{
    pub fn new() -> ObjectBuilder<T> {
        ObjectBuilder {
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

    pub fn call_constructor(&mut self, user_data: T) {
        match &mut self.constructor {
            Some(ctor) => (*ctor)(user_data),
            None => unimplemented!(),
        }
    }

    pub fn set_method<F>(&mut self, name: &'static str, func: F)
    where
        F: 'static + FnMut(T),
    {
        self.methods
            .insert(name, Box::new(func) as ObjectBuilderCall<T>);
    }
}
