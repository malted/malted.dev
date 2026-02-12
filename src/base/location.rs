use std::io::Write;
use std::sync::{Arc, Mutex};

use tiny_http::Request;
use url::Url;

#[derive(Debug)]
pub struct LocationInfo {
    pub lat: f64,
    pub lng: f64,
    pub city: String,
    pub state: String,
    pub country: String,
}

lazy_static::lazy_static! {
    pub static ref LOCATION_STATE: Arc<Mutex<Option<LocationInfo>>> = Arc::new(Mutex::new(None));
}

pub fn location(request: Request) {
    let request_url = Url::parse(&format!("http://example.com{}", request.url())).unwrap();

    let last_path_segment = request_url
        .path_segments()
        .and_then(|segments| segments.last());

    let is_dark = match last_path_segment {
        Some("dark") => true,
        Some("light") => false,
        _ => {
            let mut stream = request.into_writer();
            stream
                .write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n")
                .unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    let snapshot_url = crate::api::map::generate_url(is_dark);

    let mut stream = request.into_writer();
    stream
        .write_all(b"HTTP/1.1 302 Found\r\n")
        .unwrap();
    stream
        .write_all(format!("Location: {snapshot_url}\r\n").as_bytes())
        .unwrap();
    stream.write_all(b"Cache-Control: no-cache, no-store, must-revalidate\r\n").unwrap();
    stream.write_all(b"Content-Length: 0\r\n").unwrap();
    stream.write_all(b"\r\n").unwrap();
    stream.flush().unwrap();
}
