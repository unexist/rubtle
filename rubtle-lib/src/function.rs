///
/// @package Rubtle-Lib
///
/// @file Function functions
/// @copyright 2020 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::fmt;

use crate::{Value, Invocation};
use crate::types::{Callback, CallbackResult};

pub struct Function<T> {
    pub callback: Callback<T>,
}

impl<T> Function<T> {
    pub fn from<'a, F>(func: F) -> Function<T>
    where
        F: 'static + Fn(Invocation<T>) -> CallbackResult<Value>,
    {
        Function {
            callback: Box::new(func) as Callback<T>,
        }
    }
}

impl<T> fmt::Debug for Function<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "func")
    }
}

impl<T> Clone for Function<T> {
    fn clone(&self) -> Self {
        unimplemented!();
    }
}

impl<T> PartialEq for Function<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
