use reqwest::Error;

const BASE_URL: &str = "https://trailsinthedatabase.com";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(BASE_URL)
        .header("User-Agent", "my-rust-client")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", resp);
    Ok(())
}
