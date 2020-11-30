///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - eval
/// @copyright 2020 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::Rubtle;

///
/// Eval
///

#[test]
fn evil_eval_test() {
    let rubtle = Rubtle::new();

    rubtle.eval(
        r#"
        var rubtle = 'rubtle';
    "#,
    );
}