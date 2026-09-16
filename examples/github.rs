use crate::github::update_github_location;

#[path = "../src/base/github.rs"]
#[allow(dead_code)]
mod github;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let location = "San Francisco";

    let res = update_github_location(location).await;
    dbg!(res);
}
