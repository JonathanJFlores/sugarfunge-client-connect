use crate::auth;
use crate::utils;
use crate::user_model;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo_storage::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement, HtmlButtonElement, Document};
use log::*;

fn setup_username_field(document: Document) {
    let input_username = document.create_element("input").expect("field no created").dyn_into::<HtmlInputElement>().expect("filed");

    input_username.set_id("username-input");
    input_username.set_attribute("type", "text").ok();
    input_username.set_attribute("class", "input my-2").ok();


    let on_input = Closure::wrap(Box::new(|e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
    
        info!("username {:?}", input.value());
    }) as Box<dyn FnMut(_)>);

    input_username.add_event_listener_with_callback("input", on_input.as_ref().unchecked_ref()).ok();
    
    on_input.forget();

    let div_login = document.get_element_by_id("div-login").expect("should have #div-login on the page");
    div_login.append_child(&input_username).expect("failed to append");
}

fn setup_password_field(document: Document) {
    let input_password = document.create_element("input").expect("field no created").dyn_into::<HtmlInputElement>().expect("filed");
    input_password.set_id("password-input");
    input_password.set_attribute("type", "password").ok();
    input_password.set_attribute("class", "input my-2").ok();

    let on_input = Closure::wrap(Box::new(|e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
    
        info!("password {:?}", input.value());
    }) as Box<dyn FnMut(_)>);

    input_password.add_event_listener_with_callback("input", on_input.as_ref().unchecked_ref()).ok();
    
    on_input.forget();

    let div_login = document.get_element_by_id("div-login").expect("should have #div-login on the page");
    div_login.append_child(&input_password).expect("failed to append");
}


fn auth_setup(data: Option<auth::Auth>) {
    if let Some(auth) = data.clone() {
        let access_token = auth.access_token.clone();
        LocalStorage::set(auth::LR_AUTH_KEY, auth).ok();
        spawn_local(async move {
            let response = user_model::user_verify_seed(&access_token).await;
            match response {
                Ok(_data) => {}
                Err(_er) => {}
            }
        })
    }
}

fn setup_login_btn(document: Document) {
    let login_btn = document.create_element("button").expect("field no created").dyn_into::<HtmlButtonElement>().expect("filed");
    login_btn.set_inner_html("login");
    login_btn.set_attribute("class", "button my-2").ok();

    let on_click =  Closure::wrap(Box::new( move |_e: Event| {
        info!("click");

        let username = utils::get_value_input_by_id("username-input");
        let password = utils::get_value_input_by_id("password-input");

        let auth_data =  auth::TokenRequest::AccessToken(auth::AccessTokenRequest {
            grant_type: "password".to_string(),
            client_id: "surgar-users".to_string(),
            username: username,
            password: password,
            scope: "openid".to_string(),
        });

        info!("data {:?}", &auth_data);
        spawn_local(async move {
            let response = auth::token_request(auth_data).await;
            match response {
                Ok(data) => {
                    info!("Login Auth {:?}", data);
                    auth_setup(data)
                },
                Err(_e) => {}
            }
        });

    }) as Box<dyn FnMut(_)>);

    login_btn.set_onclick(Some(on_click.as_ref().unchecked_ref()));

    on_click.forget();     

    let div_login = document.get_element_by_id("div-login").expect("should have #div-login on the page");
    div_login.append_child(&login_btn).expect("failed to append");
}

fn refresh_token(refresh_token: &str) {
    let auth_data_req =  auth::TokenRequest::RefreshToken(auth::RefreshTokenRequest {
        grant_type: "refresh_token".to_string(),
        client_id: "surgar-users".to_string(),
        refresh_token: refresh_token.to_string(),
        scope: "openid".to_string(),
    });

    spawn_local(async move {
        let response = auth::token_request(auth_data_req).await;
        match response {
            Ok(data) => {
                info!("Refresh Token {:?}", data);                    
                auth_setup(data)
            },
            Err(_e) => {}
        }
    });
}


pub fn verify_login() -> bool {
    let auth_data: Option<auth::Auth> = LocalStorage::get(auth::LR_AUTH_KEY).ok();
    // info!("auth {:?}", auth_data);
    if let Some(auth_info) = auth_data {
        refresh_token(&auth_info.refresh_token);
        true
    } else {
        false
    }
}

pub fn setup_login(document: Document) {
    setup_username_field(document.clone());
    setup_password_field(document.clone());
    setup_login_btn(document);
} 