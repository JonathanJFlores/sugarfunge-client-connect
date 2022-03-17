use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo_storage::*;
use web_sys::{window, HtmlButtonElement, Event};
use log::*;
use crate::auth;


pub fn request() {
    let document = window().and_then(|win| win.document()).expect("Could not access document");
    let body = document.body().expect("Could not access document.body");

    let div_dashboard = document.create_element("div").expect("no div create");
    div_dashboard.set_id("div-dashboard");
    
    let logout_btn = document.create_element("button").expect("field no created").dyn_into::<HtmlButtonElement>().expect("filed");
    logout_btn.set_inner_html("logout");
    logout_btn.set_attribute("class", "button my-2").ok();

    let on_click =  Closure::wrap(Box::new( move |_e: Event| {
        info!("logout");
        LocalStorage::delete(auth::LR_AUTH_KEY);
    }) as Box<dyn FnMut(_)>);

    logout_btn.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();  


    div_dashboard.append_child(&logout_btn).expect("failed to append");

    body.append_child(&div_dashboard).expect("failed to append");
}