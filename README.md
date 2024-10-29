# old-game-launcher

## Overview

old-game-launcher is a command-line utility designed to manage and launch your favorite games. This tool supports adding games with associated metadata and launching them using specified runners.

## Features

- Add games by specifying their name, runner, and path to the game file.
- Supports launching games with various custom runners.
- Simple command-line interface for easy use.

## Requirements

- Rust (1.50 or higher)

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/otaviocosta2110/old-game-launcher.git
    cd old-game-launcher
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Copy to path
    ```bash
    sudo cp ./target/release/old-game-launcher /usr/bin
    ```

## Usage

### Creating Runners

Runners need to be created manually to specify how your games will be launched. To create a runner, follow these steps:

1. **Locate the Runners Directory**: Navigate to the runners directory:
   ```bash
   cd ~/.local/share/old-game-launcher/runners
   ```
2. **Create a New Runner File**: Create a JSON file for your runner with the desired runner name. The filename should be `{runner_name}.json`, where `runner_name` is the name you want to use for the runner.

3. **Follow the JSON Syntax**: Ensure your JSON file follows this syntax:
    ```json
    {
      "name": "runner_name",
      "command": "runner_syntax",
      "args": ["arg1", "arg2"]
    }
    ```
Runner examples can be seen at [example_config/runners](./example_config/runners)

### Adding a Game

To add a game, use the following command:

```bash
old-game-launcher -a --name "Game Name" --runner "runner_name" --path "/path/to/game"
```
