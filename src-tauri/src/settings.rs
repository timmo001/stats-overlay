use log::info;
use serde::{Deserialize, Serialize};

use crate::shared::{get_data_path, restart_app};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub autostart: bool,
    pub log_level: String,
}

fn create_settings() -> Settings {
    // Create settings from {config_path}\settings.json
    let settings = Settings {
        autostart: false,
        log_level: "INFO".to_string(),
    };

    // Create settings string
    let settings_string = serde_json::to_string(&settings).unwrap();

    info!("Creating settings file: {}", settings_string);

    // Write settings to {config_path}\settings.json
    let settings_path = format!("{}/settings.json", get_data_path());
    std::fs::write(settings_path, settings_string).unwrap();

    settings
}

#[tauri::command]
pub fn get_settings() -> Settings {
    // Read settings from {config_path}\settings.json
    let settings_path = format!("{}/settings.json", get_data_path());
    if !std::path::Path::new(&settings_path).exists() {
        return create_settings();
    }

    let settings = std::fs::read_to_string(settings_path);
    if settings.is_err() {
        return create_settings();
    }
    let settings = serde_json::from_str(&settings.unwrap());
    if settings.is_err() {
        return create_settings();
    }

    settings.unwrap()
}

#[tauri::command]
pub fn update_settings(settings: Settings) -> Result<Settings, String> {
    // Get current settings to compare
    let current_settings = get_settings();

    // Write settings to {config_path}\settings.json
    let new_settings_string = serde_json::to_string(&settings).unwrap();

    // Check if any settings have changed
    let current_settings_string = serde_json::to_string(&current_settings).unwrap();
    if current_settings_string != new_settings_string {
        info!("Settings changed: {}", new_settings_string);

        let settings_path = format!("{}/settings.json", get_data_path());
        std::fs::write(settings_path, new_settings_string).unwrap();

        // Restart the app to apply the new settings
        restart_app();
    }

    Ok(settings)
}
