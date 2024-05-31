#!/usr/bin/env bash

if ! command -v cargo-criterion
then
    echo "cargo criterion not found - installing"
    cargo install cargo-criterion
fi

if ! command -v clang
then
    echo "clang not found, please install"
fi

if ! command -v lld
then
    echo "lld not found, please install"
fi

lld_version=$(ld.lld --version)
rust_llvm_version=$(rustc --version --verbose | grep "LLVM version" | sed -e "s/LLVM version: //")
clang_version=$(clang --version)

echo "LLD version:       ${lld_version}"
echo "Rust LLVM version: ${rust_llvm_version}"
echo "clang version:     ${clang_version}"
echo "Please manually check that at least the major, and ideally the minor version of clang, lld and rust match!"


echo "Running benchmark with linker-plugin lto"
RUSTFLAGS="-Clinker-plugin-lto" cargo criterion --target=x86_64-unknown-linux-gnu
