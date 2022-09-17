$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk build --release