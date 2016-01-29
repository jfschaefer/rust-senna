//! We'll have to compile senna and our c_wrapper.c to create libc_wrapper.a
//! We assume that `sh`, `make`, `gcc` and `ar` are installed and that the senna directory
//! is located in the right place.
//! The actual compilation is managed by the make file `wrapperMakefile`


fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let pwd = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // store senna path
    std::process::Command::new("sh")
        .args(&["set_senna_path.sh", &format!("{}/senna/", pwd)])
        .status().unwrap();

    // build c stuff
    std::process::Command::new("make")
        .args(&["static_lib", "-f", "wrapperMakefile", &format!("OUT_DIR={}", out_dir), "SENNA_DIR=senna"])
        .status().unwrap();

    // `make clean` (only removes *.o files)
    std::process::Command::new("make")
        .args(&["clean", "-f", "wrapperMakefile", &format!("OUT_DIR={}", out_dir), "SENNA_DIR=senna"])
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=c_wrapper");
}

