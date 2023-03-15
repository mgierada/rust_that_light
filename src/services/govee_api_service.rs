use reqwest::Client;
use serde_json::json;

use super::light_setup_service::PayloadBody;

#[derive(Debug)]
struct ApiResponse {
    code: i16,
    message: String,
    data: Option<Data>,
}
#[derive(Debug)]
struct Data {}

pub async fn sent_put_request(
    govee_api_url: &str,
    govee_api_key: &str,
    payload: PayloadBody,
) -> () {
    let client = Client::new();
    let payload_json = json!(payload);
    let _response = client
        .put(govee_api_url)
        .header("Govee-API-Key", govee_api_key)
        .json(&payload_json)
        .send()
        .await
        .unwrap();
}
