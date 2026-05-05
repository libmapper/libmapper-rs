use std::{env, path::PathBuf};

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return; // don't try to find libmapper on docs.rs
    }

    if cfg!(windows) {
        println!("cargo:rustc-link-lib=libmapper");
        println!("cargo:rustc-link-lib=liblo");
    } else {
        let library = pkg_config::Config::new()
            .atleast_version("2.4.9")
            .probe("libmapper")
            .unwrap();

        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .clang_args(library.include_paths.iter().map(|x| format!("-I{}", x.to_str().unwrap())))
            .rustified_enum("mpr.*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

            // Write the bindings to the $OUT_DIR/bindings.rs file.
            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
    }
}