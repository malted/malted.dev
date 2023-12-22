use crate::MaltedState;
use parking_lot::RwLock;
use rocket::{serde::json::Json, State};

#[derive(serde::Serialize)]
pub struct ApiResponse {
    success: bool,
    message: String,
}

#[rocket::get("/")]
pub fn index() -> Json<ApiResponse> {
    Json(ApiResponse {
        success: true,
        message: "Hello, world!".to_string(),
    })
}

#[rocket::patch("/status?<token>&<lat>&<lon>&<city>&<country>&<timestamp>&<battery>")]
pub fn patch_location(
    malted_state: &State<RwLock<MaltedState>>,
    token: String,
    lat: f64,
    lon: f64,
    city: String,
    country: String,
    timestamp: &str,
    battery: i8,
) -> Json<ApiResponse> {
    let err = |msg: &str| -> Json<ApiResponse> {
        Json(ApiResponse {
            success: false,
            message: msg.to_string(),
        })
    };

    if !cfg!(debug_assertions) {
        if token != std::env::var("secret_token").unwrap() {
            return err("Invalid token");
        }
    }

    let timestamp = timestamp.trim().replace(' ', "+");
    if timestamp.chars().filter(|&c| c == '+').count() > 1 {
        return err("Invalid timestamp; too many spaces (or +'s)");
    }

    if let Ok(timestamp) =
        chrono::DateTime::parse_from_rfc3339(&timestamp).map(|x| x.with_timezone(&chrono::Utc))
    {
        *malted_state.write() = MaltedState {
            lat,
            lon,
            city,
            country,
            timestamp,
            battery,
        };
    } else {
        return err("Invalid timestamp");
    }

    Json(ApiResponse {
        success: true,
        message: "Location saved".to_string(),
    })
}
