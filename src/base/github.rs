use reqwest::header::HeaderValue;

pub async fn update_github_location(location: &str) {
    let github_token = std::env::var("GITHUB_PROFILE_UPDATE_TOKEN")
        .expect("an env var named GITHUB_PROFILE_UPDATE_TOKEN");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {github_token}").parse().unwrap(),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2026-03-10"),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("malted.dev"),
    );

    let res: serde_json::Value = reqwest::Client::new()
        .patch("https://api.github.com/user")
        .json(&serde_json::json!({ "location": location }))
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    dbg!(res);
}
