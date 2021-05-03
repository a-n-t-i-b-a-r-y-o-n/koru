/// Underlying HTTP client for roku device
use std::time::Duration;
use wake_on_lan::MagicPacket;
use warp::http::StatusCode;

/// GET an endpoint on the device API
pub async fn get(ipv4: &str, endpoint: &str, timeout: Duration) -> Result<String, StatusCode> {
    // Create client and send request
    let response = reqwest::Client::new()
        .get(format!("http://{}:{}/{}", ipv4, 8060, endpoint))
        .timeout(timeout)
        .send()
        .await;
    // Handle response
    match response {
        // Return Ok() w/ response text
        Ok(response) => Ok(response.text().await.unwrap_or(String::new())),
        // Return Error status, specifically for timeouts, or default to Bad Request
        Err(e) => {
            if e.is_timeout() {
                Err(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err(e.status().unwrap_or(StatusCode::BAD_REQUEST))
            }
        }
    }
}

/// POST to an endpoint on the device API
pub async fn post(ipv4: &str, endpoint: &str, body: Option<String>, timeout: Duration) -> Result<String, StatusCode> {
    // Create client and send request
    let response = reqwest::Client::new()
        .post(format!("http://{}:{}/{}", ipv4, 8060, endpoint))
        .body(body.unwrap_or(String::new()))
        .timeout(timeout)
        .send()
        .await;
    // Handle response
    match response {
        // Return Ok() w/ response text
        Ok(response) => Ok(response.text().await.unwrap_or(String::new())),
        // Return Error status or default to Bad Request
        Err(e) => Err(e.status().unwrap_or(StatusCode::BAD_REQUEST))
    }
}

/// POST to an endpoint w/o body, waking the device and retrying on timeout
// Note: useful for e.g. cold-launching apps since it avoids potential timeouts in checking power state
pub async fn waking_post(ipv4: &str, mac_address: &[u8; 6], endpoint: &str, timeout: Duration) -> Result<String, StatusCode> {
    // Create client and send request
    let response = reqwest::Client::new()
        .post(format!("http://{}:{}/{}", ipv4, 8060, endpoint))
        .timeout(timeout)
        .send()
        .await;

    match response {
        Ok(response) => {
            Ok(response.text().await.unwrap_or(String::new()))
        }
        Err(e) => {
            if e.is_timeout() {
                // Retry w/ regular get() if W-o-L succeeds
                match MagicPacket::new(mac_address).send() {
                    Ok(..) => post(ipv4, endpoint, None, timeout).await,
                    // TODO: Think of a real code to use
                    Err(_) => Err(StatusCode::IM_A_TEAPOT)
                }
            } else {
                // Return error message
                Err(e.status().unwrap_or(StatusCode::BAD_REQUEST))
            }
        }
    }
}
