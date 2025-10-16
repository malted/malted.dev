use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Request, Response};
use url::Url;

// /api
pub fn api(request: Request) {
    let choice = request
        .url()
        .split("/")
        .nth(2)
        .expect("a second url component");

    match choice {
        "location" => handle_location(request),
        "get_location" => get_location(request),
        _ => {
            let response = Response::from_string("Endpoint not found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}

// Global state to store location data
lazy_static::lazy_static! {
    static ref LOCATION_STATE: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

fn handle_location(request: Request) {
    let url_string = format!("http://localhost{}", request.url());
    let parsed_url = match Url::parse(&url_string) {
        Ok(url) => url,
        Err(_) => {
            let response = Response::from_string("Invalid URL").with_status_code(400);
            let _ = request.respond(response);
            return;
        }
    };

    // Parse query parameters
    let query_pairs: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    let lat = query_pairs.get("lat").unwrap_or(&String::new()).clone();
    let lng = query_pairs.get("lng").unwrap_or(&String::new()).clone();
    let provided_key = query_pairs.get("key").unwrap_or(&String::new()).clone();

    // Get secret key from environment
    let secret_key = match env::var("SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            let response =
                Response::from_string("Server configuration error").with_status_code(500);
            let _ = request.respond(response);
            return;
        }
    };

    // Validate the provided key
    if provided_key != secret_key {
        let response = Response::from_string("Unauthorized").with_status_code(401);
        let _ = request.respond(response);
        return;
    }

    // Validate lat and lng are not empty
    if lat.is_empty() || lng.is_empty() {
        let response =
            Response::from_string("Missing latitude or longitude parameters").with_status_code(400);
        let _ = request.respond(response);
        return;
    }

    // Save the location state
    {
        let mut state = LOCATION_STATE.lock().unwrap();
        state.insert("latitude".to_string(), lat.clone());
        state.insert("longitude".to_string(), lng.clone());
        state.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
    }

    // Return success response
    let response_body = format!("Location saved: lat={}, lng={}", lat, lng);
    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());

    let _ = request.respond(response);
}

fn get_location(request: Request) {
    let state = LOCATION_STATE.lock().unwrap();

    if state.is_empty() {
        let response = Response::from_string("No location data available").with_status_code(404);
        let _ = request.respond(response);
        return;
    }

    let lat = state
        .get("latitude")
        .map(|s| s.as_str())
        .unwrap_or("unknown");
    let lng = state
        .get("longitude")
        .map(|s| s.as_str())
        .unwrap_or("unknown");
    let timestamp = state
        .get("timestamp")
        .map(|s| s.as_str())
        .unwrap_or("unknown");

    let response_body = format!(
        "Current location:\nLatitude: {}\nLongitude: {}\nLast updated: {}",
        lat, lng, timestamp
    );

    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());

    let _ = request.respond(response);
}
