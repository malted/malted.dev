use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommitResult {
    total_count: i64,
    incomplete_results: bool,
    items: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    sha: String,
    html_url: String,
}

pub async fn fetch_commits() -> Result<Vec<Commit>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut page = 1;

    loop {
        let res = client
            .get(&format!(
                "https://api.github.com/search/commits?q=author:malted&page={page}&per_page=100"
            ))
            .header("User-Agent", "malted.dev")
            .bearer_auth(std::env::var("github_pat").expect("a github_pat env var"))
            .send()
            .await?
            .json::<CommitResult>()
            .await?;
    }

    log::info!("You've made {} commits", res.total_count);

    for commit in res.items {
        println!("{}", commit.sha);
    }

    Ok(vec![])
}
