fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return; // don't try to find libmapper on docs.rs
    }

    if cfg!(windows) {
        println!("cargo:rustc-link-lib=libmapper");
        println!("cargo:rustc-link-lin=liblo");
    } else {
        pkg_config::Config::new()
            .atleast_version("2.4.9")
            .probe("libmapper")
            .unwrap();
    }
}