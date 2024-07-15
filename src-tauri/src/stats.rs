use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub cpu: i64,
    pub ram: i64,
    pub gpu: i64,
    pub fps: i64,
}

#[tauri::command]
pub async fn get_stats() -> Result<Stats, String> {
    Ok(Stats {
        cpu: 10,
        ram: 20,
        gpu: 40,
        fps: 60,
    })
}
