use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

pub mod endpoints;
pub mod file_paths;

pub fn create_data_folder_structure() -> Result<(), std::io::Error> {
    fs::create_dir_all("data/games")?;
    fs::create_dir_all("data/characters")?;
    fs::create_dir_all("data/files")?;
    fs::create_dir_all("data/scripts")?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Game {
    id: u32,
    rows: u32,
    #[serde(rename = "titleEng")]
    title_eng: String,
    #[serde(rename = "titleJpn")]
    title_jpn: String,
    #[serde(rename = "titleJpnRoman")]
    title_jpn_roman: String,
}

fn parse_games(path: &str) -> Result<Vec<Game>, serde_json::Error> {
    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str::<Vec<Game>>(&data)
}

#[derive(Serialize, Deserialize)]
struct File {
    #[serde(rename = "engChrNames")]
    char_names_eng: Vec<String>,
    #[serde(rename = "engPlaceNames")]
    plac_names_eng: Vec<String>,
    #[serde(rename = "fname")]
    file_name: String,
    #[serde(rename = "gameId")]
    game_id: u32,
    #[serde(rename = "jpnChrNames")]
    char_names_jpn: Vec<String>,
    #[serde(rename = "jpnPlaceNames")]
    place_names_jpn: Vec<String>,
    rows: u32,
}

fn parse_file_names(
    game_id: &u32,
    paths: &file_paths::FileNames,
) -> Result<Vec<File>, serde_json::Error> {
    let data = fs::read_to_string(paths.get_file(game_id)).unwrap();
    serde_json::from_str::<Vec<File>>(&data)
}

#[tokio::main]
pub async fn download_data(
    client: &reqwest::Client,
    url: &String,
    path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp = client.get(url).send().await?.text().await?;
    let json: Value = serde_json::from_str(&resp)?;

    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &json)?;
    Ok(())
}

fn get_game_ids(paths: &file_paths::FileNames) -> Vec<u32> {
    let games = parse_games(&paths.games);

    match games {
        Ok(games) => games.iter().map(|game| game.id).collect(),
        Err(e) => {
            println!("Error parsing games: {}", e);
            vec![]
        }
    }
}

pub fn download_all_game_files(
    client: &reqwest::Client,
    urls: &endpoints::Endpoints,
    paths: &file_paths::FileNames,
) -> () {
    let game_ids = get_game_ids(&paths);

    for (i, game_id) in game_ids.iter().enumerate() {
        println!("Extracting files metadata for game {}/{}", i + 1, game_ids.len());

        let path = paths.get_file(game_id);
        let url = urls.get_files(game_id);
        let result = download_data(client, &url, &path);

        if result.is_err() {
            println!(
                "Error downloading files for game_id {}:\n{}",
                game_id,
                result.err().unwrap()
            )
        }
    }
}

pub fn download_all_game_scripts(
    client: &reqwest::Client,
    urls: &endpoints::Endpoints,
    paths: &file_paths::FileNames,
) -> () {
    let game_ids = get_game_ids(paths);

    for (i, game_id) in game_ids.iter().enumerate() {
        let files = parse_file_names(game_id, paths);
        let file_names: Vec<String> = match files {
            Ok(files) => files.iter().map(|file| file.file_name.clone()).collect(),
            Err(e) => {
                println!("Error parsing games: {}", e);
                vec![]
            }
        };

        for (j, file_name) in file_names.iter().enumerate() {
            println!(
                "Extracting scripts for game {}/{} and file {}/{}",
                i + 1,
                game_ids.len(),
                j + 1,
                file_names.len()
            );

            let path = paths.get_script(game_id, file_name);
            let url = urls.get_scripts(game_id, &file_name.to_string());
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
