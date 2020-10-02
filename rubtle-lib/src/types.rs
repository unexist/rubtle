///
/// @package Rubtle-Lib
///
/// @file Types
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//
use std::result;

use crate::{Error, Invocation, Value};

pub type Result<T> = result::Result<T, Error>;

pub type Callback<T> = Box<dyn Fn(Invocation<T>) -> Result<Value>>;

/* Special object builder types */
pub type ObjectBuilderCtor<T> = Box<dyn FnMut(&mut Invocation<T>)>;
pub type ObjectBuilderCall<T> = Box<dyn FnMut(&mut Invocation<T>) -> Result<Value>>;