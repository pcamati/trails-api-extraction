use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use trails_api_extraction::endpoints::Endpoints;
use trails_api_extraction::files_names::FileNames;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_data_folder_structure()?;

    let urls = Endpoints::new();
    let files_names = FileNames::new();

    let client = reqwest::Client::new();

    // GAMES
    download_data(&client, &urls.get_games(), &files_names.games)?;

    // CHARACTERS
    download_data(&client, &urls.get_chars(), &files_names.chars)?;

    // FILES
    download_all_game_files(&client, &urls, &files_names);

    // SCRIPTS
    download_all_game_scripts(&client, &urls, &files_names);

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

#[tokio::main]
async fn download_data(
    client: &reqwest::Client,
    url: &String,
    path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending request to {}", url);
    let resp = client.get(url).send().await?.text().await?;
    println!("Response received from {}", url);
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

fn download_all_game_files(
    client: &reqwest::Client,
    urls: &Endpoints,
    files_names: &FileNames,
) -> () {
    let games = parse_games(&files_names.games);

    let game_ids: Vec<u32> = match &games {
        Ok(games) => games.iter().map(|game| game.id).collect(),
        Err(e) => {
            println!("Error parsing games: {}", e);
            vec![]
        }
    };

    for (i, game_id) in game_ids.iter().enumerate() {
        println!("Processing game {}/{}", i + 1, game_ids.len());
        let path = files_names.get_file(game_id);
        let url = urls.get_files(&game_id);
        let result = download_data(&client, &url, &path);

        if result.is_err() {
            println!(
                "Error downloading files for game_id {}:\n{}",
                game_id,
                result.err().unwrap()
            )
        }
    }
}

fn download_all_game_scripts(
    client: &reqwest::Client,
    urls: &Endpoints,
    files_names: &FileNames,
) -> () {
    let games = parse_games(&files_names.games);

    let game_ids: Vec<u32> = match &games {
        Ok(games) => games.iter().map(|game| game.id).collect(),
        Err(e) => {
            println!("Error parsing games: {}", e);
            vec![]
        }
    };

    for (i, game_id) in game_ids.iter().enumerate() {
        let files = parse_file_names(&game_id);
        let file_names: Vec<String> = match &files {
            Ok(files) => files.iter().map(|file| file.fname.clone()).collect(),
            Err(e) => {
                println!("Error parsing games: {}", e);
                vec![]
            }
        };

        for (j, file_name) in file_names.iter().enumerate() {
            println!(
                "Processing game {}/{} - file {}/{}",
                i + 1,
                game_ids.len(),
                j + 1,
                file_names.len()
            );
            let path = format!(
                "data/scripts/scripts_game_id_{}_file_name_{}.json",
                game_id, file_name
            );
            let url = urls.get_scripts(game_id, file_name.to_string());
            let result = download_data(&client, &url, &path);

            if result.is_err() {
                println!(
                    "Error downloading files for game_id {}:\n{}",
                    game_id,
                    result.err().unwrap()
                )
            }
        }
    }
}
