use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};
use uuid::Uuid;

#[derive(Serialize)]
struct TokenRequest<'a> {
    resource: &'static str,
    client_id: &'static str,
    grant_type: &'static str,
    username: &'a str,
    password: &'a str,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let tenant_id = &args[1];
    let username = &args[2];
    let password = &args[3];
    let endpoint = &args[4];

    // Create token request
    let token_request = TokenRequest {
        resource: "74658136-14ec-4630-ad9b-26e160ff0fc6",
        client_id: "1950a258-227b-4e31-a9cf-717495945fc2",
        grant_type: "password",
        username,
        password,
    };

    // Send token request
    let uri = format!(
        "https://login.microsoftonline.com/{}/oauth2/token",
        tenant_id
    );
    let client = reqwest::Client::new();
    let response = client
        .post(&uri)
        .form(&token_request)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    let token_response: TokenResponse = response.json().await?;

    // Send request
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token_response.access_token))?,
    );
    headers.insert(
        "x-ms-client-request-id",
        HeaderValue::from_str(&Uuid::new_v4().to_string())?,
    );
    headers.insert(
        "x-ms-client-session-id",
        HeaderValue::from_str(&Uuid::new_v4().to_string())?,
    );
    let uri = endpoint.to_string() + "/skus?api-version=1.6";
    let response = client.get(uri).headers(headers).send().await?;
    let json_response: serde_json::Value = response.json().await?;
    let license = &json_response.as_array().unwrap()[0];
    println!("{:#?}", license);
    Ok(())
}
