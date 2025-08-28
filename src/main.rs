use trails_api_extraction::endpoints::Endpoints;
use trails_api_extraction::files_names::FileNames;
use trails_api_extraction::{
    create_data_folder_structure, download_all_game_files, download_all_game_scripts, download_data,
};
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
