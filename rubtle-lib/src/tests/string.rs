///
/// @package Rubtle-Lib
///
/// @file String tests
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::Rubtle;

#[test]
fn push_and_pop_str() {
    let rubtle = Rubtle::new();

    rubtle.push_str("String Test");
    let cstring = rubtle.pop_str(0);

    println!("{}", cstring)
}

#[test]
fn evil_eval_test() {
    let rubtle = Rubtle::new();

    rubtle.eval(r#"
        let rublte = 'yeah';
    "#);
}