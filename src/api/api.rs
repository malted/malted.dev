use crate::MaltedState;
use parking_lot::RwLock;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
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
            city: city.clone(),
            country: country.clone(),
            timestamp,
            battery,
        };
    } else {
        return err("Invalid timestamp");
    }

    let github_pat = std::env::var("github_pat").unwrap();
    let res = match reqwest::blocking::Client::new()
        .patch("https://api.github.com/user")
        .header(ACCEPT, "application/vnd.github+json")
        .header(USER_AGENT, "malted/malted.dev")
        .header(AUTHORIZATION, format!("Bearer {}", github_pat))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .json(&serde_json::json!({ "location": format!("{}, {}", city, country) }))
        .send()
    {
        Ok(res) => res,
        Err(err) => {
            return Json(ApiResponse {
                success: true,
                message: format!("Location saved, but the request to GitHub failed: {err}"),
            })
        }
    };

    let res_text = match res.text() {
        Ok(res_text) => res_text,
        Err(err) => {
            return Json(ApiResponse {
                success: true,
                message: format!("Location saved, reading the response from GitHub failed: {err}"),
            })
        }
    };

    Json(ApiResponse {
        success: true,
        message: format!("Location saved & location updated on GitHub!"),
    })
}
