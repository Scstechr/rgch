build:
	cargo fmt --version;
	cargo fmt;
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	./target/release/rgch -h;

c:
	cargo fmt --version;
	cargo fmt;
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	./target/release/rgch -c;

up:
	rgch -c;
	python update.py >| src/version.rs
	cargo fmt --version;
	cargo fmt;
	cargo clippy --version;
	cargo clippy --release;
	cargo build --release;
	./target/release/rgch -vp
	rgch -c "update version"
	cargo publish
	cargo install rgch

    

update:
	./target/release/rgch --pull
	./target/release/rgch -h

build-musl:
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder cargo build --release

check:
	cargo fmt
	cargo check --release

build-musl-in:
	mv target/release .
	mv target/x86_64-unknown-linux-musl/release target/
	cargo build --release
	mv target/release target/x86_64-unknown-linux-musl/
	mv ./release target/
