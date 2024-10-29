mod commands;
mod config;
mod data;
mod launcher;
mod args;

fn main() {
    let local_share_path = config::initialize_directories();
    let games_map = data::load_games_map(&local_share_path);
    let game_selected = data::select_game(&games_map);
    
    let game_path = games_map.get(&game_selected).expect("Game path not found");
    let (runner_name, game_iso_path) = data::get_game_details(game_path);
    let (runner_command, runner_args) = data::get_runner_details(runner_name, &local_share_path);

    launcher::run_game(game_iso_path, runner_command, runner_args);
}

