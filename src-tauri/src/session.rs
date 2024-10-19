use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::{Client, Proxy};
use cookie_store::CookieStore;
use log::{error, warn};
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::{self, File};
use std::io::{BufReader, Cursor, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::anyhow;
use tauri_plugin_http::reqwest::redirect::Policy;

/// `Session` is a user-friendly `Client` wrapper, which automatically handles cookies and load/store
/// cookies from/to the specified path.
#[derive(Debug, Clone)]
pub struct Session {
    #[allow(dead_code)] // just make clippy happy
    pub(crate) state: Arc<State>,
    client: Client,
}

impl Session {
    /// Try to creates a new `Session` instance, and load cookies from `cookie_store_path`.
    /// When `Session` is dropped(more specifically, when `State` is dropped), it will store cookies
    /// to `cookie_store_path`.
    pub fn try_new(cookie_store_path: PathBuf) -> anyhow::Result<Session> {
        let state = State::try_new(cookie_store_path)?;
        let state = Arc::new(state);

        let client = Client::builder()
            // .redirect(Policy::none())
            .danger_accept_invalid_certs(true)
            // .proxy(Proxy::all("http://192.168.137.1:10809").unwrap())
            .cookie_provider(
                state.cookie_store.clone()
            )
            .build()?;

        Ok(Session { state, client })
    }

    pub fn get_cookie(&self) -> anyhow::Result<String> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        self.state.cookie_store.lock().unwrap().save_incl_expired_and_nonpersistent(&mut cursor, |cookie| {
            // let format = time::format_description::parse(
            //     "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
            // ).expect("Failed to parse format description");

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(format!(
                "{}={};",
                cookie.name(),
                cookie.value(),
            ))
        }).map_err(|e| anyhow!(e.to_string()))?;
        let cookie_string = String::from_utf8(buffer)?;
        Ok(cookie_string)
    }
    pub fn save_cookie(&self) {
        self.state.save_cookie();
    }
    pub fn clear_cookie(&mut self) -> anyhow::Result<()> {
        eprintln!("clearing cookie....");
        self.state.clear_cookie()?;
        // let path = (&self.state.cookie_store_path).clone();
        // eprintln!("cookies: {}", std::fs::read_to_string(&path)?);
        // if path.exists() {
        //     fs::remove_file(&path)?;
        //     eprintln!("clear cookie in {}", path.display());
        // };
        // eprintln!("cookies: {} {}", path.display(), std::fs::read_to_string(&path)?);
        // let state = State::try_new(path.clone())?;
        // let state = Arc::new(state);
        // self.client = Client::builder()
        //     // .redirect(Policy::none())
        //     .danger_accept_invalid_certs(true)
        //     .cookie_provider(
        //         state.cookie_store.clone()
        //     )
        //     .build()?;
        // self.state = state;

        Ok(())
    }
}

impl Deref for Session {
    type Target = Client;
    fn deref(&self) -> &Client {
        &self.client
    }
}

#[derive(Debug)]
pub struct State {
    pub cookie_store_path: PathBuf,
    pub cookie_store: Arc<CookieStoreMutex>,
}

impl State {
    pub fn try_new(cookie_store_path: PathBuf) -> anyhow::Result<State> {
        // if cookie_store_path.exists() { fs::remove_file(&cookie_store_path)? };
        let cookie_store = match File::open(&cookie_store_path) {
            Ok(f) =>
                CookieStore::load_json(BufReader::new(f)).map_err(|e| {
                    let context = format!(
                        "error when read cookies from {}",
                        cookie_store_path.display()
                    );
                    anyhow::anyhow!("{}", e).context(context)
                })?,
            Err(e) => {
                warn!(
                    "open {} failed. error: {}, use default empty cookie store",
                    cookie_store_path.display(),
                    e
                );
                CookieStore::default()
            }
        };
        let cookie_store = Arc::new(CookieStoreMutex::new(cookie_store));

        Ok(State {
            cookie_store_path,
            cookie_store,
        })
    }
    pub fn save_cookie(&self) {
        let mut file = match fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.cookie_store_path)
        {
            Ok(f) => f,
            Err(e) => {
                error!(
                    "open {} for write failed. error: {}",
                    self.cookie_store_path.display(),
                    e
                );
                return;
            }
        };

        let store = self.cookie_store.lock().unwrap();
        if let Err(e) = store.save_incl_expired_and_nonpersistent_json(&mut file) {
            error!(
                "save cookies to path {} failed. error: {}",
                self.cookie_store_path.display(),
                e
            );
        }
    }

    pub fn clear_cookie(&self) -> anyhow::Result<()> {
        self.cookie_store.lock().unwrap().clear();
        eprintln!("{:?}", self.cookie_store);
        if self.cookie_store_path.exists() { fs::remove_file(&self.cookie_store_path)? };
        Ok(())
        // let mut file = match fs::OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .truncate(true)
        //     .open(&self.cookie_store_path)
        // {
        //     Ok(f) => f,
        //     Err(e) => {
        //         error!(
        //             "open {} for write failed. error: {}",
        //             self.cookie_store_path.display(),
        //             e
        //         );
        //         return;
        //     }
        // };

        // let store = CookieStore::default();
        // if let Err(e) = store.save_incl_expired_and_nonpersistent_json(&mut file) {
        //     error!(
        //         "save cookies to path {} failed. error: {}",
        //         self.cookie_store_path.display(),
        //         e
        //     );
        // }
    }
}

impl Drop for State {
    fn drop(&mut self) {

        // self.save_cookie();
    }
}