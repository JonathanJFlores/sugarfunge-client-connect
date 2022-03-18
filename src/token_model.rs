use std::str::FromStr;

use crate::config::*;
use log::info;
use reqwest::{self, header::{HeaderMap}};
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize, Deserialize)]
pub struct CreateClassInput {
    class_id: u64,
    metadata: serde_json::Value,
    owner: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateClassOutput {
    class_id: u64,
    who: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub error: Option<String>,
    pub message: String,
}


pub async fn asset_create_class(access_token: &str) -> Result<Option<CreateClassOutput>, ResponseMessage> {
    let url = SUGARFUNGE_API.to_string() + "asset/create_class";


    let str_data = json!({
        "userdata": "woops"
    });

    let data = CreateClassInput {
        class_id: 7,
        metadata: str_data,
        owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()
    };

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/json".parse().unwrap(),
    );
    
    let response = client
        .post(url)
        .bearer_auth(access_token.to_string())
        .headers(headers)
        .json(&data)
        .send()
        .await;


    match response {
        Ok(response) => {
            let data = response.json::<CreateClassOutput>().await;
            match data {
                Ok(info) => {
                    Ok(Some(info))
                }
                Err(e) => {
                    info!("Err {:?}", &e);
                    Err(ResponseMessage {
                        error: Some("".to_string()),
                        message: "".to_string(),
                    })
                }
            }
        }
        Err(e) => {
            info!("Err {:?}", &e);
            Err(ResponseMessage {
                error: Some("".to_string()),
                message: "".to_string(),
            })
        }
    }

}