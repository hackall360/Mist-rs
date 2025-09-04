# Rust Workspace

This repository includes a Rust workspace containing the `mist-rs` application and shared crates.

## Build prerequisites

To build the Rust components you'll need a recent [Rust toolchain](https://rustup.rs/) and platform-specific build tools:

- **Windows**: Install the MSVC toolchain. The easiest way is via `rustup` along with the "Desktop development with C++" workload from Visual Studio or the standalone [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio).
- **Linux**: A C toolchain such as `gcc` or `clang` and standard build utilities. On Debian-based distributions: `sudo apt-get install build-essential`.
- **macOS**: Install Xcode Command Line Tools with `xcode-select --install`.

After installing the prerequisites, you can verify the workspace builds with:

```bash
cargo check
```

## Cross-compilation

Additional targets can be installed with `rustup`:

```bash
rustup target add x86_64-unknown-linux-gnu \
    x86_64-pc-windows-msvc \
    aarch64-apple-darwin
```

For a consistent experience across platforms, install [`cross`](https://github.com/cross-rs/cross):

```bash
cargo install cross
cross test --target x86_64-pc-windows-msvc
cross build --release --target aarch64-apple-darwin
```

Windows builds require the Visual Studio Build Tools and macOS builds require
Xcode and its command line tools.
