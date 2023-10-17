use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Navigate to the C library directory
    let crate_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = format!("{}/depends/yespower", crate_dir);
    let gcc = "gcc";

    // Compile the necessary components
    Command::new(gcc)
        .current_dir(&lib_dir)
        .args(&["-c", "-O2", "-fomit-frame-pointer", "-funroll-loops", "yespower-opt.c", "sha256.c"])
        .status()
        .unwrap();

    // Link the object files into a static library
    Command::new("ar")
        .current_dir(&lib_dir)
        .args(&["rcs", "libyespower.a", "yespower-opt.o", "sha256.o"])
        .status()
        .unwrap();

    // Link the compiled library
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=yespower");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{}/yespower.h", lib_dir))
        .clang_arg(format!("-I{}", lib_dir))
        .generate()
        .expect("Failed to generate bindings.");

    // Write the bindings to a file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}