#![recursion_limit = "128000"]
#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;
mod chat;
use chat::chat_model::*;
use crate::chat::web_rtc_manager::WebRTCManager;
use yew::App;


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue>
{
    // use web-sys's global window function
    // to get a handle on the global window
    let window = web_sys::window().expect("no global \'window\' exists");
    let document =  window.document().expect("should have a document on the window");
    //let body = document.body().expect("should have a body");

    yew::initialize();
    let div = document.query_selector("#MyRustApp").unwrap().unwrap();
    App::<ChatModel<WebRTCManager>>::new().mount(div);
    yew::run_loop();

    Ok(())
}