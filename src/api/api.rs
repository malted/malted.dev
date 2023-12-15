use crate::MaltedState;
use parking_lot::RwLock;
use rocket::{serde::json::Json, State};
use std::sync::Arc;

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
    malted_state: &State<Arc<RwLock<Option<MaltedState>>>>,
    token: String,
    lat: f64,
    lon: f64,
    city: String,
    country: String,
    timestamp: String,
    battery: i8,
) -> Json<ApiResponse> {
    if token != std::env::var("secret_token").unwrap() {
        return Json(ApiResponse {
            success: false,
            message: "Invalid token".to_string(),
        });
    }

    *malted_state.write() = Some(MaltedState {
        lat,
        lon,
        city,
        country,
        timestamp,
        battery,
    });

    Json(ApiResponse {
        success: true,
        message: "Location saved".to_string(),
    })
}
