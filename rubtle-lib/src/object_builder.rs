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
use std::collections::HashMap;

use crate::{Result, Value};
use crate::types::ObjectBuilderCall;

#[derive(Default)]
pub struct Object<T> {
    pub methods: HashMap<&'static str, ObjectBuilderCall<T>>,
}

impl<T> Object<T>
where
    T: Default + 'static,
{
    pub fn has_method(&self, meth_name: &str) -> bool {
        !self.methods.is_empty() && self.methods.contains_key(meth_name)
    }

    pub fn take_method(&mut self, meth_name: &str) -> Option<ObjectBuilderCall<T>> {
        self.methods.remove(meth_name)
    }
}

impl<T> Iterator for Object<T> {
    type Item = (&'static str, ObjectBuilderCall<T>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.methods.keys().last() {
            Some(&key) => Some((key, self.methods.remove(key).unwrap())),
            None => None,
        }
    }
}

pub struct ObjectBuilder<T> {
    methods: HashMap<&'static str, ObjectBuilderCall<T>>,
}

impl<T> ObjectBuilder<T>
where
    T: Default + 'static,
{
    pub fn new() -> ObjectBuilder<T> {
        ObjectBuilder::<T> {
            methods: HashMap::new(),
        }
    }

    pub fn with_constructor<'a, F>(&'a mut self, func: F) -> &'a mut ObjectBuilder<T>
    where
        F: 'static + FnMut(&mut T) -> Result<Value>,
    {
        self.methods
            .insert("ctor", Box::new(func) as ObjectBuilderCall<T>);

        self
    }

    pub fn with_method<'a, F>(&'a mut self, name: &'static str, func: F) -> &'a mut ObjectBuilder<T>
    where
        F: 'static + FnMut(&mut T) -> Result<Value>,
    {
        assert!("ctor" != name, "use with_constructor");

        self.methods
            .insert(name, Box::new(func) as ObjectBuilderCall<T>);

        self
    }

    pub fn build(&mut self) -> Object<T> {
        let mut object = Object::<T>::default();

        std::mem::swap(&mut self.methods, &mut object.methods);

        object
    }
}
