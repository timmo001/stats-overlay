use log::{debug, info};
use platform_dirs::AppDirs;

pub fn get_data_path() -> String {
    // Get data path from {localappdata}\timmo001\stats-overlay
    let app_dirs = AppDirs::new(Some("timmo001"), true).unwrap();
    let data_path = app_dirs.data_dir.to_str().unwrap().to_string();
    debug!("Data path: {}", data_path);

    let path = format!("{}/stats-overlay", data_path);

    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn restart_app() {
    // Don't restart in debug mode
    if cfg!(debug_assertions) {
        info!("Not restarting: in debug mode");
        return;
    }

    info!("Restarting app");

    // Restart the app
    std::process::Command::new(std::env::current_exe().unwrap())
        .spawn()
        .unwrap();
    std::process::exit(0);
}
