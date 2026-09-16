use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClashBattleArena {
    pub id: i64,
    pub name: String,
    pub raw_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClashBattleTeammate {
    pub tag: String,
    pub name: String,
    pub starting_trophies: i64,
    pub trophy_change: i64,
    pub crowns: i64,
    pub king_tower_hit_points: i64,
    pub princess_towers_hit_points: Option<Vec<i64>>,
    pub elixir_leaked: f64,
    pub cards: Vec<ClashCard>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClashBattle {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub battle_time: DateTime<Utc>,
    pub arena: ClashBattleArena,
    pub team: Vec<ClashBattleTeammate>,
    pub opponent: Vec<ClashBattleTeammate>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClashCard {
    pub id: i64,
    pub level: i64,
    pub max_level: i64,
    pub name: String,
    pub rarity: String,
    pub elixir_cost: i64,
}

pub async fn last_battles() -> reqwest::Result<Vec<ClashBattle>> {
    let token = std::env::var("CLASH_ROYALE_TOKEN").expect("an env var named CLASH_ROYALE_TOKEN");
    let tag = std::env::var("CLASH_ROYALE_TAG").expect("an env var named CLASH_ROYALE_TAG");

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::try_from(format!("Bearer {token}")).unwrap(),
    );
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    let url = format!("https://api.clashroyale.com/v1/players/{tag}/battlelog");
    let res: Value = client.get(url).send().await?.json().await?;
    dbg!(&res);
    let res: Vec<ClashBattle> = serde_json::from_value(res).unwrap();
    Ok(res)
}

fn parse_timestamp(ts: String) -> Result<DateTime<Utc>, chrono::ParseError> {
    NaiveDateTime::parse_from_str(&ts, "%Y%m%dT%H%M%S%.3fZ").map(|t| t.and_utc())
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts = String::deserialize(deserializer)?;

    parse_timestamp(ts).map_err(serde::de::Error::custom)
}

pub fn top_cards(cards: &Vec<ClashCard>) -> String {
    let levels = std::collections::HashMap::from([
        ("legendary", 0),
        ("epic", 1),
        ("rare", 2),
        ("common", 3),
    ]);

    let mut cards = cards.clone();
    cards.sort_by(|a, b| {
        let ra = levels.get(a.rarity.as_str()).expect(&format!(
            "Card {:?} is not an expected card rarity: {}",
            &a, &a.rarity
        ));
        let rb = levels.get(b.rarity.as_str()).expect(&format!(
            "Card {:?} is not an expected card rarity: {}",
            &b, &b.rarity
        ));

        ra.cmp(&rb)
    });
    cards.sort_by(|a, b| b.elixir_cost.cmp(&a.elixir_cost));

    format!("{} and {}", cards[0].name, cards[1].name)
}
