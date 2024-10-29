use fzf_wrapped::Fzf;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn load_games_map(local_share_path: &PathBuf) -> HashMap<String, String> {
    let mut games_map = HashMap::new();
    let games_dir = fs::read_dir(local_share_path.join("games")).expect("Failed to read games directory");

    for entry in games_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                let game_json_str = fs::read_to_string(&path).expect("Failed to read game file");
                let game_json: Value = serde_json::from_str(&game_json_str).expect("Failed to parse game JSON");

                if let Some(name) = game_json.get("name").and_then(|n| n.as_str()) {
                    games_map.insert(name.to_string(), path.display().to_string());
                }
            }
        }
    }
    games_map
}

pub fn select_game(games_map: &HashMap<String, String>) -> String {
    let game_names: Vec<String> = games_map.keys().cloned().collect();
    
    if game_names.is_empty() {
        eprintln!("No games found in the directory");
        std::process::exit(1);
    }

    // Instantiate `fzf` just before retrieving output
    let mut fzf = Fzf::default();
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(game_names).expect("Failed to add items to fzf");
    
    let game_selected = fzf.output().expect("Failed to get the user's selection");

    game_selected
}

pub fn get_game_details(game_path: &str) -> (String, String) {
    let game_json_str = fs::read_to_string(game_path).expect("Failed to read game file");
    let game_json: Value = serde_json::from_str(&game_json_str).expect("Failed to parse game JSON");
    let runner_name = game_json["runner"].as_str().unwrap().to_string();
    let game_iso_path = game_json["path"].as_str().unwrap().to_string();

    (runner_name, game_iso_path)
}

pub fn get_runner_details(runner_name: String, local_share_path: &PathBuf) -> (String, Vec<String>) {
    let runner_path = local_share_path.join("runners").join(format!("{}.json", runner_name));
    if !runner_path.exists() {
        eprintln!("Runner not found");
        std::process::exit(1);
    }
    let runner_json_str = fs::read_to_string(&runner_path).expect("Failed to read runner file");
    let runner_json: Value = serde_json::from_str(&runner_json_str).expect("Failed to parse runner JSON");

    let runner_command = runner_json["command"].as_str().unwrap().to_string();
    let runner_args = runner_json.get("args")
        .and_then(|r| r.as_array())
        .map(|v| v.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    (runner_command, runner_args)
}

