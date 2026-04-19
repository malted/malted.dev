use geo::{Distance, Haversine};
use parking_lot::RwLock;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tiny_http::Request;
use url::Url;

use crate::base::location::LOCATION_STATE;
use crate::base::music::SongInfo;

mod api;
mod base;
mod pages;

static MAIN_BODY: &str = include_str!("main.txt");

#[derive(Debug)]
struct State {
    song_info: SongInfo,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let state = Arc::new(RwLock::new(State {
        song_info: base::music::now_playing().await?,
    }));

    let state_2 = state.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;

            let si = match base::music::now_playing().await {
                Ok(si) => si,
                Err(_) => continue,
            };
            (*state_2.write()).song_info = si;
        }
    });

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let server = tiny_http::Server::http(format!("0.0.0.0:{port}")).unwrap();
    println!("Started at http://0.0.0.0:{port}");

    for request in server.incoming_requests() {
        let url_string = format!("http://localhost{}", request.url());
        let parsed = Url::parse(&url_string).expect("invalid URL");

        let service: String = parsed
            .path_segments()
            .and_then(|mut seg| seg.next())
            .map(str::to_owned)
            .unwrap_or_default();

        let state_clone = state.clone();
        tokio::spawn(async move {
            match service.as_str() {
                "spotify" => spotify(request),
                "location" => crate::base::location::location(request),
                "api" => api::api(request),
                "foo" => foo(request),
                "linkedin" => crate::pages::linkedin::linkedin(request),
                "cv" => crate::pages::cv::cv(request),
                _ => root(request, state_clone),
            }
        });
    }

    Ok(())
}

fn spotify(request: Request) {
    let mut stream = request.into_writer();
    stream
        .write_all(b"HTTP/1.1 301 Moved Permanently\r\n")
        .unwrap();
    stream
        .write_all(b"Location: https://open.spotify.com/user/zm7avhpuqzbcauht5xygz6ai9\r\n")
        .unwrap();
    stream.write_all(b"Content-Length: 0\r\n").unwrap();
    stream.write_all(b"\r\n").unwrap();
    stream.flush().unwrap();
}

fn foo(request: Request) {
    let body = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.

Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.

Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem."#;

    paper_page(request, "FOO", body, None);
}

