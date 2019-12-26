main:
	rgch --commit -c --push -p

build:
	cargo fmt --version
	cargo fmt -- --check
	cargo clippy --version
	cargo clippy --release
	cargo clean
	cargo build --release
