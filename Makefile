start:
	cargo run

lint:
	cargo clippy

build:
	cargo build

release:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/rusty-roguelike.wasm --out-dir ./release --no-modules --no-typescript
