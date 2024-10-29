use clap::Parser;
use dirs::{self, data_local_dir};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod commands;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub runner: Option<String>,

    #[arg(short, long)]
    pub game: Option<String>,
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

pub fn get_runner_path(runner_name: String, local_share_path: PathBuf) -> PathBuf {
    let runner_path = local_share_path
        .join("runners")
        .join(format!("{}.json", runner_name));
    if !runner_path.exists() {
        eprintln!("Runner not found");
        std::process::exit(1);
    }
    return runner_path;
}

pub fn get_game_path(local_share_path: PathBuf) -> PathBuf {
    let arg = Args::parse();
    let game_str: String = arg.game.unwrap();

    let game_path = local_share_path
        .join("games")
        .join(format!("{}.json", game_str));
    if !game_path.exists() {
        eprintln!("Game not found");
        std::process::exit(1);
    }
    return game_path;
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

    let game_path = get_game_path(local_share_path.clone());

    let game_json_str = std::fs::read_to_string(&game_path).expect("Failed to read runner file");
    let game_json: serde_json::Value =
        serde_json::from_str(&game_json_str).expect("Failed to parse runner file");

    let game_name = game_json["runner"].as_str().unwrap();
    let runner_name = game_json["runner"].as_str().unwrap();

    let runner_path = get_runner_path(runner_name.to_string(), local_share_path);
    println!("{:?} {:?}", game_name, runner_path);

    let runner_json_str =
        std::fs::read_to_string(&runner_path).expect("Failed to read runner file");
    let runner_json: serde_json::Value =
        serde_json::from_str(&runner_json_str).expect("Failed to parse runner file");

    let game_iso_path = game_json["path"].as_str().unwrap();
    let runner_command = runner_json["command"].as_str().unwrap();

    let runner_args = runner_json
        .get("args")
        .and_then(|r| r.as_array())
        .map(|v| {
            v.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    run_game(
        game_iso_path.to_string(),
        runner_command.to_string(),
        runner_args,
    );
}

pub fn run_game(game_path: String, runner_command: String, runner_args: Vec<String>) {
    Command::new(runner_command)
        .args(runner_args)
        .arg(game_path)
        .spawn()
        .expect("Failed to run game");
}
