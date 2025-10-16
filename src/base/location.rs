use std::time::Duration;

use tiny_http::Request;
use tokio::io::AsyncWriteExt;
use url::Url;

pub fn start_image_save_job() {
    let mapbox_token = std::env::var("MAPBOX_TOKEN").unwrap();

    tokio::spawn(async move {
        loop {
            for style in ["light", "dark"] {
                let img_bytes = reqwest::get(&format!(
                                "https://api.mapbox.com/styles/v1/mapbox/{style}-v11/static/-0.1275,51.50722,11,0,0/800x400@2x?access_token={mapbox_token}"
                            )).await.unwrap().bytes().await.unwrap();

                let mut file = tokio::fs::File::create(format!("./location-{style}.png"))
                    .await
                    .unwrap();
                file.write_all(&img_bytes).await.unwrap();
            }

            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });
}

pub async fn image(request: Request) {
    let request_url = Url::parse(&format!("http://example.com{}", request.url())).unwrap();

    let last_path_segment = request_url
        .path_segments()
        .and_then(|segments| segments.last());

    let img_bytes = match last_path_segment {
        Some("light") => tokio::fs::read("./location-light.png").await.ok(),
        Some("dark") => tokio::fs::read("./location-dark.png").await.ok(),
        _ => None,
    };

    let img_bytes = match img_bytes {
        Some(b) => b,
        None => {
            let mut stream = request.into_writer();
            stream
                .write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n")
                .unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    let mut stream = request.into_writer();
    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.write_all(b"Content-Type: image/png\r\n").unwrap();
    stream
        .write_all(format!("Content-Length: {}\r\n", img_bytes.len()).as_bytes())
        .unwrap();
    stream.write_all(b"\r\n").unwrap();
    stream.write_all(&img_bytes).unwrap();
    stream.flush().unwrap();
}
