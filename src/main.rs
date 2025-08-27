use serde_json::Value;
use std::fs::File;

const BASE_URL: &str = "https://trailsinthedatabase.com";
const GAME_ENDPOINT: &str = "/api/game";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::from(BASE_URL);
    url.push_str(GAME_ENDPOINT);

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "my-rust-client")
        .send()
        .await?
        .text()
        .await?;

    let json = convert_to_json(&resp)?;

    let file = File::create("games.json")?;
    serde_json::to_writer_pretty(file, &json)?;

    println!("{}", resp);
    Ok(())
}

fn convert_to_json(string: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(&string)
}
