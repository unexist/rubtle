///
/// @package Rubtle-Lib
///
/// @file Invocation functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//
use crate::{Rubtle, Value};

pub struct Invocation<'rubtle, T> {
    pub rubtle: &'rubtle Rubtle,
    pub args: Option<Vec<Value>>,
    pub udata: Option<T>,
}
