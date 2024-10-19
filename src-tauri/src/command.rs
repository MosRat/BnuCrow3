use std::str::FromStr;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;
use tauri::{Emitter, State, Url};
use crate::dcp::{parse_class_table, parse_exam, parse_score, DcpSession, Soap};
// use super::DCP;
use super::{APP, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcpErr {
    pub error: String,
    pub file: &'static str,
    pub line: u32,
}
#[tauri::command]
pub async fn login(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow, username: &str, password: &str) -> Result<String, String> {
    let start = std::time::Instant::now();

    // let mut s = session.lock().await.as_mut().unwrap();
    session.lock().await.as_mut().unwrap().try_login(username, password)
        .await
        .map_err(|e| e.to_string())?;
    eprintln!("login time cost {:?}", start.elapsed());

    let handle = APP.get().unwrap().clone();

    tauri::async_runtime::spawn(async move {

        // let mut s = session.lock()
        let session = handle.state::<Mutex<Option<DcpSession>>>();
        let mut s = session
            .lock()
            .await;
        //     .await;

        let start = std::time::Instant::now();

        s.as_mut().unwrap().init_aam()
            .await
            .map_err(|e| {
                let e = e.to_string();
                eprintln!("{:?}", e);
                webview_window
                    .emit("error", DcpErr {
                        error: e.clone(),
                        file: file!(),
                        line: line!(),
                    })
                    .map_err(|e| e.to_string())
                    .expect("Error send msg!")
                ;
                e
            })?;


        eprintln!("init aam time cost {:?}", start.elapsed());

        let start = std::time::Instant::now();

        let avatar = s.as_mut().unwrap().get_avatar()
            .await
            .map_err(|e| {
                let e = e.to_string();
                eprintln!("{:?}", e);
                webview_window
                    .emit("error", DcpErr {
                        error: e.clone(),
                        file: file!(),
                        line: line!(),
                    })
                    .map_err(|e| e.to_string())
                    .expect("Error send msg!");
                e
            })?;


        s.as_mut().unwrap()
            .get_info_mut()
            .unwrap()
            .insert("avatar".to_string(), avatar,
            );

        eprintln!("get avatar time cost {:?}", start.elapsed());

        webview_window
            .emit("info_inited", ())
            .map_err(|e| {
                let e = e.to_string();
                eprintln!("{:?}", e);
                webview_window
                    .emit("error", DcpErr {
                        error: e.clone(),
                        file: file!(),
                        line: line!(),
                    })
                    .map_err(|e| e.to_string())
                    .expect("Error send msg!");
                e
            })?;


        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    });


    // *session.lock().await = Some(s);

    Ok("Successfully login!".to_string())
}

#[tauri::command]
pub async fn get_base_info(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow) -> Result<String, String> {
    if let Some(ref session) = *session.lock().await
    {
        let info = session.get_info().unwrap();
        Ok(format!("{info:?}"))
    } else {
        Err("dcp not initialize!".to_string())
    }
}

#[tauri::command]
pub async fn get_class_table(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow) -> Result<String, String> {
    if let Some(ref mut session) = *session.lock().await {
        if let Some(info) = session.get_info() {
            if let Some(class_table) = info.get("class_table") {
                Ok(class_table.clone())
            } else {
                let res = session.get_class_table()
                    .await
                    .map_err(|err| { err.to_string() })?;
                let class_parsed = parse_class_table(&res)
                    .map_err(|err| { err.to_string() })?
                    .to_string();
                if let Some(info) = session.get_info_mut() {
                    info.insert("class_table".to_string(), class_parsed.clone());
                }
                Ok(class_parsed)
            }
        } else {
            Err("info not initialize!".to_string())
        }
    } else {
        Err("dcp not initialize!".to_string())
    }
}

#[tauri::command]
pub async fn get_exam(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow, xn: String, xq: String) -> Result<String, String> {
    if let Some(ref mut session) = *session.lock().await {
        if let Some(info) = session.get_info() {
            if let Some(exam) = info.get(&format!("exam_{xn}_{xq}")) {
                Ok(exam.clone())
            } else {
                let mut combined_array = Vec::new();
                for i in 1..=4 {
                    let res = session.get_exam(xn.parse::<u32>().map_err(|err| { err.to_string() })?, xq.parse::<u32>().map_err(|err| { err.to_string() })?, i)
                        .await
                        .map_err(|err| { err.to_string() })?
                        ;
                    let res_parsed = parse_exam(&res).map_err(|err| { err.to_string() })?;
                    if let Value::Array(arr) = res_parsed {
                        combined_array.extend(arr);
                    }
                }
                let exam_parsed = Value::Array(combined_array);
                if let Some(info) = session.get_info_mut() {
                    info.insert(format!("exam_{xn}_{xq}"), exam_parsed.to_string());
                }
                Ok(exam_parsed.to_string())
            }
        } else {
            Err("info not initialize!".to_string())
        }
    } else {
        Err("dcp not initialize!".to_string())
    }
}

#[tauri::command]
pub async fn get_score(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow, xn: String, xq: String) -> Result<String, String> {
    if let Some(ref mut session) = *session.lock().await {
        if let Some(info) = session.get_info() {
            if let Some(score) = info.get(&format!("score_{xn}_{xq}")) {
                Ok(score.clone())
            } else {
                let score = session.get_score(xn.parse::<u32>().map_err(|err| { err.to_string() })?, xq.parse::<u32>().map_err(|err| { err.to_string() })?)
                    .await
                    .map_err(|err| { err.to_string() })?
                    ;
                let tbody_re = Regex::new(r"(?s)<tbody>(.*?)</tbody>").unwrap();
                let score_parsed = parse_score(tbody_re.captures(&score)
                    .ok_or(format!("Cant get table in score! {score}"))
                    .map_err(|err| { err.to_string() })?
                    .get(1)
                    .ok_or(format!("Cant get table in score! {score}"))
                    .map_err(|err| { err.to_string() })?
                    .as_str()
                )
                    .map_err(|err| { err.to_string() })?;
                if let Some(info) = session.get_info_mut() {
                    info.insert(format!("score_{xn}_{xq}"), score_parsed.to_string());
                }
                Ok(score_parsed.to_string())
            }
        } else {
            Err("info not initialize!".to_string())
        }
    } else {
        Err("dcp not initialize!".to_string())
    }
}

#[tauri::command]
pub async fn get_inner(session: State<'_, Mutex<Option<DcpSession>>>) -> Result<bool, String> {
    Ok(session.lock().await.as_ref().unwrap().inner)
}

#[tauri::command]
pub async fn get_soap_info(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow) -> Result<Soap, String> {
    Ok(session.lock().await.as_ref().unwrap().get_soap_info().await.map_err(|e| {
        let e = e.to_string();
        webview_window
            .emit("error", DcpErr {
                error: e.clone(),
                file: file!(),
                line: line!(),
            })
            .map_err(|e| e.to_string())
            .expect("Error send msg!")
        ;
        e
    })?)
}

#[tauri::command]
pub async fn webview_to(session: State<'_, Mutex<Option<DcpSession>>>, mut webview_window: tauri::WebviewWindow, path: String, js: String, with_cookies: bool) -> Result<(), String> {
    webview_window.navigate(Url::parse(&path).unwrap())
        .map_err(|e| {
            let e = e.to_string();
            webview_window
                .emit("error", DcpErr {
                    error: e.clone(),
                    file: file!(),
                    line: line!(),
                })
                .map_err(|e| e.to_string())
                .expect("Error send msg!")
            ;
            e
        })?;

    let cookies = session.lock().await.as_ref().unwrap().get_cookie().unwrap_or_default();

    webview_window.with_webview(move |webview| {
        #[cfg(target_os = "android")]
        {
            use jni::objects::{JValue, JObject};
            webview.jni_handle().exec(move |env, _, webview| {
                // 获取 WebView 的设置对象
                let mut settings = env.call_method(webview, "getSettings", "()Landroid/webkit/WebSettings;", &[])
                    .expect("Failed to get WebSettings")
                    .l()
                    .expect("Expected WebSettings object");

                // 启用 JavaScript
                env.call_method(&settings, "setJavaScriptEnabled", "(Z)V", &[JValue::Bool(1)])
                    .expect("Failed to enable JavaScript");

                // 启用内置缩放控件
                env.call_method(&settings, "setBuiltInZoomControls", "(Z)V", &[JValue::Bool(1)])
                    .expect("Failed to enable built-in zoom controls");

                // 隐藏缩放控件
                env.call_method(&settings, "setDisplayZoomControls", "(Z)V", &[JValue::Bool(0)])
                    .expect("Failed to hide zoom controls");

                // 使内容适应 WebView 的宽度
                env.call_method(&settings, "setUseWideViewPort", "(Z)V", &[JValue::Bool(1)])
                    .expect("Failed to set use wide viewport");

                // 缩放内容以适应屏幕
                env.call_method(&settings, "setLoadWithOverviewMode", "(Z)V", &[JValue::Bool(1)])
                    .expect("Failed to set load with overview mode");

                if with_cookies {
                    // 获取 CookieManager 实例
                    let cookie_manager = env.call_static_method(
                        "android/webkit/CookieManager",
                        "getInstance",
                        "()Landroid/webkit/CookieManager;",
                        &[],
                    ).expect("Failed to get CookieManager")
                        .l()
                        .expect("Expected CookieManager object");

                    // // 获取当前 URL（假设您已经加载了一个 URL）
                    // let url = env.call_method(webview, "getUrl", "()Ljava/lang/String;", &[])
                    //     .expect("Failed to get URL")
                    //     .l()
                    //     .expect("Expected String object");

                    // 使用登录页面的基础URL
                    let cas_url = "https://cas.bnu.edu.cn";
                    // 使用WebVPN页面的基础URL
                    let vpn_url = "https://onevpn.bnu.edu.cn/";

                    // 分割 cookies 字符串并设置每个 cookie
                    for cookie in cookies.lines() {
                        env.call_method(
                            &cookie_manager,
                            "setCookie",
                            "(Ljava/lang/String;Ljava/lang/String;)V",
                            &[
                                (&env.new_string(cas_url).expect("Failed to create URL string")).into(),
                                (&env.new_string(cookie.trim()).expect("Failed to create JString")).into()
                            ],
                        ).expect("Failed to set cookie");


                        env.call_method(
                            &cookie_manager,
                            "setCookie",
                            "(Ljava/lang/String;Ljava/lang/String;)V",
                            &[
                                (&env.new_string(vpn_url).expect("Failed to create URL string")).into(),
                                (&env.new_string(cookie.trim()).expect("Failed to create JString")).into()
                            ],
                        ).expect("Failed to set cookie");
                    }


                    // 同步 cookies
                    env.call_method(
                        &cookie_manager,
                        "flush",
                        "()V",
                        &[],
                    ).expect("Failed to flush cookies");
                }
            })
        }
    }).map_err(|e| e.to_string())?;

    eprintln!("js:{}", js);

    webview_window.eval(&(js +
        &format!(r#"
    const cookies = document.cookie;
    console.log("webviewCookies:",cookies);
    "#,
        ))
    ).map_err(|e| {
        let e = e.to_string();
        webview_window
            .emit("error", DcpErr {
                error: e.clone(),
                file: file!(),
                line: line!(),
            })
            .map_err(|e| e.to_string())
            .expect("Error send msg!")
        ;
        e
    })?;
    Ok(())
}


#[tauri::command]
pub async fn get_cookie(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow) -> Result<String, String> {
    Ok((*session.lock().await).as_mut().unwrap().get_cookie().map_err(|e| {
        let e = e.to_string();
        webview_window
            .emit("error", DcpErr {
                error: e.clone(),
                file: file!(),
                line: line!(),
            })
            .map_err(|e| e.to_string())
            .expect("Error send msg!")
        ;
        e
    })?)
}


#[tauri::command]
pub async fn clear_cookie(session: State<'_, Mutex<Option<DcpSession>>>, webview_window: tauri::WebviewWindow) -> Result<(), String> {
    Ok((*session.lock().await).as_mut().unwrap().clear_cookie().map_err(|e| {
        let e = e.to_string();
        webview_window
            .emit("error", DcpErr {
                error: e.clone(),
                file: file!(),
                line: line!(),
            })
            .map_err(|e| e.to_string())
            .expect("Error send msg!")
        ;
        e
    })?)
}


