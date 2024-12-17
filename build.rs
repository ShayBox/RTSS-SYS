use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let include = "C:\\Program Files (x86)\\RivaTuner Statistics Server\\SDK\\Include";
    let bindings = bindgen::Builder::default()
        .raw_line("#![allow(unused, non_camel_case_types, non_snake_case)]")
        .raw_line("#![allow(clippy::unreadable_literal, clippy::upper_case_acronyms)]")
        .header("wrapper.h")
        .clang_args(["-I", include])
        .clang_args(["-x", "c++"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(".*\\\\RTSSSharedMemory.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
