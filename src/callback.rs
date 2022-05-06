#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-callback/include/callback.hpp");

        fn callme(cb: fn(i32) -> i32);
    }
}
