use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Navigate to the C library directory
    let crate_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = &format!("{}/depends/yespower", crate_dir);
    let lib_out_dir = &format!("{}/target/depends/{}/yespower", crate_dir, env::var("TARGET").unwrap());

    // Compile the library
    // Use a code block to avoid variable name pollution
    {
        let obj_ext_name = if cfg!(target_os = "windows") { "obj" } else { "o" };

        // Create the output directory if it doesn't exist
        let lib_out_dir_path = PathBuf::from(lib_out_dir);
        if !lib_out_dir_path.exists() {
            let _ = std::fs::create_dir_all(&lib_out_dir_path);
        }

        fn build_lib_to_obj(name: &str, crate_dir: &str, lib_dir: &str, lib_out_dir: &str, obj_ext_name: &str) {
            let output_name = &format!("{}/{}.{}", lib_out_dir, name, obj_ext_name);

            // Return if output file exists
            if PathBuf::from(output_name).exists() {
                return;
            }

            // Compile
            let status = Command::new("zig")
                .current_dir(crate_dir)
                .args([
                    "cc",
                    "-c",
                    "-O3",
                    "-fomit-frame-pointer",
                    "-funroll-loops",
                    &format!("{}/{}.c", lib_dir, name),
                    "-o",
                    output_name,
                ])
                .status()
                .expect("failed to execute process");

            if !status.success() {
                eprintln!("Failed to compile {}.c", name);
                std::process::exit(1);
            }
        }

        // Compile the necessary components
        build_lib_to_obj("yespower-opt", crate_dir, lib_dir, lib_out_dir, obj_ext_name);
        build_lib_to_obj("sha256", crate_dir, lib_dir, lib_out_dir, obj_ext_name);

        let output_lib_name = &match env::consts::OS {
            "linux" => format!("{}/libyespower.a", lib_out_dir),
            "windows" => format!("{}/yespower.lib", lib_out_dir),
            _ => panic!("Unsupported operating system"),
        };

        // Only link a library if lib file doesn't exist
        if !PathBuf::from(output_lib_name).exists() {
            let status = Command::new("zig")
                .current_dir(lib_dir)
                .args([
                    "ar",
                    "rcs",
                    output_lib_name,
                    &format!("{}/{}.{}", lib_out_dir, "yespower-opt", obj_ext_name),
                    &format!("{}/{}.{}", lib_out_dir, "sha256", obj_ext_name),
                ])
                .status()
                .expect("failed to link libs");

            if !status.success() {
                eprintln!("Failed to create static library yespower.lib");
                std::process::exit(1);
            }
        }
    }

    // Link the compiled library
    println!("cargo:rustc-link-search=native={}", lib_out_dir);
    println!("cargo:rustc-link-lib=static=yespower");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{}/yespower.h", lib_dir))
        .clang_arg(format!("-I{}", lib_out_dir))
        .generate()
        .expect("Failed to generate bindings.");

    // Write the bindings to a file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}