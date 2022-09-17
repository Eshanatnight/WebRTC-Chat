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

2. Install trunk ``cargo install trunk``

To run it once build you need to type

1. Compile code with ``trunk build --release``

2. Run web server in the ``trunk serve``

Or

Simply run [scripts/build.bat](scripts/build.bat)

## Special thanks

* [Sajad Hashemian](https://github.com/sajadhsm?tab=repositories) for making this [nice HTML chat](https://codepen.io/sajadhsm/pen/odaBdd)
