use std::path::PathBuf;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = PathBuf::from(manifest_dir).join("src");
    assert!(src_dir.is_dir());
    let mut builder = cxx_build::bridge("src/lib.rs");
    builder
        .file("src/benchmark.cpp")
        .std("c++14")
        .include(src_dir);

    if std::env::var("HOST").expect("Cargo?") == std::env::var("TARGET").expect("Cargo?") {
        builder.compiler("clang-18");
    } else {
        // we are cross-compiling
    }

    let compiler = builder.get_compiler();
    if compiler.is_like_clang() {
        // todo: check clang llvm version matches rustc llvm version
        builder.flag("-flto=thin");
        // todo: sadly we can't set that here..
        // println!("cargo::rustc-flags=-Clinker-plugin-lto");
        println!("cargo::rustc-link-arg=-fuse-ld=lld");
    } else {
        panic!("Can't do LTO without clang...")
    }
    builder.compile("cxxbridge-demo");
}
