use std::fs;
use std::path::PathBuf;
use dirs::data_local_dir;

pub fn initialize_directories() -> PathBuf {
    let local_share_path = data_local_dir().unwrap().join("old-game-launcher");
    let runners_path = local_share_path.join("runners");
    let games_path = local_share_path.join("games");

    fs::create_dir_all(&runners_path).expect("Failed to create runners directory");
    fs::create_dir_all(&games_path).expect("Failed to create games directory");

    local_share_path
}

