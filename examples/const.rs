#![cfg_attr(feature = "nightly", feature(const_trait_impl))]

nightly::nightly! {
    pub ++[const] trait Default {
        fn default() -> Self;
    }

    impl ++[const] Default for () {
        fn default() -> Self {}
    }

    pub struct Thing<T>(pub T);

    impl<T: ++[[const]] Default> ++[const] Default for Thing<T> {
        fn default() -> Self {
            Self(T::default())
        }
    }

    pub ++[const] fn default<T: ++[[const]] Default>() -> T {
        T::default()
    }

    #[allow(unused_braces)]
    pub fn compile_time_default<T: ++[const] Default>() -> T {
        ++[const] { T::default() }
    }
}

fn main() {
    let _a: () = default();
    let _b: Thing<()> = default();
    let _c: () = compile_time_default();
    let _d: Thing<()> = compile_time_default();
}
