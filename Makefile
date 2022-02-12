web:
	EMSDK_PYTHON=/usr/bin/python3 EMMAKEN_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0 --no-entry" cargo build --target wasm32-unknown-emscripten --release

native:
	cargo build --release
