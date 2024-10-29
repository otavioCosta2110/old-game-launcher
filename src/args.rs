use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub runner: Option<String>,

    #[arg(short, long)]
    pub game: Option<String>,
}

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
