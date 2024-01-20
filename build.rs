fn main() {
    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c++"])
        .header("cbits/wrapper.h")
        .generate()
        .expect("unable to generate djvulibre bindings");

    let out = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(&out);

    bindings.write_to_file(out_dir.join("bindings.rs")).unwrap();

    let dst = autotools::Config::new("cbits")
        .reconf("-ivf")
        .enable("static", None)
        .disable("shared", None)
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=thread");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-flags=-l dylib=c++");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-flags=-l dylib=stdc++");

    println!("cargo:rerun-if-changed=cbits/configure.ac");
    println!("cargo:rerun-if-changed=cbits/Makefile.am");
    println!("cargo:rerun-if-changed=cbits/Thread.h");
    println!("cargo:rerun-if-changed=cbits/Thread.cpp");
}
