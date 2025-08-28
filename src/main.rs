use serde_json::Value;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_data_folder_structure()?;

    let urls = Endpoints::new();

    let client = reqwest::Client::new();

    // GAMES
    let resp = client.get(urls.get_games()).send().await?.text().await?;
    let json = convert_to_json(&resp)?;

    let file = fs::File::create("data/games/games.json")?;
    serde_json::to_writer_pretty(file, &json)?;

    // CHARACTERS
    let resp = client.get(urls.get_chars()).send().await?.text().await?;
    let json = convert_to_json(&resp)?;

    let file = fs::File::create("data/characters/characters.json")?;
    serde_json::to_writer_pretty(file, &json)?;

    // FILES
    let game_id = 1;
    let resp = client
        .get(urls.get_files(game_id))
        .send()
        .await?
        .text()
        .await?;
    let json = convert_to_json(&resp)?;

    let path = format!("data/files/files_game_id_{}.json", game_id);
    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &json)?;

    // SCRIPTS
    let game_id = 1;
    let file_name = "c0100";
    let resp = client
        .get(urls.get_scripts(game_id, file_name.to_string()))
        .send()
        .await?
        .text()
        .await?;
    let json = convert_to_json(&resp)?;

    let path = format!(
        "data/scripts/scripts_game_id_{}_file_name_{}.json",
        game_id, file_name
    );
    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &json)?;

    Ok(())
}

fn convert_to_json(string: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(&string)
}

fn create_data_folder_structure() -> Result<(), std::io::Error> {
    fs::create_dir_all("data/games")?;
    fs::create_dir_all("data/characters")?;
    fs::create_dir_all("data/files")?;
    fs::create_dir_all("data/scripts")?;
    Ok(())
}

struct Endpoints {
    base: String,
    games: String,
    chars: String,
    files: String,
    scripts: String,
}

impl Endpoints {
    fn new() -> Self {
        Self {
            base: String::from("https://trailsinthedatabase.com"),
            games: String::from("/api/game"),
            chars: String::from("/api/chr"),
            files: String::from("/api/file?game_id={game_id}"),
            scripts: String::from("/api/script/detail/{game_id}/{file_name}"),
        }
    }

    fn get_games(&self) -> String {
        format!("{}{}", self.base, self.games)
    }

    fn get_chars(&self) -> String {
        format!("{}{}", self.base, self.chars)
    }

    fn get_files(&self, game_id: u32) -> String {
        let url = format!("{}{}", self.base, self.files);
        url.replace("{game_id}", &game_id.to_string())
    }

    fn get_scripts(&self, game_id: u32, file_name: String) -> String {
        let mut url = format!("{}{}", self.base, self.scripts);
        url = url.replace("{game_id}", &game_id.to_string());
        url.replace("{file_name}", &file_name.to_string())
    }
}
