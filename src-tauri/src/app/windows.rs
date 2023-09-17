pub mod cmd {
    use tauri::{Manager, WindowBuilder, WindowUrl};

    /// 打开设置页面
    /// # Return
    /// None
    #[tauri::command]
    pub fn setting_center(handle: tauri::AppHandle, _win_type: String) {
        tauri::async_runtime::spawn(async move {
            if handle.get_window("setting").is_none() {
                WindowBuilder::new(&handle, "setting", WindowUrl::App("index.html".into()))
                    .title("Setting")
                    .resizable(true)
                    .fullscreen(false)
                    .build()
                    .unwrap();
            } else {
                let main_win = handle.get_window("setting").unwrap();
                main_win.show().unwrap();
                main_win.set_focus().unwrap();
            }
        });
    }

    /// 打开更新日志窗口
    /// # Return
    /// None
    #[tauri::command]
    pub fn update_dialog(handle: tauri::AppHandle) {
        tauri::async_runtime::spawn(async move {
            if handle.get_window("update_dialog").is_none() {
                WindowBuilder::new(&handle, "update_dialog", WindowUrl::App("/app/update_dialog".into()))
                    .title("更新日志")
                    .resizable(true)
                    .fullscreen(false)
                    .build()
                    .unwrap();
            } else {
                let main_win = handle.get_window("update_dialog").unwrap();
                main_win.show().unwrap();
                main_win.set_focus().unwrap();
            }
        });
    }
}
