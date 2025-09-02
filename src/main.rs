extern crate tiny_http;

use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let server = tiny_http::Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        thread::spawn(move || {
            let is_terminal = if let Some(ua) = &request
                .headers()
                .iter()
                .find(|h| h.field.as_str() == "User-Agent")
            {
                ua.value.as_str().contains("curl")
            } else {
                false
            };

            let response = tiny_http::Response::new_empty(tiny_http::StatusCode(200));
            let mut stream = request.into_writer();

            stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
            stream.write_all(b"Transfer-Encoding: chunked\r\n").unwrap();
            stream
                .write_all(b"Content-Type: text/plain; charset=utf-8\r\n")
                .unwrap();

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
                for c in BODY.chars() {
                    stream_line(&mut stream, &c.to_string());
                    thread::sleep(Duration::from_millis(10));
                }
            };

            stream.write_all(b"0\r\n\r\n").unwrap(); // End
            stream.flush().unwrap();
        });
    }
}

static BODY: &str = include_str!("./main.txt");

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
