use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Client;
use cookie_store::CookieStore;
use log::{error, warn};
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::{self, File};
use std::io::BufReader;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

/// `Session` is a user-friendly `Client` wrapper, which automatically handles cookies and load/store
/// cookies from/to the specified path.
#[derive(Debug, Clone)]
pub struct Session {
    #[allow(dead_code)] // just make clippy happy
    state: Arc<State>,
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
            .cookie_provider(state.cookie_store.clone())
            .build()?;

        Ok(Session { state, client })
    }
}

impl Deref for Session {
    type Target = Client;
    fn deref(&self) -> &Client {
        &self.client
    }
}

#[derive(Debug)]
struct State {
    cookie_store_path: PathBuf,
    cookie_store: Arc<CookieStoreMutex>,
}

impl State {
    pub fn try_new(cookie_store_path: PathBuf) -> anyhow::Result<State> {
        let cookie_store = match File::open(&cookie_store_path) {
            Ok(f) => CookieStore::load_json(BufReader::new(f)).map_err(|e| {
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
}

impl Drop for State {
    fn drop(&mut self) {
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
        if let Err(e) = store.save_json(&mut file) {
            error!(
                "save cookies to path {} failed. error: {}",
                self.cookie_store_path.display(),
                e
            );
        }
    }
}