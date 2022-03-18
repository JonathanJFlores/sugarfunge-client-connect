use crate::config::*;
use log::*;
use reqwest::{self};
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub error: Option<String>,
    pub message: String,
}


pub async fn user_verify_seed(access_token: &str) -> Result<Option<ResponseMessage>, ResponseMessage> {
    let url = SUGARFUNGE_API.to_string() + "user/verify_seed";

    let client = reqwest::Client::new();
    
    let response = client
        .get(url)
        .bearer_auth(access_token.to_string())
        .send()
        .await;

    match response {
        Ok(response) => {
            let data = response.json::<ResponseMessage>().await;
            match data {
                Ok(info) => {
                    Ok(Some(info))
                }
                Err(e) => {
                    info!("Err {:?}", &e);
                    Ok(None)
                }
            }
        }
        Err(e) => {
            info!("Err {:?}", &e);
            Err(
                ResponseMessage{
                    error: Some("Error".to_string()),
                    message: "Error on request".to_string(),
                }
            )
        }
    }
}
