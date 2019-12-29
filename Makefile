build:
	cargo fmt --version;
	cargo fmt 
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -h
	rgch

c:
	cargo fmt --version;
	cargo fmt 
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -c

up:
	cargo fmt --version;
	cargo fmt 
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -cp

update:
	rgch --pull
	rgch -h
