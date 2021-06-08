use crate::util::http_client::HttpClient;
use dotenv::dotenv;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::env;

const ENDPOINT: &str = "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=";

pub async fn get_gh_info_with_token(token: String, client: HttpClient) -> String {
    let mut payload = HashMap::new();
    dotenv().ok();
    payload.insert("idToken".to_string(), token);
    let res = client
        .post(
            &format!("{}{}", ENDPOINT, env::var("FIREBASE_API_KEY").unwrap()),
            payload,
        )
        .await;

    let json: Value = serde_json::from_str(&res.body).unwrap();
    json["users"][0]["screenName"].to_string()
}
