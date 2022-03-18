#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use console_error_panic_hook::set_once as set_panic_hook;
use serde_derive::{Deserialize, Serialize};
use web_sys::{window};
use log::*;
pub mod utils;
pub mod config;
pub mod auth;
pub mod user_model;
mod login;
mod token_model;
mod dashboard;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppState {
    Login,
    Dashboard
}


fn start_app() {
    let document = window().and_then(|win| win.document()).expect("Could not access document");
    let body = document.body().expect("Could not access document.body");

    let text_node = document.create_text_node("Login");
    body.append_child(text_node.as_ref()).expect("Failed to append text");

    let div_login = document.create_element("div");
    div_login.as_ref().expect("").set_id("div-login");
    body.append_child(div_login.as_ref().expect("")).expect("Failed to append el");
    login::setup_login(document);
}

// #[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
// extern "C" {
//     fn snippetTest();
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // set_panic_hook();
    // snippetTest();
    info!("start app");


    let mut state = AppState::Login;
    let auth_verify = login::verify_login();
    
    if auth_verify {
        state = AppState::Dashboard
    }

    match state {
        AppState::Login =>  start_app(),
        AppState::Dashboard => dashboard::request()
    }
}