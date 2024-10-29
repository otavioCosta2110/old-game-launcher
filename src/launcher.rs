use std::process::Command;

pub fn run_game(game_path: String, runner_command: String, runner_args: Vec<String>) {
    Command::new(runner_command)
        .args(runner_args)
        .arg(game_path)
        .spawn()
        .expect("Failed to run game");
}

