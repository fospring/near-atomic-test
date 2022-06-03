rebuild:
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	mkdir -p res
	cp ./target/wasm32-unknown-unknown/release/near_atomic_test.wasm ./res/

