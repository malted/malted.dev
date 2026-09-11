use plist::{Dictionary, Value};
use serde::Serialize;
use std::ops::Index;

#[derive(Debug, Serialize)]
struct ReadingListItem {
    url: String,
    title: Option<String>,
    date_added: Option<String>,
}

impl ReadingListItem {
    fn extract() -> Vec<Self> {
        let list = Value::from_file("/Users/malted/Library/Safari/Bookmarks.plist")
            .expect("failed to read ~/Library/Safari/Bookmarks.plist");
        let list = list
            .as_dictionary()
            .expect("failed to convert Bookmarks.plist to a plist::Dictionary");

        list.get("Children")
            .expect("Failed to find 'Children' element in Bookmarks.plist dict")
            .as_array()
            .expect("Failed to convert 'Children' element in Bookmarks.plist to array")
            .into_iter()
            .find(|i| {
                i.as_dictionary().unwrap().get("Title").unwrap().as_string()
                    == Some("com.apple.ReadingList")
            })
            .expect("Failed to find com.apple.ReadingList")
            .as_dictionary()
            .expect("Failed to convert com.apple.ReadingList to a dictionary")
            .get("Children")
            .expect("Failed to find Children in com.apple.ReadingList dictionary")
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| x.as_dictionary().unwrap())
            .map(|c| {
                let url = c.get("URLString").unwrap().as_string().unwrap().to_string();
                let rl = c.get("ReadingList").unwrap().as_dictionary().unwrap();
                let title = rl
                    .get("PreviewText")
                    .map(|s| s.as_string())
                    .flatten()
                    .map(|s| s.to_string());
                let date_added = rl
                    .get("DateAdded")
                    .unwrap()
                    .as_string()
                    .map(|s| s.to_string());

                ReadingListItem {
                    title,
                    url,
                    date_added,
                }
            })
            .collect()
    }
}

fn main() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60 * 60));

        let key = match std::fs::read_to_string("/Users/malted/.config/maltedd") {
            Ok(k) => k,
            Err(_) => continue,
        };

        let items = ReadingListItem::extract();

        let client = reqwest::blocking::Client::new();
        client
            .post("https://malted.dev/???")
            .json(&items)
            .header("Authorization", key.trim())
            .send();
    }
}
