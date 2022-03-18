use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use gloo_storage::*;
use web_sys::{window, HtmlButtonElement, Event};
use log::*;
use crate::auth;
use crate::token_model;


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

    let request_btn = document.create_element("button").expect("field no created").dyn_into::<HtmlButtonElement>().expect("filed");
    request_btn.set_inner_html("Request");
    request_btn.set_attribute("class", "button my-2").ok();

    let on_send =  Closure::wrap(Box::new( move |_e: Event| {
        let auth_data: Option<auth::Auth> = LocalStorage::get(auth::LR_AUTH_KEY).ok();
        if let Some(auth_info) = auth_data {
            spawn_local(async move {
                let response = token_model::asset_create_class(&auth_info.access_token).await;
                match response {
                    Ok(data) => {
                        info!("Create Class Output {:?}", data);
                    },
                    Err(_e) => {}
                }
            })
        } else {

        }
    }) as Box<dyn FnMut(_)>);

    request_btn.set_onclick(Some(on_send.as_ref().unchecked_ref()));
    on_send.forget();  


    div_dashboard.append_child(&logout_btn).expect("failed to append");
    div_dashboard.append_child(&request_btn).expect("failed to append");

    body.append_child(&div_dashboard).expect("failed to append");
}