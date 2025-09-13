use args::get_args;
use add_game::add_game;
use std::process::Command;

mod args;
mod add_game;
mod config;
mod data;

fn main() {
    let args = get_args();

    if args.add_mode {
        let name = args.name.expect("Game name is required in add mode");
        let runner = args.runner.expect("Runner is required in add mode");
        let path = args.path.expect("Path is required in add mode");

        add_game(name, path, runner);
        println!("Game added successfully.");
        return;
    }

    let local_share_path = config::initialize_directories();
    let games_map = data::load_games_map(&local_share_path);
    let game_selected = data::select_game(&games_map);

    let game_path = games_map.get(&game_selected).expect("Game path not found");
    let (runner_name, game_iso_path) = data::get_game_details(game_path);
    let (runner_command, runner_args) = data::get_runner_details(runner_name, &local_share_path);

    run_game(game_iso_path, runner_command, runner_args);
}

pub fn run_game(game_path: String, runner_command: String, runner_args: Vec<String>) {

    println!("{}", game_path);
    if runner_command.is_empty() {
        match Command::new(game_path).spawn() {
            Ok(child) => {
                println!("Game started successfully with PID: {}", child.id());
            }
            Err(e) => {
                eprintln!("Error while running the game directly: {}", e);
            }
        }
        return;
    }

    Command::new(runner_command)
        .args(runner_args)
        .arg(game_path)
        .spawn()
        .expect("Failed to run game");
    }
