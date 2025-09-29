use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_settings(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("settings") {
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    std::thread::spawn({
        let app = app.clone();
        move || {
            WebviewWindowBuilder::new(
                &app,
                "settings",
                WebviewUrl::App("settings.html".into()),
            )
                .title("Settings")
                .inner_size(800.0, 480.0)
                .min_inner_size(800.0, 480.0)
                .resizable(true)
                .closable(true)
                .build()
                .unwrap();
        }
    });

    Ok(())
}
