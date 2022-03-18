use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use reqwest::{self, header::{HeaderMap}};
use log::*;


pub const LR_AUTH_KEY: &'static str = "app.sugarfunge-client-example.auth";

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    pub id_token: String,
    pub session_state: String,
    pub scope: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Claims {
    #[serde(rename = "x-hasura-allowed-roles", default)]
    pub allowed_roles: Vec<String>,
    #[serde(rename = "x-hasura-default-role", default)]
    pub default_role: String,
    #[serde(rename = "x-hasura-user-id", default)]
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Token {
    #[serde(rename = "https://hasura.io/jwt/claims", default)]
    pub claims: Claims,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    #[serde(rename = "sub", default)]
    pub keycloak_user_id: Uuid,
}

#[derive(Debug, Default, Serialize)]
pub struct AccessTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub scope: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RefreshTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Serialize)]
pub enum TokenRequest {
    AccessToken(AccessTokenRequest),
    RefreshToken(RefreshTokenRequest),
}


#[derive(Debug, Default, Serialize)]
pub struct KeycloakError {
    pub error: String,
    pub error_description: String,
}

pub async fn token_request(req_type: TokenRequest) -> Result<Option<Auth>, anyhow::Error> {
    let url = "http://localhost:8025/auth/realms/Sugarfunge/protocol/openid-connect/token";

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-url-encoded".parse().unwrap(),
    );

    let client = reqwest::Client::new();

    match req_type {
        TokenRequest::AccessToken(data) => {
            let request = client.post(url).headers(headers).form(&data).build();

            match request {
                Ok(req) => {
                    let response = client.execute(req).await;

                    match response {
                        Ok(response) => {
                            let data = response.json::<Auth>().await;
                            match data {
                                Ok(auth) => {
                                    Ok(Some(auth))
                                }
                                Err(e) => {
                                    info!("Err {:?}", &e);
                                    Ok(None)
                                }
                            }
                        }
                        Err(e) => {
                            info!("Err {:?}", &e);
                            // let data_error = e.json::<KeycloakError>().await;
                            Ok(None)
                        }
                    }
                }
                Err(e) => {
                    info!("{:?}", e);
                    Ok(None)
                }
            }
        }
        TokenRequest::RefreshToken(data) => {
            let request = client.post(url).headers(headers).form(&data).build();

            match request {
                Ok(req) => {
                    let response = client.execute(req).await;

                    match response {
                        Ok(response) => {
                            let data = response.json::<Auth>().await;
                            match data {
                                Ok(auth) => {
                                    Ok(Some(auth))
                                }
                                Err(e) => {
                                    info!("Err {:?}", &e);
                                    Ok(None)
                                }
                            }
                        }
                        Err(e) => {
                            info!("Err {:?}", &e);
                            // let data_error = e.json::<KeycloakError>().await;
                            Ok(None)
                        }
                    }
                }
                Err(e) => {
                    info!("{:?}", e);
                    Ok(None)
                }
            }
        }
    }
}