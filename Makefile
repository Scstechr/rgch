build:
	cargo fmt --version;
	cargo fmt;
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -h;

c:
	cargo fmt --version;
	cargo fmt;
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -c;

up:
	rgch -c;
	python update.py >| src/version.rs
	cargo fmt --version;
	cargo fmt;
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	rgch -vp

update:
	rgch --pull
	rgch -h
