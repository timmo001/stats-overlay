mod autostart;
mod logger;
mod settings;
mod shared;
mod stats;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    Emitter, Manager,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

use crate::{
    autostart::setup_autostart,
    logger::setup_logger,
    settings::{get_settings, update_settings},
    stats::{get_stats, setup_stats_thread},
};

#[tauri::command]
async fn set_window(app: tauri::AppHandle) -> Result<(), String> {
    // Get the main window
    let window = app.get_webview_window("main").unwrap();

    let _ = window.clone().with_webview(move |_webview| {
        // Allow clickthrough on the window (Windows)
        #[cfg(target_os = "windows")]
        unsafe {
            let hwnd = window.hwnd().unwrap().0;
            let hwnd = windows::Win32::Foundation::HWND(hwnd);
            use windows::Win32::UI::WindowsAndMessaging::*;
            let nindex = GWL_EXSTYLE;
            let style = WS_EX_APPWINDOW
                | WS_EX_COMPOSITED
                | WS_EX_LAYERED
                | WS_EX_TRANSPARENT
                | WS_EX_TOPMOST;
            let _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initial setup of settings
    get_settings();

    // Setup logger
    setup_logger().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            get_stats,
            set_window,
            update_settings
        ])
        .setup(|app| {
            {
                let main_window = app.get_webview_window("main").unwrap();

                // Hide the windows on startup
                main_window.hide().unwrap();
                app.get_webview_window("settings").unwrap().hide().unwrap();

                // Setup autostart
                setup_autostart(app).unwrap();

                // Setup tray menu
                let separator = PredefinedMenuItem::separator(app)?;
                let toggle_window =
                    MenuItemBuilder::with_id("toggle_window", "Show/Hide window").build(app)?;
                let show_settings =
                    MenuItemBuilder::with_id("show_settings", "Open settings").build(app)?;
                let exit = MenuItemBuilder::with_id("exit", "Exit").build(app)?;

                let menu = MenuBuilder::new(app)
                    .items(&[
                        &toggle_window,
                        &separator,
                        &show_settings,
                        &separator,
                        &exit,
                    ])
                    .build()?;

                // Setup tray icon
                let tray = app.tray_by_id("main").unwrap();
                tray.set_menu(Some(menu))?;
                tray.on_menu_event(move |app, event| match event.id().as_ref() {
                    "toggle_window" => {
                        let _ = app.emit("shortcut-event", "Alt+S");
                    }
                    "show_settings" => {
                        // Get the settings window
                        let settings_window = app.get_webview_window("settings").unwrap();

                        // Send the show event to the window
                        settings_window.emit("show", {}).unwrap();

                        // Open devtools on startup
                        #[cfg(debug_assertions)] // Only include this code on debug builds
                        {
                            settings_window.open_devtools();
                        };

                        // Focus the window
                        settings_window.set_focus().unwrap();
                    }
                    "exit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                });

                // Open devtools on startup
                #[cfg(debug_assertions)] // Only include this code on debug builds
                {
                    // Open devtools
                    main_window.open_devtools();
                };

                // Setup hotkeys
                app.handle().plugin(
                    match tauri_plugin_global_shortcut::Builder::new().with_shortcuts(["alt+s"]) {
                        Ok(builder) => builder
                            .with_handler(|app, shortcut, event| {
                                if event.state == ShortcutState::Pressed {
                                    if shortcut.matches(Modifiers::ALT, Code::KeyS) {
                                        let _ = app.emit("shortcut-event", "Alt+S");
                                    }
                                }
                            })
                            .build(),
                        Err(e) => {
                            // Log and return an empty builder
                            log::error!("Error registering shortcut: {:?}", e);
                            tauri_plugin_global_shortcut::Builder::new().build()
                        }
                    },
                )?;

                setup_stats_thread(app);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
