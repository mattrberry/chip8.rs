.DEFAULT_GOAL := native
WASM_TARGET := wasm32-unknown-emscripten

native:
	cargo build

wasm: export EMMAKEN_CFLAGS = -s USE_SDL=2 -s "EXTRA_EXPORTED_RUNTIME_METHODS=['ccall', 'cwrap']" -s EXPORT_ALL=1
wasm:
	cargo build --target=$(WASM_TARGET)
	cp target/$(WASM_TARGET)/debug/chip8.js static/
	cp target/$(WASM_TARGET)/debug/chip8.wasm static/

clean:
	rm -rf target
	rm -f static/chip8.js
	rm -f static/chip8.wasm

.PHONY: native wasm clean
