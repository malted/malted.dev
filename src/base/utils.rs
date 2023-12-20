use rand::prelude::{thread_rng, SliceRandom};
use rocket::{
    request::{self, FromRequest, Outcome},
    Request,
};

#[derive(Debug, Default, FromForm)]
pub struct RequesterInfo {
    pub coords: (f64, f64),
    pub city: String,
    pub region: String,
    pub timezone: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequesterInfo {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if cfg!(debug_assertions) {
            return Outcome::Success(RequesterInfo::default());
        }

        let header_as_string = |header: &str| -> String {
            request
                .headers()
                .get_one(header)
                .expect(&format!("Missing header: {header}"))
                .to_string()
        };
        let lat: f64 = header_as_string("Cf-Iplatitude").parse().unwrap();
        let lon: f64 = header_as_string("Cf-Iplongitude").parse().unwrap();
        let city = header_as_string("Cf-Ipcity");
        let region = header_as_string("Cf-Region");
        let timezone = header_as_string("Cf-Timezone");

        return Outcome::Success(RequesterInfo {
            coords: (lat, lon),
            city,
            region,
            timezone,
        });
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

// pub fn location_section(malted_state: Arc<Mutex<Option<crate::MaltedState>>>) -> String {
//     let tmp = malted_state.lock();
//     if let (Some(ref ts), Some(ref loc)) = (tmp.timestamp, tmp.location) {
//         format!("\nAs of {ts} ago, I'm {loc}\n")
//     } else {
//     }
// }

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
