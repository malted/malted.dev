use crate::MaltedState;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use p256::pkcs8::DecodePrivateKey;
use parking_lot::RwLock;
use rocket::response::Redirect;
use rocket::State;
use std::env;

fn img(
    malted_state: &State<RwLock<MaltedState>>,
    colour_scheme: &str,
    zoom: Option<u8>,
    size: (Option<u16>, Option<u16>),
) -> Redirect {
    let team_id = env::var("mk_team_id").expect("mapkit team id");
    let key_id = env::var("mk_key_id").expect("mapkit key id");
    let private_key = env::var("mk_private_key").expect("mapkit private key");
    let private_key = private_key.replace("\\n", "\n");

    let zoom = zoom.unwrap_or(12);
    let size = (size.0.unwrap_or(600), size.1.unwrap_or(400));

    let s = malted_state.read();

    let query = if s.city.trim().is_empty() || s.country.trim().is_empty() || s.city.trim().to_lowercase() == "burlington" {
        String::from("burlington,vermont,usa")
    } else {
        format!("{},{}", s.city, s.country)
    };
    let query = urlencoding::encode(&query);
    let query = format!(
        "center={query}&z={zoom}&size={width}x{height}&scale=2&colorScheme={colour_scheme}",
        width = size.0,
        height = size.1
    );
    let path = format!("/api/v1/snapshot?{query}&teamId={team_id}&keyId={key_id}");

    let signature: Signature = SigningKey::from_pkcs8_pem(&private_key)
        .expect("a valid signature from the private key")
        .sign(&path.as_bytes());
    let signature: String = URL_SAFE.encode(signature.to_bytes());

    let url = format!("https://snapshot.apple-mapkit.com/{path}&signature={signature}");

    Redirect::to(url)
}

#[rocket::get("/map/light?<zoom>&<width>&<height>")]
pub fn map_light(
    malted_state: &State<RwLock<MaltedState>>,
    zoom: Option<u8>,
    width: Option<u16>,
    height: Option<u16>,
) -> Redirect {
    img(malted_state, "light", zoom, (width, height))
}

#[rocket::get("/map/dark?<zoom>&<width>&<height>")]
pub fn map_dark(
    malted_state: &State<RwLock<MaltedState>>,
    zoom: Option<u8>,
    width: Option<u16>,
    height: Option<u16>,
) -> Redirect {
    img(malted_state, "dark", zoom, (width, height))
}
