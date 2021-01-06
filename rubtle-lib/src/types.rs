///
/// @package Rubtle-Lib
///
/// @file Types
/// @copyright 2020-2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//
use std::result::Result;

use crate::{Error, Invocation, Value};

pub type CallbackResult<T> = Result<T, Error>;
pub type Callback<T> = Box<dyn Fn(Invocation<T>) -> CallbackResult<Value>>;

/* Special object builder types */
pub type ObjectBuilderCtor<T> = Box<dyn FnMut(&mut Invocation<T>)>;
pub type ObjectBuilderCallback<T> = Box<dyn FnMut(&mut Invocation<T>) -> CallbackResult<Value>>;