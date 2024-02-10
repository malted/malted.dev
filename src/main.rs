#[macro_use]
extern crate rocket;

mod api;
mod base;
mod content;

#[derive(Debug, Default, Clone)]
pub struct MaltedState {
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub country: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub battery: i8,
}

#[launch]
fn rocket() -> _ {
    let mut config = rocket::config::Config::release_default();
    if !cfg!(debug_assertions) {
        config.address = std::net::IpAddr::from([0, 0, 0, 0]);
        dotenv::dotenv().ok();
    }

    rocket::custom(config)
        .manage(parking_lot::RwLock::new(MaltedState::default()))
        .mount("/", routes![base::index, base::random_site, base::raytrace])
        // .mount("/", FileServer::from(relative!("include")))
        .mount("/api", routes![api::index, api::patch_location])
        .mount("/content", routes![content::map_light, content::map_dark])
}
