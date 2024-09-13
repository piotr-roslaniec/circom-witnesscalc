use std::io::Result;
use std::env;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/graph_witness.h")
        .allowlist_type("gw_status_t");

    if env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        builder = builder
            // Specify the include path for Clang
            .clang_arg("-I/usr/include")
            .clang_arg("-I/usr/include/x86_64-linux-gnu")
            .clang_arg("-I/usr/include/i386-linux-gnu");
    }

    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Generate protobuf bindings
    let empty_array: &[&Path] = &[];
    prost_build::compile_protos(&["protos/messages.proto"], empty_array)?;

    Ok(())
}