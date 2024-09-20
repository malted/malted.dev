pub mod utils;
pub use utils::*;

pub mod base;
pub use base::*;

mod stats {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct HcbStats {
        pub date: DateTime<Utc>,
        pub transactions_volume: u64,
        pub transactions_count: u64,
        pub currently_online: u64,
    }

    pub async fn hcb_stats() -> Result<HcbStats, Box<dyn std::error::Error>> {
        Ok(reqwest::get("https://hcb.hackclub.com/stats")
            .await?
            .json()
            .await?)
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ReplitStats {
        pub file_count: u64,
        pub repl_count: u64,
    }

    pub async fn replit_stats() -> Result<ReplitStats, Box<dyn std::error::Error>> {
        Ok(reqwest::get("http://takeout.hackclub.com/stats")
            .await?
            .json()
            .await?)
    }
}
