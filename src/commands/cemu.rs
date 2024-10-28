use std::process::Command;
use std::path::PathBuf;

pub fn run_game(game_path: PathBuf) {
    Command::new("cemu")
        .arg("-g")
        .arg(game_path)
        .spawn()
        .expect("Failed to run game");
}

