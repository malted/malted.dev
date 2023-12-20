#[macro_use]
extern crate rocket;
use parking_lot::RwLock;
use std::sync::Arc;
mod api;
mod base;

#[derive(Debug, Default, Clone)]
pub struct MaltedState {
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub country: String,
    pub timestamp: String,
    pub battery: i8,
}

#[launch]
fn rocket() -> _ {
    let mut config = rocket::config::Config::release_default();
    if !cfg!(debug_assertions) {
        config.address = std::net::IpAddr::from([0, 0, 0, 0]);
    }

    rocket::custom(config)
        .manage(Arc::new(RwLock::new(None::<MaltedState>)))
        .mount("/", routes![base::index, base::random_site])
        .mount("/api", routes![api::index, api::patch_location])
}
