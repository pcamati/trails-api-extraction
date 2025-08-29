use log::{error, info};
use simplelog::*;
use std::fs;
use trails_api_extraction::endpoints::Endpoints;
use trails_api_extraction::file_paths::FileNames;
use trails_api_extraction::{
    create_data_folder_structure, download_all_game_files, download_all_game_scripts, download_data,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")?;

    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

    let client = reqwest::Client::new();
    let urls = Endpoints::new();
    let paths = FileNames::new();

    match create_data_folder_structure() {
        Ok(_) => info!("Data folder structure created successfully."),
        Err(e) => error!(
            "Failed to create data folder structure.\nError message: {}",
            e
        ),
    }

    // Download games metadata
    let result = download_data(&client, &urls.get_games(), &paths.games);
    match result {
        Ok(_) => info!("Downloaded games metadata successfully."),
        Err(e) => error!("Failed to download games metadata.\nError message: {}", e),
    }

    // Download characters metadata
    let result = download_data(&client, &urls.get_chars(), &paths.chars);
    match result {
        Ok(_) => info!("Downloaded characters metadata successfully."),
        Err(e) => error!(
            "Failed to download characters metadata.\nError message: {}",
            e
        ),
    }

    // Download files metadata for each game
    download_all_game_files(&client, &urls, &paths);

    // Download all scripts metadata for dialogues of each file
    download_all_game_scripts(&client, &urls, &paths);

    Ok(())
}
