mod commands;
mod config;
mod utils;

use std::sync::Arc;

use anyhow::Result;
use arc_swap::ArcSwap;
use commands::{get_app_dir, get_config, greet};
use config::AppConfig;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, SubmenuBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::PageLoadPayload,
    App, AppHandle, Builder, Manager, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

const APP_NAME: &str = "chatapp";

pub struct AppState {
    pub config: Arc<ArcSwap<AppConfig>>,
}

pub fn app() -> Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir, get_config])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler);
    Ok(builder)
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");

    let state = AppState {
        config: Arc::new(ArcSwap::new(Arc::new(AppConfig::try_new()?))),
    };

    app.manage(state);

    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    setup_menu(handle)?;

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }

    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    webview.open_devtools();

    Ok(())
}

// on_page_load is called when the webview is created.
fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload) {
    info!("Page loaded: {}", webview.label());
}

// F: Fn(&Window<R>, &WindowEvent
fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("Close requested on {:?}", window.label());
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
        ])
        .level(tracing::log::LevelFilter::Info)
}

fn setup_menu(app: &AppHandle) -> Result<(), tauri::Error> {
    let icon = app.default_window_icon().unwrap().clone();
    let file_menu = SubmenuBuilder::with_id(app, "file", "File")
        .item(&MenuItem::with_id(
            app,
            "open",
            "Open",
            true,
            Some("CmdOrCtrl+O"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "save",
            "Save",
            true,
            Some("CmdOrCtrl+S"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "saveas",
            "Save As",
            true,
            Some("CmdOrCtrl+Shift+S"),
        )?)
        .separator()
        .quit()
        .build()?;
    let edit_menu = SubmenuBuilder::with_id(app, "edit", "Edit")
        .item(&MenuItem::with_id(
            app,
            "process",
            "Process",
            true,
            Some("CmdOrCtrl+P"),
        )?)
        .separator()
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .separator()
        .select_all()
        .item(&CheckMenuItem::with_id(
            app,
            "checkme",
            "Check Me",
            true,
            true,
            None::<&str>,
        )?)
        .build()?;
    let tray_menu = SubmenuBuilder::with_id(app, "tray", "Tray")
        .item(&MenuItem::with_id(app, "open", "Open", true, None::<&str>)?)
        .item(&MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?)
        .separator()
        .quit()
        .build()?;

    TrayIconBuilder::with_id(format!("{}-tray", APP_NAME))
        .tooltip(APP_NAME)
        .icon(icon)
        .menu(&tray_menu)
        .show_menu_on_left_click(true)
        .on_tray_icon_event(|tray, event| {
            //info!("Tray icon event: {:?}", event);
            if let TrayIconEvent::Click {
                button: MouseButton::Right,
                ..
            } = event
            {
                open_main(tray.app_handle()).expect("Failed to open main window in setup_menu");
            }
        })
        .build(app)?;

    let menu = Menu::with_items(app, &[&file_menu, &edit_menu])?;
    app.set_menu(menu)?;
    app.on_menu_event(|app, event| {
        info!("menu event: {:?}", event);
        match event.id.as_ref() {
            "open" => open_main(app).unwrap(),
            "save" => {}
            "saveas" => {}
            "process" => {}
            "checkme" => {
                // toggle checkme status and update config and runtime state
                // for runtime state - Arc<Mutex<State>> / ArcSwap
            }
            _ => {}
        }
    });
    Ok(())
}

fn open_main(handle: &AppHandle) -> Result<(), tauri::Error> {
    handle
        .get_webview_window("main")
        .ok_or_else(|| tauri::Error::WindowNotFound)?
        .show()?;

    Ok(())
}
