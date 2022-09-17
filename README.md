# A WebRTC Chat with Yew

## About

This is a Web-RTC chat app created in Rust.
I wanted to see and learn WASM.

![demo](demo.gif)

## Build and Run

To build this project you need to:

1. Add `wasm32-unknown-unknown` as a target

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

2. Install wasm-pack ``cargo install wasm-pack``

3. Install http-server ``npm install --global http-server``

4. Run by ``http-server -p 8080``

Or

Simply run [scripts/build.ps1](scripts/build.ps1)

## Special thanks

* [Sajad Hashemian](https://github.com/sajadhsm?tab=repositories) for making this [nice HTML chat](https://codepen.io/sajadhsm/pen/odaBdd)
