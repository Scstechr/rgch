build:
	cargo fmt --version;
	cargo fmt 
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -h

up:
	cargo fmt --version;
	cargo fmt 
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -cp
