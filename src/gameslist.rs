use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub struct Game {
    pub name: String,
    pub steam_uri: Option<String>,
}

pub fn gameslist(guild_id: String) -> Vec<Game> {
    let path = file_path(guild_id);
    read_gameslist_from_file(path).unwrap()
}

#[allow(dead_code)]
pub fn create_file(_guild_id: String) {
}

fn file_path(guild_id: String) -> String {
    // Make sure to save this to some file outside of the project
    // e.g. ~/.local/utilibot/data/{}/gameslist.json
    // Use `dirs`
    format!("src/data/{}/gameslist.json", guild_id)
}

fn read_gameslist_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Game>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let games = serde_json::from_reader(reader)?;

    Ok(games)
}
