extern crate bindgen;
extern crate make_cmd;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use make_cmd::make;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_dir = env::var("OUT_DIR").unwrap();

    let libucl_dir = Path::new(&manifest_dir)
        .join("deps")
        .join("libucl");

    let header_dir = Path::new(&libucl_dir).join("include");

    let status = Command::new("./autogen.sh").current_dir(&libucl_dir).status();

    println!("Status from autogen.sh: {:?}", status);

    let status = Command::new("./configure")
        .current_dir(&libucl_dir)
        .arg(&format!("--prefix={}", Path::new(&output_dir).display()))
        .arg("--enable-urls")
        .arg("--enable-regex")
        .arg("--disable-shared")
        .arg("--disable-dependency-tracking")
        .arg("--with-pic")
        .status();

    println!("Status from configure: {:?}", status);

    let status = make()
        .current_dir(&libucl_dir)
        .arg("-j")
        .arg(env::var("NUM_JOBS").unwrap())
        .arg("install")
        .status();

    println!("Status from make: {:?}", status);

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header(Path::new(&header_dir).join("ucl.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}",
             Path::new(&output_dir).join("lib").display());
    println!("cargo:rustc-link-lib=static=ucl");
}
