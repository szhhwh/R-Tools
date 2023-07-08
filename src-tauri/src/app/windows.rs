pub mod cmd {
    use tauri::{Manager, WindowBuilder, WindowUrl};

    #[tauri::command]
    pub fn setting_center(handle: tauri::AppHandle, _win_type: String) {
        tauri::async_runtime::spawn(async move {
            if handle.get_window("setting").is_none() {
                WindowBuilder::new(
                    &handle,
                    "setting",
                    WindowUrl::App("index.html".into()),
                )
                .title("Setting")
                .resizable(true)
                .fullscreen(false)
                .build()
                .unwrap();
            } else {
                let main_win = handle.get_window("main").unwrap();
                main_win.show().unwrap();
                main_win.set_focus().unwrap();
            }
        });
    }
}
