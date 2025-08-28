pub struct FileNames {
    pub games: String,
    pub chars: String,
    files: String,
    scripts: String,
}

impl FileNames {
    // Defines paths where data will be stored
    // Brackets serve as placeholders
    pub fn new() -> Self {
        Self {
            games: String::from("data/games/games.json"),
            chars: String::from("data/characters/characters.json"),
            files: String::from("data/files/files_game_id_{game_id}.json"),
            scripts: String::from(
                "data/scripts/scripts_game_id_{game_id}_file_name_{file_name}.json",
            ),
        }
    }

    pub fn get_file(&self, game_id: &u32) -> String {
        self.files.replace("{game_id}", &game_id.to_string())
    }

    pub fn get_script(&self, game_id: &u32, file_name: &String) -> String {
        self.scripts
            .replace("{game_id}", &game_id.to_string())
            .replace("{file_name}", &file_name)
    }
}
