use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'a', long, help = "Enter add mode to add a new game")]
    pub add_mode: bool,

    #[arg(short = 'n', long, requires = "add_mode", help = "Name of the game to add")]
    pub name: Option<String>,

    #[arg(short = 'r', long, requires = "add_mode", help = "Runner for the game")]
    pub runner: Option<String>,

    #[arg(short = 'p', long, requires = "add_mode", help = "Path to the game executable")]
    pub path: Option<PathBuf>,
}

pub fn get_args() -> Args {
    Args::parse()
}
