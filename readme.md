# Rust-Angband

A spiritual fork of the original C-based [Angband](https://angband.github.io/angband), version 4.2.3, re-implemented in Rust.

## Dependencies
This depends on OpenGL and GLFW directly, which are included in the root of this repo. GLFW depends on cmake and a C/C++ compiler - you'll probably have to install cmake from your favorite package manager (and possibly add it to your PATH after).

## Running without Cargo
Running from within cargo (`cargo run`) will take relative folder movement into account. If you want to run this locally after it's built, you'll need to copy `resources` to `target/{build}/resources`, right next to the `rust-angband.exe` you'll want to use.

There are future plans of integrating this into a build system (`cargo build` and `build.rs` are *not* meant for this), and also of packaging some of these into a binary format and providing others as run-time downloads via a service. This would provide a combination of known content and expandability.