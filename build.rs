// extern crate cmake;
//
// use cmake::Config;

use std::path::Path;

fn main() {
    // let dst = Config::new("crand").build();
    // println!("cargo:rustc-link-search=native={}", dst.display());
    // println!("cargo:rustc-link-lib=static=crand");
    // cc::Build::new()
    //     .file("crand/crand.c")
    //     .compiler(Path::new(r"C:\MinGW\bin\gcc.exe"))
    //     .compile("crand");
    let p = r"C:\Users\Gerrit\OneDrive - Fachhochschule Aachen\Module\Kryptologie\Dev\rust_aribas_random\crand\";
    println!("cargo:rustc-link-search={}", p);
    println!("cargo:rustc-link-lib=static=crand");
}