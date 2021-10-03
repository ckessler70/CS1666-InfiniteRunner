# CS1666-InfiniteRunner
Group project for CS1666, fall 2021

## Installing SDL2

Install SDL2 fully on your machine
1. **MacOS**
    1. Run `brew install gcc` 
    2. Run `brew install sdl2` 
    3. Run `brew install sdl2_image` 
    4. Run `brew install sdl2_mixer` 
    5. Run `brew install sdl2_ttf` 
    6. Add the following to your `~/.bash_profile`: `export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"`
    7. Run `source ~/.bash_profile`
    8. Run `rustup default nightly`
    9. Run `rustup update`
2. **Windows (assuming Rust installed through `rustup`)**
    1. Run `rustup default nightly`
    2. Run `rustup update`
    3. Download the [SDL2-devel-2.0.16-VC.zip](https://www.libsdl.org/download-2.0.php)
    4. Download the [SDL2_image-devel-2.0.5-VC.zip](https://www.libsdl.org/projects/SDL_image/)
    5. Download the [SDL2_mixer-devel-2.0.4-VC.zip](https://www.libsdl.org/projects/SDL_mixer/)
    6. Download the [SDL2_ttf-devel-2.0.15-VC.zip](https://www.libsdl.org/projects/SDL_ttf/)
    7. Locate your install of rustup. Mine was `C:\Users\{username}\.rustup`
    8. Navigate to roughly the following path: `C:\Users\{username}\.rustup\toolchains\{current_toolchain}\lib\rustlib\x86_64-pc-windows-msvc\lib` where `current_toolchain` will likely be the most recently modified folder with the name `nightly` in it
        1. I think the process is similar for those who have rust installed through different means. Basing off of the https://github.com/Rust-SDL2/rust-sdl2 repo, the folder path might be `C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-msvc\lib` though I cannot confirm.
    10. Add the path found in **viii** to your environment variables like so that the variable name is `LIBRARY_PATH`
    11. From each .zip, navigate roughly to `{file name}\lib\x64` and copy all contents into the path mentioned in **viii**
    12. Copy these files found within their respective .zips (`SDL2.dll`, `SDL2_image.dll`, `SDL2_mixer.dll`, and `SDL2_ttf.dll`) to your project folder placed in the same location as `Cargo.toml` (From what I can tell, this needs to be done every time you want to utilize SDL2)
3. **[Linux](https://github.com/Rust-SDL2/rust-sdl2#linux)**
    1. Furthering from the instructions found on the rust-sdl2 repo, you may need to install the following packages: `libsdl2-image-dev`, `libsdl2-mixer-dev`, and `libsdl2-ttf-dev`

## Building and Running 

1. `cargo build`
2. `cargo run --example ...`
