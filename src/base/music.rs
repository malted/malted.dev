use chrono::DateTime;
use chrono_humanize::HumanTime;
use std::env::var;

#[derive(Debug)]
pub struct SongInfo {
    pub now_playing: bool,
    pub track: String,
    pub artist: String,
    pub ago: Option<String>,
    pub month_artist: String,
}

static LASTFM_URL_BASE: &str = "https://ws.audioscrobbler.com/2.0";
static LASTFM_USERNAME: &str = "ma1ted";

pub async fn now_playing() -> Result<SongInfo, Box<dyn std::error::Error + Send + Sync>> {
    let lastfm_api_key = var("LASTFM_API_KEY")?;

    let (now_response, month_artist) = tokio::join!(fetch_now_playing(&lastfm_api_key), fetch_month_artist(&lastfm_api_key));
    let now_response = now_response?;
    let month_artist = month_artist?;

    let now_playing = now_response
        .pointer("/recenttracks/track/0/@attr/nowplaying")
        .map(|o| o.as_str())
        .flatten()
        == Some("true");

    let track = now_response
        .pointer("/recenttracks/track/0/name")
        .map(|o| o.as_str())
        .flatten()
        .ok_or("track not a valid str")?
        .to_string();
    let artist = now_response
        .pointer("/recenttracks/track/0/artist/#text")
        .map(|o| o.as_str())
        .flatten()
        .ok_or("artist not a valid str")?
        .to_string();

    // I LOVE RUST RAHHHHH
    let ago = now_response
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
        month_artist,
    })
}

async fn fetch_now_playing(lastfm_api_key: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    Ok(reqwest::get(format!("{LASTFM_URL_BASE}/?method=user.getrecenttracks&user={LASTFM_USERNAME}&api_key={lastfm_api_key}&format=json&limit=1"))
        .await?
        .json()
        .await?)
}

async fn fetch_month_artist(lastfm_api_key: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    Ok(reqwest::get(format!(
        "{LASTFM_URL_BASE}/?method=user.gettopartists&user={LASTFM_USERNAME}&api_key={lastfm_api_key}&format=json&period=1month&limit=1"
    ))
    .await?
    .json::<serde_json::Value>()
    .await?
    .pointer("/topartists/artist/0/name")
    .map(|o| o.as_str())
    .flatten()
    .ok_or("artist not a valid str")?
    .to_string())
}
