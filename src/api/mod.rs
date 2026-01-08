use crate::models::Method;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{str::FromStr, time::Instant};

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub body: String,
    pub headers: String,
    pub status: String,
    pub status_code: u16,
    pub time: String,
    pub size: String,
}

#[derive(Debug, Clone)]
pub enum ApiError {
    RequestFailed(String),
}

pub fn perform_request(
    method: Method,
    url: &str,
    body: &str,
    headers_vec: Vec<(String, String)>,
) -> Result<ApiResponse, ApiError> {
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
        Method::POST => client.post(url).headers(headers).body(body.to_string()),
        Method::PUT => client.put(url).headers(headers).body(body.to_string()),
        Method::PATCH => client.patch(url).headers(headers).body(body.to_string()),
        Method::DELETE => client.delete(url).headers(headers),
        Method::GET => client.get(url).headers(headers),
    };

    let start_time = Instant::now();
    let result = request_builder.send();
    let duration = start_time.elapsed();

    match result {
        Ok(response) => {
            let status_code = response.status();
            let size = response.content_length().unwrap_or(0);

            let mut headers_str = String::new();
            for (key, value) in response.headers() {
                headers_str.push_str(&format!("{}: {}\n", key, value.to_str().unwrap_or("")));
            }

            // Pretty Print JSON
            let body_str = match response.json::<Value>() {
                Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_default(),
                Err(_) => "Error: Could not parse JSON".to_string(),
            };

            Ok(ApiResponse {
                body: body_str,
                headers: headers_str,
                status: format!(
                    "{} {}",
                    status_code.as_u16(),
                    status_code.canonical_reason().unwrap_or("")
                ),
                status_code: status_code.as_u16(),
                time: format!("{:.2?}", duration),
                size: format!("{} bytes", size),
            })
        }
        Err(e) => Err(ApiError::RequestFailed(e.to_string())),
    }
}
