@echo off
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk build --release
trunk serve