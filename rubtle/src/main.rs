///
/// @package Rubtle
///
/// @file Rubtle main entry
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
extern crate rubtle_lib as rubtle;

use rubtle::Rubtle;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if 1 < args.len() {
        let contents = fs::read_to_string(&args[1]);

        println!("{:?}", contents);

        let _rubtle = Rubtle::new();
    } else {
        println!("Usage: {}: <JSFile>", args[0]);
    }
}
