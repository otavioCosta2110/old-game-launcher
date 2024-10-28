use std::path::PathBuf;
use clap::Parser;
use std::fs;

mod commands;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub game: PathBuf,
    #[arg(short, long)]
    pub mode: String,
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn get_game_path_buf() -> PathBuf {
    let arg = Args::parse();
    arg.game
}

pub fn get_game_absolute_path() -> PathBuf {
    let game_path = get_game_path_buf();
    fs::canonicalize(game_path).unwrap_or_else(|_| {
        eprintln!("Error resolving absolute path");
        PathBuf::new()
    })
}

pub fn get_game_string() -> String {
    let arg = Args::parse();
    arg.game.to_str().unwrap().to_string()
}

pub fn get_mode() -> String {
    let arg = Args::parse();
    arg.mode
}

fn main() {
    match get_mode().as_str() {
        "wine" => commands::wine::run_game(get_game_absolute_path()),
        "native" => commands::native::run_game(get_game_absolute_path()),
        "pcsx2" => commands::pcsx2::run_game(get_game_absolute_path()),
        "cemu" => commands::cemu::run_game(get_game_absolute_path()),
        _ => eprintln!("Invalid mode"),
    }
}
