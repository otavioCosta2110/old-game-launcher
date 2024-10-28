use std::path::PathBuf;
use std::process::Command;

pub fn run_game(game_path: PathBuf){
    Command::new("wine")
        .arg(game_path)
        .spawn()
        .expect("Failed to run game");
}
