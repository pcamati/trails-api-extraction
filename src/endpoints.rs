pub struct Endpoints {
    base: String,
    games: String,
    chars: String,
    files: String,
    scripts: String,
}

impl Endpoints {
    // Defines each relevant endpoint
    // Brackets serve as placeholders
    pub fn new() -> Self {
        Self {
            base: String::from("https://trailsinthedatabase.com"),
            games: String::from("/api/game"),
            chars: String::from("/api/chr"),
            files: String::from("/api/file?game_id={game_id}"),
            scripts: String::from("/api/script/detail/{game_id}/{file_name}"),
        }
    }

    pub fn get_games(&self) -> String {
        format!("{}{}", self.base, self.games)
    }

    pub fn get_chars(&self) -> String {
        format!("{}{}", self.base, self.chars)
    }

    pub fn get_files(&self, game_id: &u32) -> String {
        let url = format!("{}{}", self.base, self.files);
        url.replace("{game_id}", &game_id.to_string())
    }

    pub fn get_scripts(&self, game_id: &u32, file_name: &String) -> String {
        let mut url = format!("{}{}", self.base, self.scripts);
        url = url.replace("{game_id}", &game_id.to_string());
        url.replace("{file_name}", &file_name.to_string())
    }
}
