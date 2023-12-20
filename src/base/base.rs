use super::*;
use crate::MaltedState;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::response::{stream::TextStream, Redirect};
use rocket::tokio::sync::Mutex;
use rocket::tokio::time::{self, Duration};
use rocket::Request;
use rocket::State;

use std::sync::Arc;

#[get("/???")]
pub fn random_site() -> Redirect {
    Redirect::temporary(random_link())
}

#[derive(Debug, FromForm)]
pub struct RequesterIp(Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequesterIp {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if cfg!(debug_assertions) {
            return Outcome::Success(RequesterIp(None));
        }
        match request.headers().get_one("X-Forwarded-For") {
            Some(ip) => Outcome::Success(RequesterIp(Some(ip.to_string()))),
            None => Outcome::Error((Status::BadRequest, ())),
        }
    }
}

#[get("/")]
pub async fn index(
    malted_state: &State<Arc<parking_lot::RwLock<Option<MaltedState>>>>,
    real_ip: RequesterIp,
) -> TextStream![String] {
    let default_interval = Duration::from_millis(4);
    let long_interval = Duration::from_millis(50);

    let remote_loc: Arc<Mutex<Option<(f64, f64)>>> = Arc::new(Mutex::new(None));
    let thread_remote_loc = remote_loc.clone();

    if let Some(req_ip_raw) = real_ip.0 {
        let req_ip = req_ip_raw.parse::<std::net::IpAddr>().unwrap();

        rocket::tokio::task::spawn(async move {
            if req_ip.is_loopback() {
                println!("Loopback IP, skipping IP lookup");
                return;
            }

            println!("Remote IP: {:#?}", req_ip);

            let req = reqwest::get(format!("https://ip-api.com/line/{req_ip}?fields=lat,lon"))
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .text()
                .await
                .unwrap();

            println!(
                "{}",
                format!("https://ip-api.com/line/{req_ip}?fields=lat,lon")
            );
            println!("{:#?}", req);

            let loc: (&str, &str) = req.split_once('\n').unwrap();

            println!("{:#?}", loc);

            thread_remote_loc
                .lock()
                .await
                .replace((loc.0.parse::<f64>().unwrap(), loc.1.parse::<f64>().unwrap()));
        });
    }

    let body = [
        "🐢---------------------",
        "START OF TRANSMISSION",
        "---------------------",
        "🐇",
        &format!("{} People call me Malted.", utils::random_greeting()),
        "I research computer graphics, write systems software, do funky 501(c)(3) stuff, and make computers crunch numbers in just the right way.",
        "",
        "Most days, I push to github.com/hackclub. Other days, I force push to github.com/malted. This site is open source; go and find it, and give me a follow while you're at it.",
        "",
        "It's my job to;",
        "Design hypertext documents",
        "Write hypertext documents",
        "💸 Give internet money wiiings 💸",
        "",
        "It's not my job to;",
        "Write shaders, yet I do it anyway",
        "",
        "You should read;",
        "☞ Why the Future Doesn't Need Us - Bill Joy for WIRED",
        "☞ docs.vulkan.org - KHRONOS Group",
        "☞ My Family and Other Animals - Gerald Durrell",
        "",
        "A random, awesome website I think you should visit;",
        "https://malted.dev/???",
        "",
        "",
    ];
    let max_length = body
        .iter()
        .map(|s| s.replace(['🐢', '🐇'], "").len())
        .max()
        .unwrap_or(0);

    let body = utils::justify(false, &body, max_length);

    let battery_message = if let Some(state) = malted_state.read().clone() {
        let battery = state.battery;
        match battery {
            ..=10 => format!(", but my phone is extremely low on battery ({battery}%), so I might not answer your call."),
            11..=40 => format!(". I'm running low on battery ({battery}%), but I should be able to answer your call."),
            41.. => format!(". I've got plenty of battery right now ({battery}%), so I'll probably answer your call."),
        }
    } else {
        ", but my phone is dead right now, so I won't get your call.".to_string()
    };

    let contact = utils::justify(
        false,
        &[
            &format!("Message me @Malted on the Hack Club Slack, or email me at this domain. If you're in a pinch, call me at malted at malted dot dev (but only for the pinchiest of pinches){battery_message}"),
            "",
        ],
        max_length,
    );

    let epilogue = utils::justify(
        false,
        &[
            "🐢",
            "-------------------",
            "END OF TRANSMISSION",
            "-------------------",
        ],
        max_length,
    );

    TextStream! {
        let mut interval = time::interval(long_interval);
        macro_rules! typewr {
            ($text:expr) => {
                let line_max = 50;
                let mut line_length = 0;
                for c in $text.chars() {
                    if c == '🐢' {
                        interval = time::interval(long_interval);
                        continue;
                    } else if c == '🐇' {
                        interval = time::interval(default_interval);
                        continue;
                    } else {
                        if c == '\n' {
                            yield c.to_string();
                            line_length = 0;
                        } else if line_length >= line_max && c == ' ' {
                            yield "\n".to_string();
                            line_length = 0;
                        } else {
                            line_length += 1;
                            yield c.to_string();
                        }
                    }

                    if c != '\n' && c != '\r' && c != '\t' && c != ' ' {
                        interval.tick().await;
                    }
                }
            };
        }

        typewr!(body);

        if let Some((lat, lon)) = remote_loc.lock().await.clone() {
            typewr!(format!("{lat} {lon}"));
        } else {
            typewr!("🐢Hm. I was going to tell you where I am, but apparently my server doesn't know, or doesn't want to tell you.🐇\n\n");
        }
        // Check if malted_state is some
        // if let Some(state) = malted_state.clone().read() {
        //     // interval = time::interval(long_interval);
        //     //     typewr!("\nHm. I was going to tell you where I am, but apparently my server doesn't know, or doesn't want to tell you.\n\n".to_string());
        // } else {
        //     let malted_state = malted_state.read();
        //     let remote_loc = remote_loc.lock();
        // }

        // typewr!(utils::location_section(malted_state));
        // interval = time::interval(default_interval);

        typewr!(contact);

        interval = time::interval(long_interval);
        typewr!(epilogue);
    }
}
