#![feature(int_roundings)]

use parking_lot::RwLock;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tiny_http::Request;
use url::Url;

use crate::base::music::SongInfo;

mod api;
mod base;

static LINE_MAX: usize = 80;

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
            "investigations" => investigation(request),
            "api" => api::api(request),
            _ => root(request, state_clone),
        });
    }

    Ok(())
}

fn investigation(request: Request) {
    let choice = request
        .url()
        .split("/")
        .nth(2)
        .expect("a second url component");

    let body = std::fs::read_to_string(format!("./src/investigations/{choice}.txt")).unwrap();

    let mut stream = request.into_writer();
    stream_http(&mut stream, true);
    stream_header(&mut stream);

    let title = "A Little Walk On The Prairie";
    let title_length = title.chars().count();

    let title_px = (LINE_MAX - title_length).div_floor(2);

    let body = "\n\nMALTED.DEV\nINVESTIGATION CIRCULAR 1                            SEP 2025\n\n\n"
        .to_owned()
        + &" ".repeat(title_px - 2)
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
        + &body;

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
            stream_line(&mut stream, &c.to_string());

            if c != '\n' || c != ' ' {
                thread::sleep(Duration::from_micros(20_000));
            }
        }
    }

    stream.write_all(b"0\r\n\r\n").unwrap(); // End
    stream.flush().unwrap();
}

fn root(request: Request, state: Arc<RwLock<State>>) {
    // let ip = "1.1.1.1"; // Replace with your target IP
    // let hops = traceroute::Traceroute::new(ip).unwrap().collect::<Vec<_>>();
    // for hop in hops {
    //     println!("{:?}", hop);
    // }

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

    let song_string = format!(", and {} listening to {} by {}", time, si.track, si.artist);

    let body = BODY_RAW
        .replace(
            "🏠",
            &request
                .remote_addr()
                .unwrap_or(&SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(127, 0, 0, 1),
                    8080,
                )))
                .ip()
                .to_string(),
        )
        .replace("🎵", &song_string);

    let is_terminal = if let Some(ua) = &request
        .headers()
        .iter()
        .find(|h| h.field.as_str() == "User-Agent")
    {
        ua.value.as_str().contains("crlus")
    } else {
        false
    };

    let mut stream = request.into_writer();

    stream_http(&mut stream, false);

    if is_terminal {
        let t = std::env::var("SECRET_2").unwrap();
        let mut bin = "".to_string();
        for character in t.clone().into_bytes() {
            bin += &format!("0{:b} ", character);
        }
        let bin = bin.replace("0", ":").replace("1", "：");
        let bin = bin + "\r\n";

        stream.write_all(bin.as_bytes()).unwrap();
        stream.write_all(b"\r\n").unwrap(); // End the header section.

        for c in std::env::var("SECRET_1").unwrap().lines() {
            stream_line(&mut stream, "\u{001b}[2J\u{001b}[H");
            stream_line(&mut stream, &c.to_string());
            stream_line(&mut stream, "\n");
            thread::sleep(Duration::from_millis(700));
        }
    } else {
        stream.write_all(b"\r\n").unwrap(); // End the header section
        stream_header(&mut stream);
        for c in body.chars() {
            stream_line(&mut stream, &c.to_string());
            thread::sleep(Duration::from_micros(1_000));
        }
    };

    stream.write_all(b"0\r\n\r\n").unwrap(); // End
    stream.flush().unwrap();
}

static BODY_RAW: &str = include_str!("./main.txt");

fn stream_line(stream: &mut Box<dyn Write + Send + 'static>, content: &str) {
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
