use std::collections::HashMap;
use std::env;
use tiny_http::{Header, Request, Response};
use url::Url;

use crate::base::location::{LocationInfo, LOCATION_STATE};

// /api
pub fn api(request: Request) {
    let choice = request
        .url()
        .split("/")
        .nth(2)
        .expect("a second url component")
        .split('?')
        .next()
        .unwrap();

    match choice {
        "location" => handle_location(request),
        "get_location" => get_location(request),
        _ => {
            let response = Response::from_string("Endpoint not found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
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

    let provided_key = query_pairs.get("key").unwrap_or(&String::new()).clone();

    let secret_key = match env::var("SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            let response =
                Response::from_string("Server configuration error").with_status_code(500);
            let _ = request.respond(response);
            return;
        }
    };

    if provided_key != secret_key {
        let response = Response::from_string("Unauthorized").with_status_code(401);
        let _ = request.respond(response);
        return;
    }

    let lat: f64 = match query_pairs.get("lat").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            let response = Response::from_string("Missing or invalid lat").with_status_code(400);
            let _ = request.respond(response);
            return;
        }
    };
    let lng: f64 = match query_pairs.get("lng").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            let response = Response::from_string("Missing or invalid lng").with_status_code(400);
            let _ = request.respond(response);
            return;
        }
    };
    let city = query_pairs.get("city").cloned().unwrap_or_default();
    let state_name = query_pairs.get("state").cloned().unwrap_or_default();
    let country = query_pairs.get("country").cloned().unwrap_or_default();

    {
        let mut state = LOCATION_STATE.lock().unwrap();
        *state = Some(LocationInfo {
            lat,
            lng,
            city: city.clone(),
            state: state_name.clone(),
            country: country.clone(),
        });
    }

    let response_body = format!(
        "Location saved: lat={}, lng={}, city={}, state={}, country={}",
        lat, lng, city, state_name, country
    );
    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());

    let _ = request.respond(response);
}

fn get_location(request: Request) {
    let state = LOCATION_STATE.lock().unwrap();

    let loc = match state.as_ref() {
        Some(loc) => loc,
        None => {
            let response =
                Response::from_string("No location data available").with_status_code(404);
            let _ = request.respond(response);
            return;
        }
    };

    let response_body = format!(
        "Current location:\nLatitude: {}\nLongitude: {}\nCity: {}\nState: {}\nCountry: {}",
        loc.lat, loc.lng, loc.city, loc.state, loc.country
    );

    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());

    let _ = request.respond(response);
}
