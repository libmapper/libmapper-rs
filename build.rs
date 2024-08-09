fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return; // don't try to find libmapper on docs.rs
    }

    pkg_config::Config::new()
        .atleast_version("2.4.9")
        .probe("libmapper")
        .unwrap();
}