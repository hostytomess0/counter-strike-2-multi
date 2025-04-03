use std::sync::{Arc, Mutex};
use tokio::task;
use reqwest::Client;

struct Config {
    aimbot_enabled: bool,
    esp_enabled: bool,
    speedhack_enabled: bool,
}

struct GameState {
    config: Arc<Mutex<Config>>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            config: Arc::new(Mutex::new(Config {
                aimbot_enabled: false,
                esp_enabled: false,
                speedhack_enabled: false,
            })),
        }
    }

    fn toggle_aimbot(&self) {
        let mut config = self.config.lock().unwrap();
        config.aimbot_enabled = !config.aimbot_enabled;
    }

    fn toggle_esp(&self) {
        let mut config = self.config.lock().unwrap();
        config.esp_enabled = !config.esp_enabled;
    }

    fn toggle_speedhack(&self) {
        let mut config = self.config.lock().unwrap();
        config.speedhack_enabled = !config.speedhack_enabled;
    }
}

#[tokio::main]
async fn main() {
    let game_state = GameState::new();
    let client = Client::new();

    task::spawn(async move {
        loop {
            let config = game_state.config.lock().unwrap();
            if config.aimbot_enabled {
                // Aimbot logic here
            }
            if config.esp_enabled {
                // ESP logic here
            }
            if config.speedhack_enabled {
                // Speedhack logic here
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    // Simulate user input to toggle features
    game_state.toggle_aimbot();
    game_state.toggle_esp();
    game_state.toggle_speedhack();

    let _ = client.get("http://localhost:8080").send().await;
}