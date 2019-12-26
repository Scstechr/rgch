main:
	rgch --commit -c --push -p

build:
	cargo fmt --version
	cargo fmt -- --check
	cargo clippy --version
	cargo clippy
	cargo clean
	cargo build --release
