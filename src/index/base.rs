use std::sync::Arc;

use super::*;
use crate::MaltedState;
use parking_lot::RwLock;
use rocket::{
    response::{stream::TextStream, Redirect},
    tokio::time::{self, Duration},
    State,
};
use stats::{hcb_stats, replit_stats};
use thousands::Separable;

use malted_dev::ExtendableIterator;

#[get("/???")]
pub fn random_site() -> Redirect {
    Redirect::temporary(random_link())
}

#[get("/")]
pub async fn index(
    malted_state: &State<RwLock<MaltedState>>,
    req_info: RequesterInfo<'_>,
) -> TextStream![String] {
    let short_interval = Duration::from_millis(4);
    let long_interval = Duration::from_millis(50);

    let (tx_hcb, rx_hcb) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        let _ = tx_hcb.send(hcb_stats().await.ok());
    });
    let mut rx_hcb = Some(rx_hcb);

    let (tx_replit, rx_replit) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        let _ = tx_replit.send(replit_stats().await.ok());
    });
    let mut rx_replit = Some(rx_replit);

    let body = include_str!("../../include/index.txt");
    let mut body = ExtendableIterator::new(body.chars().collect());

    let location = location_section(malted_state, &req_info);
    let battery_message = battery_message(malted_state);

    let agent = req_info.user_agent.map(|ua| ua.to_string());

    TextStream! {
        let mut interval = time::interval(long_interval);

        let mut line_max = 60;
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
        yield "​".repeat(1_025).to_string();

        while let Some(char) = body.next() {
            match char {
                '🐢' => interval = time::interval(long_interval),
                '🐇' => interval = time::interval(short_interval),
                '👋' => body.push_str(random_greeting()),
                '💵' => {
                    let msg = rx_hcb.take().expect("only one 💵 in index.txt").await.ok().flatten().map(|d| {
                        let amount = (d.transactions_volume as f64 / 100.0).separate_with_commas();

                        let relative = std::time::Duration::from_secs(
                            (chrono::Utc::now() - d.date)
                                .to_std()
                                .expect("Time should be positive")
                                .as_secs(),
                        );
                        let ago = humantime::format_duration(relative);

                        // format!("${amount} as of {ago} ago")
                        format!("${amount}")

                    }).unwrap_or("over $25M".into());
                    body.push_str(&msg);
                }
                '📁' => {
                    let msg = rx_replit.take().expect("only one 📁 in index.txt").await.ok().flatten().map(|d| d.file_count.separate_with_commas()).unwrap_or("over 300,000".into());
                    body.push_str(&msg);
                }
                '📍' => body.push_str(&location),
                '🔋' => body.push_str(&battery_message),
                '\n' => {
                    yield char.to_string();
                    line_length = 0;
                }
                _ => {
                    if line_length >= line_max && char == ' ' {
                        yield '\n'.to_string();
                        line_length = 0;
                    } else {
                        yield char.to_string();
                        line_length += 1;
                        interval.tick().await;
                    }
                },
            }
        }

        // macro_rules! typewr {
        //     ($text:expr) => {



        //         // while let Some(item) = iter.next() {
        //             //     print!("{}", item);

        //             //     if item == 'b' {
        //             //         iter.push('1');
        //             //         iter.push('2');
        //             //     }
        //             // }

        //         let chars = $text.chars();
        //         for c in chars {
        //             if c == '🐢' {
        //                 ;
        //                 continue;
        //             } else if c == '🐇' {
        //                 interval = time::interval(default_interval);
        //                 continue;
        //             } else {
        //                 if c == '\n' {
        //                     yield c.to_string();
        //                     line_length = 0;
        //                 } else if line_length >= line_max && c == ' ' {
        //                     yield '\n'.to_string();
        //                     line_length = 0;
        //                 } else {
        //                     line_length += 1;
        //                     yield c.to_string();
        //                 }
        //             }

        //             if c != '\n' && c != '\r' && c != '\t' && c != ' ' {
        //                 interval.tick().await;
        //             }
        //         }
        //     };
        // }

        // typewr!(body);

        // typewr!(location);

        // typewr!(contact);

        // interval = time::interval(long_interval);
        // typewr!(epilogue);
    }
}

#[get("/raytrace.sql")]
pub async fn raytrace() -> &'static str {
    include_str!("../../include/raytrace.sql")
}
