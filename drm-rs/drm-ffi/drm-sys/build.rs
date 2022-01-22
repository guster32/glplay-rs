extern crate bindgen;

use std::env;
use std::path::PathBuf;

// use self::bindgen::{Builder, CodegenConfig};
// let config = CodegenConfig::all();
// Builder::default()
//     .clang_args("-I/drm/")
//     // The input header we would like to generate
//     // bindings for.
//     .header("wrapper.h")
//     .ctypes_prefix("libc")
//     .with_codegen_config(config)
//     .prepend_enum_name(false)
//     .layout_tests(false)
//     .generate_comments(false)
//     .rustfmt_bindings(true)
//     .derive_copy(true)
//     .derive_debug(true)
//     .derive_default(true)
//     .derive_hash(true)
//     .derive_eq(true)
//     .allowlist_recursively(true)
//     .use_core()

pub fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include/drm")
        .header("wrapper.h")
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}