fn root(request: Request, state: Arc<RwLock<State>>) {
    let user_agent: Option<String> = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "User-Agent")
        .map(|h| h.value.as_str().to_owned());
    dbg!(&user_agent);
    let lat: Option<f64> = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "Cf-Iplatitude")
        .map(|h| h.value.as_str().parse().ok())
        .flatten();
    let lng: Option<f64> = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "Cf-Iplongitude")
        .map(|h| h.value.as_str().parse().ok())
        .flatten();
    let city: Option<String> = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "Cf-Ipcity")
        .map(|h| h.value.as_str().to_owned());
    let region: Option<String> = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "Cf-Region")
        .map(|h| h.value.as_str().to_owned());
    let visitor_country: Option<String> = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "Cf-Ipcountry")
        .map(|h| h.value.as_str().to_owned());

    let my_location = LOCATION_STATE.lock().unwrap();
    let location_string = match (my_location.as_ref(), lat, lng, visitor_country.as_deref()) {
        (Some(me_loc), Some(lat), Some(lng), Some(their_country)) => {
            let me = geo::point!(x: me_loc.lat, y: me_loc.lng);
            let you = geo::point!(x: lat, y: lng);
            let distance = Haversine.distance(me, you) as isize / 1_000;

            let is_uk = me_loc.country.eq_ignore_ascii_case("UK")
                || me_loc.country.eq_ignore_ascii_case("United Kingdom")
                || me_loc.country.eq_ignore_ascii_case("GB");
            let same_country = me_loc.country.eq_ignore_ascii_case(their_country);

            let where_i_am = if is_uk {
                "at home in the UK".to_string()
            } else if same_country {
                format!("in {}, {}", me_loc.city, me_loc.state)
            } else {
                format!("in {}, {}", me_loc.city, me_loc.country)
            };

            let visitor_loc = match (city.as_deref(), region.as_deref()) {
                (Some(c), Some(r)) => format!(" ({c}, {r})"),
                (Some(c), None) => format!(" ({c})"),
                _ => String::new(),
            };
            let starter = format!(
                "Right now I'm {where_i_am}, which is {distance}km away from you{visitor_loc}."
            );

            match distance {
                d if d < 10 => format!(
                    "I'm {where_i_am} - we're {distance}km apart, practically on top of each other. Let's grab a coffee!"
                ),
                d if d < 100 => format!("{starter} It's a doable drive; let's meet up!"),
                d if d < 500 => format!(
                    "{starter} That's a chonky drive, so let's coordinate & meet up sometime!"
                ),
                d if d < 5000 => format!("{starter} When we're closer, let's meet up!"),
                _ => format!(
                    "{starter} That's like, a whole world away. Why are you so far away? Why am I so far away?? Questions that could be rendered moot with a flight :)"
                ),
            }
        }
        (Some(me_loc), _, _, _) => {
            let is_uk = me_loc.country.eq_ignore_ascii_case("UK")
                || me_loc.country.eq_ignore_ascii_case("United Kingdom")
                || me_loc.country.eq_ignore_ascii_case("GB");

            let where_i_am = if is_uk {
                "at home in the UK".to_string()
            } else {
                format!("in {}, {}", me_loc.city, me_loc.country)
            };

            format!("I'm {where_i_am}.")
        }
        _ => {
            "I can't tell where either of us are right now, but I hope to see you soon!".to_string()
        }
    };
    drop(my_location);

    let si = &state.read().song_info;
    let time = if si.now_playing {
        "I'm currently"
    } else {
        if let Some(ago) = &si.ago {
            &format!("{}{} I was", ago[..1].to_uppercase(), &ago[1..])
        } else {
            "A bit ago I was"
        }
    };

    let song_string = format!(
        "{} listening to {} by {}. This month I've been listening to lots of {}!!",
        time, si.track, si.artist, si.month_artist
    );

    let body = MAIN_BODY
        .replace("🎵", &song_string)
        .replace("📌", &location_string);

    let utc_now = chrono::Utc::now();
    let month_fmt = utc_now.format("%b");
    let year_fmt = utc_now.format("%Y");
    let top_right = format!("{month_fmt} {year_fmt}");

    paper_page(
        request,
        "PROFILE CIRCULAR REV 10",
        &body,
        Some(("MALTED.DEV", &top_right)),
    );
}

// fn root(request: Request, state: Arc<RwLock<State>>) {
//     // let ip = "1.1.1.1"; // Replace with your target IP
//     // let hops = traceroute::Traceroute::new(ip).unwrap().collect::<Vec<_>>();
//     // for hop in hops {
//     //     println!("{:?}", hop);
//     // }

//     let body = BODY_RAW
//         .replace(
//             "🏠",
//             &request
//                 .remote_addr()
//                 .unwrap_or(&SocketAddr::V4(SocketAddrV4::new(
//                     Ipv4Addr::new(127, 0, 0, 1),
//                     8080,
//                 )))
//                 .ip()
//                 .to_string(),
//         )
//         .replace("🎵", &song_string);

//     let is_terminal = if let Some(ua) = &request
//         .headers()
//         .iter()
//         .find(|h| h.field.as_str() == "User-Agent")
//     {
//         ua.value.as_str().contains("crlus")
//     } else {
//         false
//     };

//     let mut stream = request.into_writer();

//     stream_http(&mut stream, false);

//     if is_terminal {
//         let t = std::env::var("SECRET_2").unwrap();
//         let mut bin = "".to_string();
//         for character in t.clone().into_bytes() {
//             bin += &format!("0{:b} ", character);
//         }
//         let bin = bin.replace("0", ":").replace("1", "：");
//         let bin = bin + "\r\n";

//         stream.write_all(bin.as_bytes()).unwrap();
//         stream.write_all(b"\r\n").unwrap(); // End the header section.

//         for c in std::env::var("SECRET_1").unwrap().lines() {
//             stream_writable(&mut stream, "\u{001b}[2J\u{001b}[H");
//             stream_writable(&mut stream, &c.to_string());
//             stream_writable(&mut stream, "\n");
//             thread::sleep(Duration::from_millis(700));
//         }
//     } else {
//         stream.write_all(b"\r\n").unwrap(); // End the header section
//         stream_header(&mut stream);
//         for c in body.chars() {
//             stream_writable(&mut stream, &c.to_string());
//             thread::sleep(Duration::from_micros(1_000));
//         }
//     };

