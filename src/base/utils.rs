use geo::algorithm::haversine_distance::HaversineDistance;

use rand::prelude::{thread_rng, SliceRandom};
use rocket::{
    request::{self, FromRequest, Outcome},
    Request,
};

#[derive(Debug, Default, FromForm)]
pub struct RequesterInfo<'r> {
    pub coords: Option<(f64, f64)>,
    pub city: Option<&'r str>,
    pub region: Option<&'r str>,
    pub timezone: Option<&'r str>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequesterInfo<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if cfg!(debug_assertions) {
            return Outcome::Success(RequesterInfo::default());
        }

        let headers = request.headers();

        let lat: Option<f64> = headers
            .get_one("Cf-Iplatitude")
            .and_then(|x| x.parse().ok());
        let lon = headers
            .get_one("Cf-Iplongitude")
            .and_then(|x| x.parse().ok());
        let city = headers.get_one("Cf-Ipcity");
        let region = headers.get_one("Cf-Region");
        let timezone = headers.get_one("Cf-Timezone");

        let coords = match (lat, lon) {
            (Some(lat), Some(lon)) => Some((lat, lon)),
            _ => None,
        };

        Outcome::Success(RequesterInfo {
            coords,
            city,
            region,
            timezone,
        })
    }
}

pub fn random_greeting() -> &'static str {
    let greetings: Vec<&str> = include_str!("../../include/greetings.txt")
        .split("\n")
        .collect();
    greetings.choose(&mut thread_rng()).unwrap()
}

pub fn random_link() -> &'static str {
    let links: Vec<&str> = include_str!("../../include/links.txt")
        .split("\n\n")
        .map(|x| x.split("\n").nth(1).unwrap())
        .collect();
    links.choose(&mut thread_rng()).unwrap()
}

pub fn location_section(
    malted_state: &rocket::State<parking_lot::RwLock<crate::MaltedState>>,
    req_info: &RequesterInfo,
) -> String {
    let malted_state = malted_state.read();

    let relative = chrono::Utc::now() - malted_state.timestamp;
    let relative = std::time::Duration::from_secs(
        relative
            .to_std()
            .expect("Time should be positive")
            .as_secs(),
    );

    let ago = format!("As of {} ago,", humantime::format_duration(relative));

    match (req_info.coords, req_info.city, req_info.region) {
        (Some(req_coords), Some(req_city), Some(req_region)) => {
            let me = geo::point!(x: malted_state.lat, y: malted_state.lon);
            let you = geo::point!(x: req_coords.0, y: req_coords.1);
            let distance = me.haversine_distance(&you) as isize / 1_000; // In kilometres

            let starter_near = format!(
                "{ago} I'm in {}, which is {distance}km away from {req_city}.",
                malted_state.city
            );

            let starter_far = format!(
                "{ago} I'm in {}, {}, which is {distance}km away from {req_city}, {req_region}.",
                malted_state.city, malted_state.country
            );

            match distance {
                d if d < 10 => format!("{ago} we're both in {req_city} - {distance}km is practically on top of each other. Let's grab a coffee!"),
                d if d < 100 => format!("{starter_near} It's a doable drive; let's meet up!"),
                d if d < 500 => format!("{starter_far} That's a chonky drive, so let's coordinate & meet up sometime!"),
                d if d < 5000 => format!("{starter_far} When we're closer, let's meet up!"),
                _ => format!("{starter_far} That's like, a whole world away. Why are you so far away? Why am I so far away?? Questions that could be rendered moot with a flight :)"),
            }
        }
        _ => format!(
            "{ago} I'm in {}. I can't tell where you are from your IP address, but I hope to see you soon!",
            malted_state.city
        ),
    }
}

pub fn justify(should_justify: bool, text: &[&str], max_length: usize) -> String {
    text.iter()
        .map(|x| {
            if should_justify {
                format!("{:^width$}", x, width = max_length)
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
