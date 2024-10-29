use serde_json::json;
use std::fs;
use std::path::PathBuf;

use crate::config::initialize_directories;
use crate::data::get_games_path;

pub fn add_game(name: String, path: PathBuf, runner: String) -> PathBuf {
    let abs_path = path.canonicalize().expect("Failed to get absolute path");
    let game_json = json!({
        "name": name,
        "path": abs_path,
        "runner": runner
    });

    let local_share_path = initialize_directories();

    let game_json_str =
        serde_json::to_string_pretty(&game_json).expect("Failed to serialize game JSON");
    let game_path = get_games_path(local_share_path).join(format!("{}.json", name));
    fs::write(&game_path, game_json_str).expect("Failed to write game file");

    game_path
}
