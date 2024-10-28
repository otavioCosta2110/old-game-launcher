use std::path::PathBuf;
use clap::Parser;
use std::fs;
use dirs::{self, data_local_dir};

mod commands;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub runner: String,
}

pub fn get_args() -> Args {
    Args::parse()
}

// pub fn get_game_path_buf() -> PathBuf {
//     let arg = Args::parse();
//     arg.game
// }
//
// pub fn get_game_absolute_path() -> PathBuf {
//     let game_path = get_game_path_buf();
//     fs::canonicalize(game_path).unwrap_or_else(|_| {
//         eprintln!("Error resolving absolute path");
//         PathBuf::new()
//     })
// }
//
// pub fn get_game_string() -> String {
//     let arg = Args::parse();
//     arg.game.to_str().unwrap().to_string()
// }

pub fn get_runner_path(local_share_path: PathBuf) -> PathBuf {
    let arg = Args::parse();
    let runner_str: String = arg.runner;

    let runner_path = local_share_path.join("runners").join(format!("{}.json", runner_str));
    if !runner_path.exists() {
        eprintln!("Runner not found");
        std::process::exit(1);
    }
    return runner_path;
}

fn main() {
    let local_share_path = data_local_dir().unwrap().join("old-game-launcher");
    if !local_share_path.exists() {
        let local_share_path_runners = local_share_path.join("runners");
        let local_share_path_games = local_share_path.join("games");
        fs::create_dir_all(&local_share_path).expect("Failed to create local share directory");
        fs::create_dir_all(&local_share_path_runners).expect("Failed to create runners directory");
        fs::create_dir_all(&local_share_path_games).expect("Failed to create games directory");
    }

    let runner_path = get_runner_path(local_share_path);

    let json_str = std::fs::read_to_string(&runner_path).expect("Failed to read runner file");
    let json: serde_json::Value = serde_json::from_str(&json_str).expect("Failed to parse runner file");

    let runner_name = json["name"].as_str().unwrap();

    println!("{:?}", runner_name);
}
