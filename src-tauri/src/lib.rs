#![allow(dead_code, unused_imports, unused_mut, unused_variables,non_snake_case)]
mod session;
mod encrypt;
mod dcp;
mod command;
use once_cell::sync::OnceCell;
use anyhow::Result;

use tauri::webview::PlatformWebview;
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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let start = std::time::Instant::now();
    eprintln!("asassasasasasasasasasasas");
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .setup(move |app| {
            eprintln!("start time:{:?}", start.elapsed());
            APP.get_or_init(|| app.handle().clone());
            tauri::async_runtime::spawn(setup(app.handle().clone()));
            app.get_webview_window("main")
                .unwrap()
                .with_webview(webview_setup)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
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

async fn setup(app: AppHandle) -> Result<()> {
    // init code ocr model
    REC.get_or_init(|| Rec::from_embed().unwrap());

    // init dcp
    let mut session = DcpSession::build()?;
    session.set_inner().await?;
    app.manage::<Mutex<Option<DcpSession>>>(Mutex::new(Some(session)));

    Ok(())
}

fn webview_setup(webview: PlatformWebview) {
    #[cfg(target_os = "android")]
    {
        use jni::objects::JValue;
        webview.jni_handle().exec(|env, _, webview| {
            // 获取 WebView 的设置对象
            let mut settings = env
                .call_method(
                    webview,
                    "getSettings",
                    "()Landroid/webkit/WebSettings;",
                    &[],
                )
                .expect("Failed to get WebSettings")
                .l()
                .expect("Expected WebSettings object");

            // 启用 JavaScript
            env.call_method(
                &settings,
                "setJavaScriptEnabled",
                "(Z)V",
                &[JValue::Bool(1)],
            )
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
            env.call_method(
                &settings,
                "setLoadWithOverviewMode",
                "(Z)V",
                &[JValue::Bool(1)],
            )
            .expect("Failed to set load with overview mode");
        })
    }
}
