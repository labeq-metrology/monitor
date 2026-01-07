use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_popup_webview(
    app: AppHandle,
    title: String,
    label: String,
    source: String,
) -> Result<(), String> {
    if let Some(win) = app.get_webview_window(&label) {
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    std::thread::spawn({
        let app = app.clone();
        move || {
            WebviewWindowBuilder::new(&app, label, WebviewUrl::App(source.into()))
                .title(title)
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
