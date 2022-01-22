# Rust-Angband

A spiritual fork of the original C-based [Angband](https://angband.github.io/angband), version 4.2.3, re-implemented in Rust.

## Dependencies
This depends on SDL2. A good way to get it in place is via [vcpkg](https://github.com/microsoft/vcpkg), installed globally (on the assumption that you're on Windows, like I am, and Rust is using its default of the msvc toolchain). In addition to putting SDL2 in your global `vcpkg` cache, you'll need to edit `build.rs` to point to that cache.

When putting this in place, I was also getting the original project to build on Windows via CMake - I'm not sure if it's required for this project, but at the same time I also added the SDL2 folders to my system environment variables (like stated on the [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc-with-vcpkg) GitHub page).