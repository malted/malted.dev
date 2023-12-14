use rand::prelude::{thread_rng, SliceRandom};

pub fn random_greeting() -> &'static str {
    let greetings: Vec<&str> = include_str!("../include/greetings.txt")
        .split("\n")
        .collect();
    greetings.choose(&mut thread_rng()).unwrap()
}

pub fn random_link() -> &'static str {
    let links: Vec<&str> = include_str!("../include/links.txt")
        .split("\n\n")
        .map(|x| x.split("\n").nth(1).unwrap())
        .collect();
    links.choose(&mut thread_rng()).unwrap()
}

pub fn location_section(timestamp: Option<String>, location: Option<String>) -> String {
    if let (Some(ref ts), Some(ref loc)) = (timestamp, location) {
        format!("\nAs of {ts} ago, I'm {loc}\n")
    } else {
        "\nHm. I was going to tell you where I am, but apparently my server doesn't know, or doesn't want to tell you.\n\n".to_string()
    }
}

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
