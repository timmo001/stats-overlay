use serde::{Deserialize, Serialize};
use sysinfo::{RefreshKind, System};
use tauri::{Emitter, Manager};
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub cpu: f64, // CPU Usage Percentage to 1 decimal place
    pub memory: f64, // Memory Usage Percentage to 1 decimal place
}

fn update_stats(sys: &System) -> Stats {
    let cpu_info = sys.global_cpu_info();

    let cpu = cpu_info.cpu_usage() as f64;
    let memory = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;

    Stats {
        cpu: (cpu * 10.0).round() / 10.0,
        memory: (memory * 10.0).round() / 10.0,
    }
}

#[tauri::command]
pub async fn get_stats() -> Result<Stats, String> {
    let sys_kind = RefreshKind::everything().without_processes();
    let mut sys = System::new_with_specifics(sys_kind);

    sys.refresh_all();

    Ok(update_stats(&sys))
}

pub fn setup_stats_thread(app: &tauri::App) {
    let main_window = app.get_webview_window("main").unwrap();

    // Start a thread to update the stats every second
    tokio::spawn(async move {
        let sys_kind = RefreshKind::everything().without_processes();
        let mut sys = System::new_with_specifics(sys_kind);

        loop {
            // Update stats
            sys.refresh_all();
            let stats = update_stats(&sys);

            // Send the stats to the frontend
            main_window.emit("stats", stats).unwrap();

            // Sleep for one second. This yields control back to the tokio runtime
            // and allows other tasks to run.
            sleep(Duration::from_secs(1)).await;
        }
    });
}
