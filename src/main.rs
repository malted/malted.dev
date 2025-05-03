#[macro_use]
extern crate rocket;

mod api;
mod content;
mod index;
mod newsletter;

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
    dotenv::dotenv().ok();

    let mut config = rocket::config::Config::release_default();
    config.port = 8080;

    if !cfg!(debug_assertions) {
        config.address = std::net::IpAddr::from([0, 0, 0, 0]);
    }

    rocket::custom(config)
        .manage(parking_lot::RwLock::new(MaltedState::default()))
        .mount(
            "/",
            routes![index::index, index::random_site, index::raytrace],
        )
        .mount("/api", routes![api::index, api::patch_location])
        .mount("/content", routes![content::map_light, content::map_dark])
        .mount(
            "/bottle",
            routes![
                newsletter::subscribe,
                newsletter::slack_slash_command_handler,
                newsletter::form
            ],
        )
}
