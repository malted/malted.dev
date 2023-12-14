use parking_lot::Mutex;
use rand::prelude::{thread_rng, SliceRandom};
use std::sync::Arc;

pub fn random_greeting() -> &'static str {
    let greetings: Vec<&str> = include_str!("../../include/greetings.txt")
        .split("\n")
        .collect();
    greetings.choose(&mut thread_rng()).unwrap()
}

pub fn random_link() -> &'static str {
    let links: Vec<&str> = include_str!("../../include/links.txt")
        .split("\n\n")
        .map(|x| x.split("\n").nth(1).unwrap())
        .collect();
    links.choose(&mut thread_rng()).unwrap()
}

// pub fn location_section(malted_state: Arc<Mutex<Option<crate::MaltedState>>>) -> String {
//     let tmp = malted_state.lock();
//     if let (Some(ref ts), Some(ref loc)) = (tmp.timestamp, tmp.location) {
//         format!("\nAs of {ts} ago, I'm {loc}\n")
//     } else {
//     }
// }

pub fn justify(should_justify: bool, text: &[&str], max_length: usize) -> String {
    text.iter()
        .map(|x| {
            if should_justify {
                format!("{:^width$}", x, width = max_length)
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
