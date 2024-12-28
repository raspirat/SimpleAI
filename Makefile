# Define constants for different build targets
CARGO := cargo

TAURI_TARGET = target/release/bundle
WEB_TARGET = target/wasm32-unknown-unknown/release

# Build all targets
all: build-web build-desktop

# Build the project for WebAssembly
build-web:
	@echo "Building for WebAssembly..."
	$(CARGO) build --release --target wasm32-unknown-unknown --features "web"
	@wasm-bindgen --out-dir $(WEB_TARGET) --target web $(WEB_TARGET)/my_dioxus_app.wasm

# Build the project for Desktop (Tauri)
build-desktop:
	@echo "Building for Desktop..."
	$(CARGO) tauri build

# Run the web version
run-web: build-web
	@echo "Running Web version..."
	python3 -m http.server 8000 --directory $(WEB_TARGET)

# Run the desktop version
run-desktop: build-desktop
	@echo "Running Desktop version..."
	$(TAURI_TARGET)/my_dioxus_app

# Clean the project
clean:
	@echo "Cleaning the project..."
	$(CARGO) clean
	rm -rf $(WEB_TARGET)

.PHONY: all build-web build-desktop run-web run-desktop clean