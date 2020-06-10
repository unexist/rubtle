extern crate rubtle_lib as rubtle;

use rubtle::Rubtle;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if 1 < args.len() {
        let contents = fs::read_to_string(&args[1]);

        println!("{:?}", contents);

        let rubtle = Rubtle::new();
    } else {
        println!("Usage: {}: <JSFile>", args[0]);
    }
}