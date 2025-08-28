use trails_api_extraction::endpoints::Endpoints;
use trails_api_extraction::file_paths::FileNames;
use trails_api_extraction::{
    create_data_folder_structure, download_all_game_files, download_all_game_scripts, download_data,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let urls = Endpoints::new();
    let paths = FileNames::new();

    create_data_folder_structure()?;

    // Download games metadata
    download_data(&client, &urls.get_games(), &paths.games)?;

    // Download characters metadata
    download_data(&client, &urls.get_chars(), &paths.chars)?;

    // Download files metadata for each game
    download_all_game_files(&client, &urls, &paths);

    // Download all scripts metadata for dialogues of each file
    download_all_game_scripts(&client, &urls, &paths);

    Ok(())
}
