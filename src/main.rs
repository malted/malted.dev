#[macro_use]
extern crate rocket;
use rocket::response::{stream::TextStream, Redirect};
use rocket::tokio::time::{self, Duration, Interval};
use std::net::IpAddr;
mod utils;

#[get("/???")]
fn random_site() -> Redirect {
    Redirect::temporary(utils::random_link())
}

#[get("/")]
fn index() -> TextStream![String] {
    let default_interval = Duration::from_millis(4);
    let long_interval = Duration::from_millis(40);

    let location: Option<String> = None;
    let timestamp: Option<String> = None;
    // let location = "I'm at home; we're approx. 100 km away from each other right now.";

    let body = [
        "🐢---------------------",
        "START OF TRANSMISSION",
        "---------------------",
        "🐇",
        &format!("{} People call me Malted.", utils::random_greeting()),
        "I research computer graphics, write systems software, do funky 501(c)(3) stuff, and make computers crunch numbers in just the right way.",
        "",
        "Most days, I push to github.com/hackclub.",
        "Other days, I force push to github.com/malted.",
        "This site is open source; go and find it,",
        "and give me a follow while you're at it.",
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
    ];
    let max_length = body
        .iter()
        .map(|s| s.replace(['🐢', '🐇'], "").len())
        .max()
        .unwrap_or(0);

    let body = utils::justify(false, &body, max_length);

    let contact = utils::justify(
        false,
        &[
            "Message me @Malted on the Hack Club Slack, or email me at this domain. If you're in a pinch, call me at malted at malted dot dev (but only for the pinchiest of pinches).",
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

        if timestamp.is_none() || location.is_none() {
            interval = time::interval(long_interval);
        }
        typewr!(utils::location_section(timestamp, location));
        interval = time::interval(default_interval);

        typewr!(contact);

        interval = time::interval(long_interval);
        typewr!(epilogue);
    }
}

#[launch]
fn rocket() -> _ {
    let mut config = rocket::config::Config::release_default();
    config.address = IpAddr::from([0, 0, 0, 0]);

    rocket::custom(config).mount("/", routes![index, random_site])

    // rocket::custom(config)
    //     .mount("/", routes![/* .. */])
    //     .launch();
    // rocket::build().mount("/", routes![index, random_site])
}
