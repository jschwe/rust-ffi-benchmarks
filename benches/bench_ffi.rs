use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut g_empty_fn = c.benchmark_group("empty_func");
    g_empty_fn.bench_function("Rust-Rust", |b| {
        b.iter(|| {
            ffi_benchmark::rust::empty_func();
        })
    });
    g_empty_fn.bench_function("Rust-C++", |b| {
        b.iter(|| {
            ffi_benchmark::cxx_ffi::EmptyFunc();
        })
    });
    g_empty_fn.bench_function("Rust-C", |b| {
        b.iter(|| {
            unsafe { ffi_benchmark::c_ffi::EmptyFuncV2() };
        })
    });
    g_empty_fn.finish();

    let mut g_add_1 = c.benchmark_group("SingleAdd");
    let base_add: i32 = 555;
    g_add_1.bench_with_input("Rust-Rust", &base_add, |b, base| {
        b.iter(|| {
            ffi_benchmark::rust::add1(*base);
        })
    });
    g_add_1.bench_with_input("Rust-C++", &base_add, |b, base| {
        b.iter(|| {
            ffi_benchmark::cxx_ffi::add1(*base);
        })
    });
    g_add_1.bench_with_input("Rust-C", &base_add, |b, base| {
        b.iter(|| {
            unsafe { ffi_benchmark::c_ffi::add1_v2(*base) };
        })
    });

    g_add_1.finish();

    let mut g_sum_vec = c.benchmark_group("SliceAdd1000");
    let vec: Vec<u32> = (1000..2000).collect();
    g_sum_vec.bench_with_input("Rust-Rust", &vec, |b, v| {
        b.iter(|| {
            ffi_benchmark::rust::sum_slice(v);
        })
    });
    g_sum_vec.bench_with_input("Rust-C++", &vec, |b, v| {
        b.iter(|| {
            ffi_benchmark::cxx_ffi::SumSlice(v);
        })
    });
    g_sum_vec.finish();

    let mut g_sum_vec2 = c.benchmark_group("SliceAdd100_000");

    let vec: Vec<u32> = (0..100_000).collect();
    g_sum_vec2.bench_with_input("Rust-Rust", &vec, |b, v| {
        b.iter(|| {
            ffi_benchmark::rust::sum_slice(v);
        })
    });
    g_sum_vec2.bench_with_input("Rust-C++", &vec, |b, v| {
        b.iter(|| {
            ffi_benchmark::cxx_ffi::SumSlice(v);
        })
    });
    g_sum_vec2.finish();
}

criterion_group!(ffi, benchmark);
criterion_main!(ffi);
