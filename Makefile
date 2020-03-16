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

build-musl:
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder cargo build --release

check:
	cargo fmt
	cargo check --release

build-musl-in:
	cp -rf target/release release 
	cargo build --release
	mv target/release/rgch target/x86_64-unknown-linux-musl/release/
	mv ./release target/
