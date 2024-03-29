# Cross-Compiling for Windows from Linux or macOS
.PHONY: install-windows-target
install-windows-target:
	rustup target add x86_64-pc-windows-gnu

# Compile for Windows
.PHONY: build-windows
build-windows:
	cargo build --release --target x86_64-pc-windows-gnu
	