pub mod rust {
    #[inline(never)]
    pub fn empty_func() {}

    #[inline(never)]
    pub fn add1(val: i32) -> i32 {
        val + 1
    }

    #[inline(never)]
    pub fn sum_slice(v: &[u32]) -> u32 {
        v.iter().sum()
    }
}

#[cxx::bridge]
pub mod cxx_ffi {
    unsafe extern "C++" {
        include!("benchmark.h");

        fn EmptyFunc();
        fn add1(v: i32) -> i32;
        fn SumSlice(v: &[u32]) -> u32;

        // fn CreateCxxVector(len: usize) -> CxxVector;
        // fn SumCxxVector(v: &CxxVector<u32>) -> u32;
    }
}

pub mod c_ffi {
    extern "C" {
        pub fn EmptyFuncV2();

        pub fn add1_v2(v: i32) -> i32;
    }
}
