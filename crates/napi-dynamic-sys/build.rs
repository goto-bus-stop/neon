use std::env;
use std::path::PathBuf;
use bindgen::EnumVariation;

pub fn main() {
    let bindings = bindgen::Builder::new()
        .header("include/node_api.h")
        .dynamic_loading(true)
        .dynamic_library_name("NodeApi")
        .whitelist_function("napi_.*")
        .whitelist_type("napi_.*")
        .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
        .generate()
        .unwrap();


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
