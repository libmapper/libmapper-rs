fn main() {
    pkg_config::Config::new()
        .atleast_version("2.4.6")
        .probe("libmapper")
        .unwrap();
}