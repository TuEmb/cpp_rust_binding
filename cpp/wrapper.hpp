#include <iostream>
#include <memory>
#include "base.hpp"

extern "C" {
    void rust_hello();  // Declaration of the Rust function

    class RustOverridden : public Base {
    public:
        void hello() const override {
            rust_hello();  // Call the Rust implementation
        }
    };

    std::unique_ptr<Base> create_rust_overridden() {
        std::cout << "Hello from C++!" << std::endl;
        return std::make_unique<RustOverridden>();
    }
}
