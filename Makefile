CARGO := cargo
DIOXUS := dx

APP_NAME = SimpleAIEditor
TAURI_TARGET = target/release/bundle
WEB_TARGET = target/wasm32-unknown-unknown/release
DESKTOP_TARGET = target/release

APP_PATH = $(DESKTOP_TARGET)/$(APP_NAME)


all: build-web build-desktop

build-web:
	@echo "Building for WebAssembly..."
	$(CARGO) build --release --target wasm32-unknown-unknown --features "web"

build-desktop:
	@echo "Building for Desktop..."
	$(CARGO) build --release --features "desktop"

run-web:
	@echo "Running Web version..."
	$(DIOXUS) serve --platform web --features "web"

run-desktop: build-desktop
	@echo "Running Desktop version..."
	exec $(APP_PATH)

clean:
	@echo "Cleaning the project..."
	$(CARGO) clean
	rm -rf $(WEB_TARGET)

.PHONY: all build-web build-desktop run-web run-desktop clean
