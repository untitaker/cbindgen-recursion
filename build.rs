extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_parse_expand(&["cbindgen-recursion"])
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}
