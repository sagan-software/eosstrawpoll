extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //println!("cargo:rustc-link-lib=bz2");
    let contracts_dir = env::current_dir()
        .unwrap()
        .join("../extern/eos/contracts")
        .canonicalize()
        .unwrap();

    let bits_dir = contracts_dir
        .join("musl/upstream/include")
        .canonicalize()
        .unwrap();

    // println!("cargo:rustc-link-search={}", contracts_dir.display());
    // println!("cargo:include={}", contracts_dir.display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg(format!("-I{}", contracts_dir.display()).as_str())
        .clang_arg(format!("-I{}", bits_dir.display()).as_str())
        .clang_arg("-std=c++14")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // println!("{}", out_path.display());
}
