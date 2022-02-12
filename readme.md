# Rust/SDL/Emscripten Template

Rust/SDL2 project template that can compile for both native and web targets.

## How to run

### Web

1. [Install Emscripten](https://emscripten.org/docs/getting_started/downloads.html)
2. Activate Emscripten (`source ./emsdk_env.sh`)
3. Run `./dev_server` to compile and set up a little webserver.

It'll automatically recompile and host whenever you make changes in `./src`.

### Native

Run `cargo run`

## How to use

This project is designed so that you can clone this repo, delete `./src/game.rs`, tweak `./src/index.html`, and drop in your own game code.  Have fun!
