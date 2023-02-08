# rust_colony
A Dark Colony Web Clone 

## Install / check required tools
Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)
- [Bevy Setup](https://bevyengine.org/learn/book/getting-started/setup/)
- Add wasm target : `rustup target add wasm32-unknown-unknown`
- Install wasm-bingen : `cargo install wasm-bindgen-cli`

# Build 

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-name wasm --out-dir wasm/ --target web target/wasm32-unknown-unknown/release/rust_colony.wasm
