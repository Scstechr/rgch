main:
	rgch --commit -c --push -p

build:
	cargo fmt --version;
	cargo fmt 
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
