///
/// @package Rubtle-Duktape
///
/// @file Rubtle bindgen builder
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///
extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    /* Tell cargo to tell rustc to link the static library.*/
    println!("cargo:rustc-link-lib=static=duktape");

    /* Tell cargo to invalidate the built crate whenever the wrapper changes */
    println!("cargo:rerun-if-changed=duktape_wrapper.h");

    /* Try to fix llvm path for macos */
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=LIBCLANG_PATH=/usr/local/opt/llvm/bin");
        println!("cargo:rustc-env=LLVM_CONFIG_PATH=/usr/local/opt/llvm/bin/llvm-config");
    }

    /* Create bindings */
    let bindings = bindgen::Builder::default()
        .header("src/duktape_wrapper.h")
        .clang_arg("-Iduktape")
        .clang_arg("-std=c99")
        .trust_clang_mangling(false)
        .whitelist_type("^(?:rust_)?duk_.*")
        .whitelist_function("^(?:rust_)?duk_.*")
        .whitelist_var("^DUK_.*")
        .generate()
        .expect("Unable to generate bindings");

    /* Write the bindings to the $OUT_DIR/bindings.rs file. */
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    /* Build duktape */
    let mut builder = cc::Build::new();

    builder
        .include("duktape")
        .flag("-std=c99")
        .file("duktape/duktape.c");

    if cfg!(feature = "use-exec-timeout-check") {
        builder.define("RUST_DUK_USE_EXEC_TIMEOUT_CHECK", None);
    }

    builder.compile("libduktape.a");
}
