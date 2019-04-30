use bindgen;
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::io::{prelude::*, Seek, SeekFrom};

fn main() {
    // Tell cargo to tell rustc to link the vJoyInterface
    // library.
    println!("cargo:rustc-link-lib=vJoyInterface");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("vjoy-headers/bindgen/vjoy.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("vjoy_bindgen.rs"))
        .expect("Couldn't write bindings!");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("src/vjoy_bindgen.rs").expect("open fail");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read fail");

    let new_contents = format!(
        "#![allow(dead_code)]\n#![allow(non_snake_case)]\n#![allow(non_camel_case_types)]\n\n{}",
        contents
    );
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(new_contents.as_bytes()).expect("write fail");
}
