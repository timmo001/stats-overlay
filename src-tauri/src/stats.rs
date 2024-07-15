use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub cpu: f64, // CPU Usage Percentage to 2 decimal places
    pub mem: f64, // Memory Usage Percentage to 2 decimal places
}

#[tauri::command]
pub async fn get_stats() -> Result<Stats, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_info = sys.global_cpu_info();

    let cpu = cpu_info.cpu_usage() as f64;
    let mem = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;

    Ok(Stats {
        cpu: (cpu * 100.0).round() / 100.0,
        mem: (mem * 100.0).round() / 100.0,
    })
}
