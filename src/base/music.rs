use chrono::DateTime;
use chrono_humanize::HumanTime;
use std::env::var;

#[derive(Debug)]
pub struct SongInfo {
    pub now_playing: bool,
    pub track: String,
    pub artist: String,
    pub ago: Option<String>,
}

pub async fn now_playing() -> Result<SongInfo, Box<dyn std::error::Error>> {
    let lastfm_api_key = var("LASTFM_API_KEY")?;

    let access_token_response: serde_json::Value = reqwest::get(format!("https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user=ma1ted&api_key={lastfm_api_key}&format=json&limit=1"))
        .await?
        .json()
        .await?;

    let now_playing = access_token_response
        .pointer("/recenttracks/track/0/@attr/nowplaying")
        .map(|o| o.as_str())
        .flatten()
        == Some("true");

    let track = access_token_response
        .pointer("/recenttracks/track/0/name")
        .map(|o| o.as_str())
        .flatten()
        .ok_or("track not a valid str")?
        .to_string();
    let artist = access_token_response
        .pointer("/recenttracks/track/0/artist/#text")
        .map(|o| o.as_str())
        .flatten()
        .ok_or("artist not a valid str")?
        .to_string();

    // I LOVE RUST RAHHHHH
    let ago = access_token_response
        .pointer("/recenttracks/track/0/date/uts")
        .map(|o| {
            o.as_str().map(|o2| {
                o2.parse::<i64>().ok().map(|o3| {
                    DateTime::from_timestamp(o3, 0).map(|o4| format!("{}", HumanTime::from(o4)))
                })
            })
        })
        .flatten()
        .flatten()
        .flatten();

    Ok(SongInfo {
        now_playing,
        track,
        artist,
        ago,
    })
}
