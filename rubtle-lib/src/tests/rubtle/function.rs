///
/// @package Rubtle-Lib
///
/// @file Rubtle tests - function
/// @copyright 2020 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
use crate::{Rubtle, Value, CallbackResult};

use crate::tests::rubtle::helper::js_printer;

///
/// Global functions
///

#[test]
fn set_global_function_as_closure() {
    let rubtle = Rubtle::new();

    rubtle.set_global_function("square", |inv| -> CallbackResult<Value> {
        let args = inv.args.unwrap();

        let i = args.first().unwrap().as_number().unwrap();

        Ok(Value::from(i * i))
    });
}

#[test]
fn set_and_run_global_printer() {
    let rubtle = Rubtle::new();

    rubtle.set_global_function("print", js_printer);

    rubtle.eval(
        r#"
        print('Test');
    "#,
    );
}