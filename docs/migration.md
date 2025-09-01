# Swift to Rust Migration Guide

This document tracks the ongoing effort to port Mist's Swift codebase to Rust. It outlines module mappings, required tooling, build steps for supported platforms, and a checklist to monitor progress.

## Module Mapping

| Swift Module | Rust Crate/Component | Status | Outstanding Work |
|--------------|---------------------|--------|------------------|
| Mist (macOS app) | `mist-rs` CLI application | In progress | Port remaining app features and integrate with GUI where needed |
| Shared utilities | `mist-core` library | In progress | Translate remaining helpers, installers, and model logic |
| MistHelperTool | *(TBD)* | Not started | Reimplement helper tool functionality in Rust |

## Required Tools

- [`rustup`](https://rustup.rs/) to manage Rust toolchains.
- [`cross`](https://github.com/cross-rs/cross) for cross-compiling to other targets.
- Platform SDKs:
  - **Windows**: Visual Studio Build Tools (MSVC) with the "Desktop development with C++" workload.
  - **Linux**: A C toolchain such as `gcc` or `clang` along with standard build utilities (`build-essential` on Debian/Ubuntu).
  - **macOS**: Xcode Command Line Tools installed via `xcode-select --install`.

Install `cross` after setting up `rustup`:

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

## Build Instructions

### Windows

1. Install `rustup` and the MSVC toolchain.
2. Install Visual Studio Build Tools with C++ components.
3. Install `cross` using the command above.
4. Clone the repository:
   ```bash
   git clone https://github.com/OWNER/Mist-rs.git
   cd Mist-rs
   ```
5. Build the workspace:
   ```bash
   cargo build --release
   ```

### Linux

1. Install `rustup` and the desired toolchain.
2. Install build dependencies (`sudo apt-get install build-essential` on Debian/Ubuntu).
3. Install `cross` using the command above.
4. Clone the repository and build:
   ```bash
   git clone https://github.com/OWNER/Mist-rs.git
   cd Mist-rs
   cargo build --release
   ```

### macOS

1. Install `rustup` and the default toolchain.
2. Install Xcode Command Line Tools: `xcode-select --install`.
3. Install `cross` using the command above.
4. Clone the repository and build:
   ```bash
   git clone https://github.com/OWNER/Mist-rs.git
   cd Mist-rs
   cargo build --release
   ```

## Progress Checklist

- [ ] Port `Shared` module to `mist-core`.
- [ ] Port `Mist` application to Rust.
- [ ] Reimplement `MistHelperTool` in Rust.

