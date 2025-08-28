use serde::{Deserialize, Serialize, ser};
use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_data_folder_structure()?;

    let urls = Endpoints::new();

    let client = reqwest::Client::new();

    // GAMES
    let path = "data/games/games.json".to_string();
    download_data(&client, urls.get_games(), path)?;

    // CHARACTERS
    let path = "data/characters/characters.json".to_string();
    download_data(&client, urls.get_chars(), path)?;

    // FILES
    let game_id = 1;
    let path = format!("data/files/files_game_id_{}.json", game_id);
    download_data(&client, urls.get_files(&game_id), path)?;

    // SCRIPTS
    let game_id = 1;
    let file_name = "c0100";
    let path = format!(
        "data/scripts/scripts_game_id_{}_file_name_{}.json",
        game_id, file_name
    );
    download_data(
        &client,
        urls.get_scripts(game_id, file_name.to_string()),
        path,
    )?;

    let path = "data/games/games.json".to_string();
    let games = parse_games(&path);

    let game_ids: Vec<u32> = match &games {
        Ok(games) => games.iter().map(|game| game.id).collect(),
        Err(e) => {
            println!("Error parsing games: {}", e);
            vec![]
        }
    };

    for (i, game_id) in game_ids.iter().enumerate() {
        println!("Processing game {}/{}", i + 1, game_ids.len());
        let path = format!("data/files/files_game_id_{}.json", &game_id);
        let url = urls.get_files(&game_id);
        download_data(&client, url, path)?;
    }

    let game_id = 1;
    let files = parse_file_names(&game_id);
    let file_names: Vec<String> = match &files {
        Ok(files) => files.iter().map(|file| file.fname.clone()).collect(),
        Err(e) => {
            println!("Error parsing games: {}", e);
            vec![]
        }
    };

    for (i, file_name) in file_names.iter().enumerate() {
        println!("Processing game {game_id} - file {}/{}", i + 1, file_names.len());
        let path = format!(
            "data/scripts/scripts_game_id_{}_file_name_{}.json",
            game_id, file_name
        );
        let url = urls.get_scripts(game_id, file_name.to_string());
        download_data(&client, url, path)?;
    }

    Ok(())
}

fn create_data_folder_structure() -> Result<(), std::io::Error> {
    println!("Creating data folder structure...");
    fs::create_dir_all("data/games")?;
    fs::create_dir_all("data/characters")?;
    fs::create_dir_all("data/files")?;
    fs::create_dir_all("data/scripts")?;
    println!("Data folder structure created.");
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

    fn get_files(&self, game_id: &u32) -> String {
        let url = format!("{}{}", self.base, self.files);
        url.replace("{game_id}", &game_id.to_string())
    }

    fn get_scripts(&self, game_id: u32, file_name: String) -> String {
        let mut url = format!("{}{}", self.base, self.scripts);
        url = url.replace("{game_id}", &game_id.to_string());
        url.replace("{file_name}", &file_name.to_string())
    }
}

#[tokio::main]
async fn download_data(
    client: &reqwest::Client,
    url: String,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending request to {}", &url);
    let resp = client.get(&url).send().await?.text().await?;
    println!("Response received from {}", &url);
    let json: Value = serde_json::from_str(&resp)?;

    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &json)?;

    Ok(())
}

fn parse_games(path: &str) -> Result<Vec<Game>, serde_json::Error> {
    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str::<Vec<Game>>(&data)
}

fn parse_file_names(game_id: &u32) -> Result<Vec<File>, serde_json::Error> {
    let path = format!("data/files/files_game_id_{}.json", game_id);
    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str::<Vec<File>>(&data)
}

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    id: u32,
    rows: u32,
    titleEng: String,
    titleJpn: String,
    titleJpnRoman: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct File {
    engChrNames: Vec<String>,
    engPlaceNames: Vec<String>,
    fname: String,
    gameId: u32,
    jpnChrNames: Vec<String>,
    jpnPlaceNames: Vec<String>,
    rows: u32,
}
