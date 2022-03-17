use crate::config::*;
use log::*;
use reqwest::{self};

use serde::Serialize;


#[derive(Debug, Default, Serialize)]
pub struct ResponseMessage {
    pub error: Option<String>,
    pub message: String,
}


pub async fn user_verify_seed(access_token: &str) -> Result<Option<ResponseMessage>, ResponseMessage> {
    let url = SUGARFUNGE_API.to_string() + "user/verify_seed";
    // let mut headers = HeaderMap::new();
    // let token = "Bearer ".to_owned() + &access_token;

    // headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).expect("no token"));
    // headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).expect("no token"));

    // info!("url {} headers {:?}", url, headers);
    let client = reqwest::Client::new();
    
    let response = client
        .get(url)
        .bearer_auth(access_token.to_string())
        // .fetch_mode_no_cors()
        // .headers(headers)
        .send()
        .await;

    info!("data {:?}", response);
    Ok(Some(ResponseMessage {
        error: None,
        message: "hello".to_string(),
    }))
}
