// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
mod session;
mod dcp;

mod encrypt;

mod command;


use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager};
use dddd_ocr::Rec;
use tokio::sync::Mutex;
use crate::dcp::DcpSession;
use crate::command::{
    login,
    get_base_info,
    get_class_table,
    get_soap_info,
    webview_to,
    clear_cookie,
    get_score,
    get_exam,
    get_inner,
    get_cookie,
};

pub static APP: OnceCell<AppHandle> = OnceCell::new();
pub static REC:OnceCell<Rec> = OnceCell::new();
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            APP.get_or_init(|| app.handle().clone());
            REC.get_or_init(|| Rec::from_embed().unwrap());
            app.get_webview_window("main").unwrap().with_webview(|webview| {
                #[cfg(target_os = "android")]
                {
                    use jni::objects::JValue;
                    webview.jni_handle().exec(|env, _, webview| {
                        // 获取 WebView 的设置对象
                        let mut settings = env.call_method(webview, "getSettings", "()Landroid/webkit/WebSettings;", &[])
                            .expect("Failed to get WebSettings")
                            .l()
                            .expect("Expected WebSettings object");

                        // 启用 JavaScript
                        env.call_method(&settings, "setJavaScriptEnabled", "(Z)V", &[JValue::Bool(1)])
                            .expect("Failed to enable JavaScript");

                        // // 启用内置缩放控件
                        // env.call_method(&settings, "setBuiltInZoomControls", "(Z)V", &[JValue::Bool(1)])
                        //     .expect("Failed to enable built-in zoom controls");
                        //
                        // // 隐藏缩放控件
                        // env.call_method(&settings, "setDisplayZoomControls", "(Z)V", &[JValue::Bool(0)])
                        //     .expect("Failed to hide zoom controls");

                        // 使内容适应 WebView 的宽度
                        env.call_method(&settings, "setUseWideViewPort", "(Z)V", &[JValue::Bool(1)])
                            .expect("Failed to set use wide viewport");

                        // 缩放内容以适应屏幕
                        env.call_method(&settings, "setLoadWithOverviewMode", "(Z)V", &[JValue::Bool(1)])
                            .expect("Failed to set load with overview mode");
                    })
                }
            }).map_err(|e| e.to_string())?;


            app.manage::<Mutex<Option<DcpSession>>>(Mutex::new(Some(DcpSession::build()
                .map_err(|e| e.to_string())?)));
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                handle.state::<Mutex<Option<DcpSession>>>()
                    .lock().await
                    .as_mut()
                    .unwrap()
                    .set_inner().await
                    ?;
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            get_base_info,
            get_class_table,
            get_soap_info,
            clear_cookie,
            webview_to,
            get_score,
            get_exam,
            get_inner,
            get_cookie,

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// pub fn setup_app(app:&mut tauri::App)
// ->std::result::Result<(), Box<dyn std::error::Error>>{
//
//     Ok(())
// }