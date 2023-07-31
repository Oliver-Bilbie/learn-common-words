bootstrap:
	@curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
	@source "~/.cargo/env"
	@cargo install --locked trunk
	@cargo install --locked wasm-bindgen-cli

serve:
	@trunk serve

build:
	@echo "[INFO] Building frontend..."
	@trunk build --release
	@echo "[INFO] Build complete!"
