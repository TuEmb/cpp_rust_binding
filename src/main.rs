#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wrapper.hpp");

        type Base;

        fn create_rust_overridden() -> UniquePtr<Base>;
        fn hello(&self); // Declare the hello method
    }
}

#[no_mangle]
pub extern "C" fn rust_hello() {
    println!("Hello from Rust!");
}

fn main() {
    let base = ffi::create_rust_overridden();
    base.hello();
}
