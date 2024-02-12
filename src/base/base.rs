use super::*;
use crate::MaltedState;
use parking_lot::RwLock;
use rocket::{
    response::{stream::TextStream, Redirect},
    tokio::time::{self, Duration},
    State,
};

#[get("/???")]
pub fn random_site() -> Redirect {
    Redirect::temporary(random_link())
}

#[get("/")]
pub async fn index(
    malted_state: &State<RwLock<MaltedState>>,
    req_info: RequesterInfo<'_>,
) -> TextStream![String] {
    let default_interval = Duration::from_millis(4);
    let long_interval = Duration::from_millis(50);

    let body = [
        "🐢---------------------",
        "START OF TRANSMISSION",
        "---------------------",
        "🐇",
        &format!("{} People call me Malted.", random_greeting()),
        "I research computer graphics, write systems software, do funky 501(c)(3) stuff, and make computers crunch numbers in just the right way.",
        "",
        "Most days, I push to github.com/hackclub. Other days, I force push to github.com/malted. This site is open source; go and find it, and follow me while you're at it.",
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
        "☞ The Year of Living Danishly - Helen Russell",
        "☞ docs.vulkan.org - KHRONOS Group",
        "☞ My Family and Other Animals - Gerald Durrell",
        "",
        "A random, awesome website I think you should visit;",
        "https://malted.dev/???",
        "",
        "",
        ].join("\n");

    let location = if malted_state.read().battery == 0 {
        let loc_in = match req_info.city {
            Some(city) if !city.trim().is_empty() => format!("in {city} "),
            _ => String::from("wherever you are "),
        };

        format!("🐢Hm. I was going to tell you where I am, but apparently my server doesn't know, or doesn't want to tell you. I hope to visit you {loc_in}soon though!🐇\n\n")
    } else {
        location_section(malted_state, &req_info)
    };

    let battery_message = match malted_state.read().battery {
        ..=0 => format!(", but my phone is dead right now, so I won't get your call."),
        b @ 1..=10 => {
            format!(", but my phone is extremely low on battery ({b}%), so I might not answer.")
        }
        b @ 11..=40 => format!(
            ". I'm running low on battery ({b}%), but I should be able to answer your call."
        ),
        b @ 41.. => format!(
            ". I've got plenty of battery right now ({b}%), so I'll probably answer your call."
        ),
    };

    let contact = format!("Message me @Malted on the Hack Club Slack, or email me at this domain. Otherwise, call me at malted at malted dot dev{battery_message}\n");

    let epilogue = [
        "🐢",
        "-------------------",
        "END OF TRANSMISSION",
        "-------------------",
    ]
    .join("\n");

    let agent = req_info.user_agent.map(|ua| ua.to_string());

    TextStream! {
        let mut interval = time::interval(long_interval);
        macro_rules! typewr {
            ($text:expr) => {
                let mut line_max = 50;
                if let Some(ref ua) = agent {
                    if ua.to_lowercase().contains("mobile") {
                        line_max = i32::MAX;
                    }
                }
                let mut line_length = 0;

                /* Safari does not display streamed content realtime
                 * if the first chunk is too small. To get around this,
                 * I send 200 zero-width spaces beforehand. "Safari is
                 * a perfectly executed implementation of the modern web
                 * browser" and other statements uttered by the derranged. */
                yield "​".repeat(200).to_string();

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
                            yield '\n'.to_string();
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

        typewr!(location);

        typewr!(contact);

        interval = time::interval(long_interval);
        typewr!(epilogue);
    }
}

#[get("/raytrace.sql")]
pub async fn raytrace() -> &'static str {
    include_str!("../../include/raytrace.sql")
}
