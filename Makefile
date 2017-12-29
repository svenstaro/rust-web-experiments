default: build

.PHONY: build
build:
	cargo web build --use-system-emscripten --target-webasm-emscripten

.PHONY: run
run: build
	cargo web start --use-system-emscripten --target-webasm-emscripten
