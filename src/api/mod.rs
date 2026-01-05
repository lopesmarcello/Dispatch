use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{str::FromStr, time::Instant};

pub struct RequestResult {
    pub body: String,
    pub status: String,
    pub time: String,
    pub size: String,
    pub is_error: bool,
}

pub fn perform_request(
    method: &str,
    url: &str,
    body: &str,
    headers_vec: Vec<(String, String)>,
) -> RequestResult {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    for (key, value) in headers_vec {
        if let Ok(h_name) = HeaderName::from_str(&key) {
            if let Ok(h_val) = HeaderValue::from_str(&value) {
                headers.insert(h_name, h_val);
            }
        }
    }

    let request_builder = match method {
        "POST" => client.post(url).body(body.to_string()),
        "PUT" => client.put(url).body(body.to_string()),
        "PATCH" => client.patch(url).body(body.to_string()),
        "DELETE" => client.delete(url),
        _ => client.get(url).headers(headers),
    };

    let start_time = Instant::now();
    let result = request_builder.send();
    let duration = start_time.elapsed();

    match result {
        Ok(response) => {
            let status_code = response.status();
            let size = response.content_length().unwrap_or(0);

            // Pretty Print JSON
            let body_str = match response.json::<Value>() {
                Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_default(),
                Err(_) => "Error: Could not parse JSON".to_string(),
            };

            RequestResult {
                body: body_str,
                status: format!(
                    "{} {}",
                    status_code.as_u16(),
                    status_code.canonical_reason().unwrap_or("")
                ),
                time: format!("{:.2?}", duration),
                size: format!("{} bytes", size),
                is_error: status_code.is_client_error() || status_code.is_server_error(),
            }
        }
        Err(e) => RequestResult {
            body: format!("Request Failed: {}", e),
            status: "Error".to_string(),
            time: "0 ms".to_string(),
            size: "0 bytes".to_string(),
            is_error: true,
        },
    }
}
