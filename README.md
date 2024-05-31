# FFI benchmarks

This is a small benchmark project, that can give you some numbers on the overhead in
various different FFI scenarios when you are calling C or C++ code.

You can also follow the instructions to setup `-Clinker-plugin-lto`, which can effectively
eliminate a lot of the overhead in many scenarios.


### Linker plugin based Link-time-optmizations (LTO)

The rustc documentation has an informative section on [Linker plugin based LTO][linker-plugin-lto].
The main requirements are:
- The C/C++ compiler is clang, with a version matching the LLVM version of Rust. You can compare them by running
  `clang --version` and `rustc --version --verbose`.
- The C/C++ and Rust code are both compiled with LTO on (`-flto` )
- The rustcode is compiled with the rustflag `-Clinker-plugin-lto` set. Please note, that you will need to
  explicitly specify the `--target` for Rust if you use the RUSTFLAGS environment variable,
  otherwise the rustflags will also apply to build scripts and cause build failures.

You can check `run_benchmark.sh` for an example setup that enables linker plugin lto.

[linker-plugin-lto]: https://doc.rust-lang.org/rustc/linker-plugin-lto.html