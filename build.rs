fn main() {
    cxx_build::bridge("src/main.rs")
        .file("cpp/base.cpp")
        .include("cpp")
        .std("c++14")
        .compile("cpp_rust_binding");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/base.cpp");
}