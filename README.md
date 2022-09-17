# A WebRTC Chat with Yew

## About

After reading this [wonderful post](https://medium.com/leaningtech/porting-a-c-multiplayer-game-to-the-web-with-cheerp-webrtc-and-firebase-29fbbc62c5ca) by the [Cheerp team](https://leaningtech.com/pages/cheerp.html)
I wanted to see if the same can be done with "pure" Rust using WASM.

Since doing a game from scratch was too complicated as a first step I aimed to just play along with Rust, HTML (using [Yew](https://github.com/yewstack/yew)) and WebRTC by doing a chat application. 3D and WebGL will come another time.

![demo](demo.gif)

## Build and Run

To build this project you need to:

1. Install wasm-pack with ``cargo install wasm-pack``

2. Install http-server ``cargo install http-server``

To run it once build you need to type

1. Compile code with ``wasm-pack build --target web --no-typescript --out-dir ./static/pkg``

2. Run web server in the ``static`` dir with ``http-server -p 8080`` (Alternatively you can use any static file server like ``python3 -m http.server``)

## Special thanks

* The [Rust](https://www.rust-lang.org/) team for making a great language
* The [Yew](https://github.com/yewstack/yew) team for making an awesome front-end framework
* [Sajad Hashemian](https://github.com/sajadhsm?tab=repositories) for making this [nice HTML chat](https://codepen.io/sajadhsm/pen/odaBdd)
