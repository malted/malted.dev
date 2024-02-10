use crate::MaltedState;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use p256::pkcs8::DecodePrivateKey;
use parking_lot::RwLock;
use rocket::response::Redirect;
use rocket::State;
use std::env;

fn img(malted_state: &State<RwLock<MaltedState>>, colour_scheme: &str) -> Redirect {
    let team_id = env::var("APPLE_TEAM_ID").expect("apple team id");
    let key_id = env::var("APPLE_KEY_ID").expect("apple key id");
    let private_key = env::var("APPLE_PRIVATE_KEY").expect("apple private key");

    let s = malted_state.read();
    let query = format!("{},{}", s.city, s.country);
    let query = urlencoding::encode(&query);
    let query = format!("center={query}&z=8&scale=2&colorScheme={colour_scheme}");
    let path = format!("/api/v1/snapshot?{query}&teamId={team_id}&keyId={key_id}");

    let signature: Signature = SigningKey::from_pkcs8_pem(&private_key)
        .unwrap()
        .sign(&path.as_bytes());
    let signature: String = URL_SAFE.encode(signature.to_bytes());

    let url = format!("https://snapshot.apple-mapkit.com/{path}&signature={signature}");

    Redirect::to(url)
}

#[rocket::get("/map/light")]
pub fn map_light(malted_state: &State<RwLock<MaltedState>>) -> Redirect {
    img(malted_state, "light")
}

#[rocket::get("/map/dark")]
pub fn map_dark(malted_state: &State<RwLock<MaltedState>>) -> Redirect {
    img(malted_state, "dark")
}
