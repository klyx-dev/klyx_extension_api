# Klyx Extension API

A Rust-based API for building extensions for the Klyx platform. This API provides a WebAssembly-compatible interface for creating custom extensions that can interact with Android functionality and UI components.

## Overview

The Klyx Extension API allows developers to create extensions using Rust that compile to WebAssembly (WASM). Extensions can leverage platform-specific features like Android toast notifications and UI components while maintaining cross-platform compatibility.

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- `wasm32-unknown-unknown` target installed
- Cargo

### Installing the WASM Target

```bash
rustup target add wasm32-unknown-unknown
```

### Creating an Extension

1. Create a new Rust project:
```bash
cargo new --lib my_klyx_extension
cd my_klyx_extension
```

2. Add the Klyx Extension API to your `Cargo.toml`:
```toml
[package]
name = "my_klyx_extension"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
klyx_extension_api = "1.1.0"
```

3. Implement your extension in `src/lib.rs`:
```rust
#![no_std]

use klyx_extension_api::{Extension, register_extension, android::show_toast};

pub struct MyExtension;

impl Extension for MyExtension {
    fn init() {
        show_toast("Hello from my Klyx extension!");
    }
}

register_extension!(MyExtension);
```

4. Build your extension:
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Building Extensions

### Development Build
```bash
cargo build --target wasm32-unknown-unknown
```

### Release Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be located at:
```
target/wasm32-unknown-unknown/release/your_extension_name.wasm
```

## Example Extension

Here's a complete example of a simple extension:

```rust
#![no_std]

use klyx_extension_api::{Extension, register_extension, android::show_toast};

pub struct WelcomeExtension;

impl Extension for WelcomeExtension {
    fn init() {
        show_toast("Welcome to Klyx!");
    }
}

register_extension!(WelcomeExtension);
```

## Best Practices

1. **Keep extensions lightweight** - Extensions run in a constrained WASM environment
2. **Handle errors gracefully** - Always consider error cases in your extension logic
3. **Use `#![no_std]`** - The API is designed for `no_std` environments
4. **Test thoroughly** - Test your extension on target devices before distribution

## Platform Support

- **Android**: Full support with native integration
- **Other platforms**: Basic WASM runtime support

## Architecture

Extensions are compiled to WebAssembly and loaded by the Klyx runtime. The API provides:

- **Safe bindings** to platform-specific functionality
- **Cross-platform compatibility** through WebAssembly
- **Minimal overhead** with `no_std` design
- **Type safety** through Rust's type system

## Limitations

- Extensions run in a sandboxed WASM environment
- Limited access to system resources
- No direct file system access
- Network requests must go through provided APIs

## Contributing

Contributions to the Klyx Extension API are welcome! Please ensure your code:

- Follows Rust best practices
- Maintains `no_std` compatibility
- Includes appropriate documentation
- Passes all tests

## License

This project is licensed under the terms specified in the main Klyx project.

## Support

For questions, issues, or feature requests, please refer to the main Klyx project repository or documentation.