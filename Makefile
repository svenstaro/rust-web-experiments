default: build

.PHONY: build
build:
	cargo build --target=asmjs-unknown-emscripten
	cargo build --target=wasm32-unknown-emscripten

.PHONY: run
run: build
	mkdir -p data
	cp target/asmjs-unknown-emscripten/debug/rust-web-experiments.* data/
	cd data && python -m http.server