//     stream.write_all(b"0\r\n\r\n").unwrap(); // End
//     stream.flush().unwrap();
// }

fn paper_page(request: Request, title: &str, body: &str, top_line: Option<(&str, &str)>) {
    let mut line_max = 60;

    let user_agent = request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "User-Agent")
        .map(|h| h.value.as_str().to_owned());

    if let Some(ref ua) = user_agent {
        if ua.to_lowercase().contains("mobile") {
            line_max = 38;
        }
    }

    let mut stream = request.into_writer();
    stream_http(&mut stream, true);
    stream_header(&mut stream);

    let title_length = title.chars().count();
    let title_px = (line_max - title_length) / 2;

    let mut formatted = String::from("\n\n");

    if let Some((left, right)) = top_line {
        let padding = line_max - left.len() - right.len();
        formatted += left;
        formatted += &" ".repeat(padding);
        formatted += right;
        formatted += "\n\n\n";
    }

    formatted += &" ".repeat(title_px - 2);
    formatted += "┌";
    formatted += &"─".repeat(title_length + 2);
    formatted += "┐\n";
    formatted += &" ".repeat(title_px - 2);
    formatted += "│ ";
    formatted += title;
    formatted += " │\n";
    formatted += &" ".repeat(title_px - 2);
    formatted += "└";
    formatted += &"─".repeat(title_length + 2);
    formatted += "┘";
    formatted += "\n\n";
    formatted += &"═".repeat(line_max);
    formatted += "\n\n";
    formatted += body;
    formatted += "\n\n";

    let broken_lines: String = formatted
        .lines()
        .map(|line| {
            let mut result = String::new();
            let mut remaining = line;

            while !remaining.is_empty() {
                if remaining.chars().count() <= line_max {
                    result.push_str(remaining);
                    break;
                }

                let chars: Vec<char> = remaining.chars().collect();
                let mut break_point = line_max;
                for i in (0..line_max).rev() {
                    if chars[i] == ' ' {
                        break_point = i;
                        break;
                    }
                }

                let byte_index = remaining
                    .char_indices()
                    .nth(break_point)
                    .map(|(byte_idx, _)| byte_idx)
                    .unwrap_or(remaining.len());

                let (current_part, rest) = remaining.split_at(byte_index);
                result.push_str(current_part.trim_end());
                result.push('\n');
                remaining = rest.trim_start();
            }

            result.push('\n');
            result
        })
        .collect();

    for (line_idx, line) in broken_lines.lines().enumerate() {
        let margin = format!(" {} |  ", if line_idx % 3 == 1 { "◯" } else { " " });
        let line = margin + line + "\n";

        for c in line.chars() {
            stream_writable(&mut stream, &c.to_string());

            if c != '\n' && c != ' ' {
                thread::sleep(Duration::from_micros(5_000));
            }
        }
    }

    stream.write_all(b"0\r\n\r\n").unwrap();
    stream.flush().unwrap();
}

fn stream_writable(stream: &mut Box<dyn Write + Send + 'static>, content: &str) {
    stream
        .write_all(format!("{:x}\r\n{}\r\n", content.len(), content).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}

fn stream_header(stream: &mut Box<dyn Write + Send + 'static>) {
    let zero_width_spaces: String = std::iter::repeat('\u{200B}').take(342).collect();
    stream.write_all(b"402\r\n").unwrap();
    stream.write_all(zero_width_spaces.as_bytes()).unwrap();
    stream.write_all(b"\r\n").unwrap();
    stream.flush().unwrap();
}

fn stream_http(stream: &mut Box<dyn Write + Send + 'static>, finish: bool) {
    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.write_all(b"Transfer-Encoding: chunked\r\n").unwrap();
    stream
        .write_all(b"Content-Type: text/plain; charset=utf-8\r\n")
        .unwrap();

    if finish {
        stream.write_all(b"\r\n").unwrap();
    }
}
