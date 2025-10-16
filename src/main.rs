use geo::algorithm::haversine_distance::HaversineDistance;
use parking_lot::RwLock;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tiny_http::{Request, Response};
use url::Url;

use crate::base::music::SongInfo;

mod api;
mod base;

static LINE_MAX: usize = 60;

#[derive(Debug)]
struct State {
    song_info: SongInfo,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let server = tiny_http::Server::http("0.0.0.0:8000").unwrap();
    println!("Started at http://0.0.0.0:8000");

    for request in server.incoming_requests() {
        let url_string = format!("http://localhost{}", request.url());
        let parsed = Url::parse(&url_string).expect("invalid URL");

        let service: String = parsed
            .path_segments()
            .and_then(|mut seg| seg.next())
            .map(str::to_owned)
            .unwrap_or_default();

        let state_clone = state.clone();
        thread::spawn(move || match service.as_str() {
            "spotify" => spotify(request),
            "api" => api::api(request),
            _ => root(request, state_clone),
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
        .write_all(b"Location: https://open.spotify.com/user/zm7avhpuqzbcauht5xygz6ai9ers\r\n")
        .unwrap();
    stream.write_all(b"Content-Length: 0\r\n").unwrap();
    stream.write_all(b"\r\n").unwrap();
    stream.flush().unwrap();
}

fn root(request: Request, state: Arc<RwLock<State>>) {
    let headers = request.headers();
    let lat: Option<f64> = headers
        .iter()
        .find(|h| h.field.as_str() == "Cf-Iplatitude")
        .map(|h| h.value.as_str().parse().ok())
        .flatten();
    let lng: Option<f64> = headers
        .iter()
        .find(|h| h.field.as_str() == "Cf-Iplongitude")
        .map(|h| h.value.as_str().parse().ok())
        .flatten();
    let city = headers
        .iter()
        .find(|h| h.field.as_str() == "Cf-Ipcity")
        .map(|h| h.value.as_str());
    let region = headers
        .iter()
        .find(|h| h.field.as_str() == "Cf-Region")
        .map(|h| h.value.as_str());
    let timezone = headers
        .iter()
        .find(|h| h.field.as_str() == "Cf-Timezone")
        .map(|h| h.value.as_str());

    let location_string = match (lat, lng, city, region) {
        (Some(lat), Some(lng), Some(city), Some(region)) => {
            let me = geo::point!(x: 51.50722, y: -0.1275);
            let you = geo::point!(x: lat, y: lng);
            let distance = me.haversine_distance(&you) as isize / 1_000; // In kilometres

            let starter_near =
                format!("Right now I'm at home, which is {distance}km away from you in ({city}).");

            let starter_far = format!(
                "Right now I'm at home, which is {distance}km away from you ({city}, {region})."
            );

            match distance {
                d if d < 10 => format!(
                    "we're both in {city} - {distance}km is practically on top of each other. Let's grab a coffee!"
                ),
                d if d < 100 => format!("{starter_near} It's a doable drive; let's meet up!"),
                d if d < 500 => format!(
                    "{starter_far} That's a chonky drive, so let's coordinate & meet up sometime!"
                ),
                d if d < 5000 => format!("{starter_far} When we're closer, let's meet up!"),
                _ => format!(
                    "{starter_far} That's like, a whole world away. Why are you so far away? Why am I so far away?? Questions that could be rendered moot with a flight :)"
                ),
            }
        }
        _ => format!(
            "I'm at home. I can't tell where you are from your IP address, but I hope to see you soon!"
        ),
    };

    let mut stream = request.into_writer();
    stream_http(&mut stream, true);
    stream_header(&mut stream);

    let si = &state.read().song_info;
    let time = if si.now_playing {
        "I'm currently"
    } else {
        if let Some(ago) = &si.ago {
            &format!("{} I was", ago)
        } else {
            "a bit ago I was"
        }
    };

    let song_string = format!(
        ", and {} listening to {} by {}. This month I've been listening to lots of {}!!",
        time, si.track, si.artist, si.month_artist
    );

    let body = std::fs::read_to_string("src/main.txt")
        .unwrap()
        .replace("🎵", &song_string)
        .replace("📌", &location_string);

    let title = "PROFILE CIRCULAR REV 10";
    let title_length = title.chars().count();

    let month_abbr = chrono::Utc::now().format("%b").to_string().to_uppercase();

    let title_px = (LINE_MAX - title_length) / 2;

    let body = format!(
        "\n\nMALTED.DEV{}{month_abbr} 2025\n\n\n",
        " ".repeat(LINE_MAX - 18)
    ) + &" ".repeat(title_px - 2)
        + "┌"
        + &"─".repeat(title_length + 2)
        + "┐\n"
        + &" ".repeat(title_px - 2)
        + "│ "
        + title
        + " │\n"
        + &" ".repeat(title_px - 2)
        + "└"
        + &"─".repeat(title_length + 2)
        + "┘"
        + "\n\n"
        + &"═".repeat(LINE_MAX)
        + "\n\n"
        + &body
        + "\n\n";

    let broken_lines: String = body
        .lines()
        .map(|line| {
            let mut result = String::new();
            let mut remaining = line;

            while !remaining.is_empty() {
                if remaining.chars().count() <= LINE_MAX {
                    // Line fits, add it and we're done
                    result.push_str(remaining);
                    break;
                }

                // Line is too long, need to break it
                let chars: Vec<char> = remaining.chars().collect();

                // Look for the last space within LINE_MAX characters
                let mut break_point = LINE_MAX;
                for i in (0..LINE_MAX).rev() {
                    if chars[i] == ' ' {
                        break_point = i;
                        break;
                    }
                }

                // Convert character index back to byte index for splitting
                let byte_index = remaining
                    .char_indices()
                    .nth(break_point)
                    .map(|(byte_idx, _)| byte_idx)
                    .unwrap_or(remaining.len());

                let (current_part, rest) = remaining.split_at(byte_index);
                result.push_str(current_part.trim_end()); // Remove trailing space
                result.push('\n');

                // Skip leading spaces on the next line
                remaining = rest.trim_start();
            }

            result.push('\n');
            result
        })
        .collect();

    for (line_idx, line) in broken_lines.lines().enumerate() {
        let margin = format!(" {} |  ", if line_idx % 3 == 1 { "◯" } else { " " });
        let line = margin + &line + "\n";

        for c in line.chars() {
            stream_writable(&mut stream, &c.to_string());

            if c != '\n' && c != ' ' {
                thread::sleep(Duration::from_micros(5_000));
            }
        }
    }

    stream.write_all(b"0\r\n\r\n").unwrap(); // End
    stream.flush().unwrap();
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
