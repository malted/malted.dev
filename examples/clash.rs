#[path = "../src/base/clash.rs"]
#[allow(dead_code)]
mod clash;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let battle = clash::last_battles().await;
    dbg!(battle);
}
