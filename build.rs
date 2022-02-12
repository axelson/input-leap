extern crate bindgen;
extern crate cbindgen;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    let mut config: cbindgen::Config = Default::default();

    config.language = cbindgen::Language::Cxx;

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("./target/input-leap-rust-bindings.h");
}
