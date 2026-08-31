use tauri::{ AppHandle, Manager, WebviewUrl, WebviewWindowBuilder };
use serde::Deserialize;
use reqwest::Client;
use semver::Version;

const GITHUB_OWNER: &str = "kennething";
const GITHUB_REPO: &str = "better-slides";

#[derive(Debug, Deserialize)]
struct GithubRelease {
  tag_name: String,
}

pub async fn check_for_update(app: AppHandle) -> Result<(), String> {
  let tauri_version: String = std::env::var("TAURI_VERSION").expect("TAURI_VERSION not found in environment variables");

  let client: Client = Client::new();

  let url: String = format!("https://api.github.com/repos/{}/{}/releases/latest", GITHUB_OWNER, GITHUB_REPO);

  let release: GithubRelease = client
    .get(url)
    .header("User-Agent", "my-tauri-app")
    .header("Accept", "application/vnd.github+json")
    .send().await
    .map_err(|error| error.to_string())?
    .error_for_status()
    .map_err(|error| error.to_string())?
    .json().await
    .map_err(|error| error.to_string())?;

  let current: &str = tauri_version.trim_start_matches('v');
  let latest: &str = release.tag_name.trim_start_matches('v');

  let current_version: Version = Version::parse(current).map_err(|error| error.to_string())?;
  let latest_version: Version = Version::parse(latest).map_err(|error| error.to_string())?;

  if latest_version > current_version {
    open_update_window(&app, current, latest)?;
  }

  Ok(())
}

fn open_update_window(app: &AppHandle, current: &str, latest: &str) -> Result<(), String> {
  if app.get_webview_window("update").is_some() {
    return Ok(());
  }

  let url: String = format!("/update?current={}&latest={}", urlencoding::encode(current), urlencoding::encode(latest));

  WebviewWindowBuilder::new(app, "update", WebviewUrl::App(url.into()))
    .title("Update Available")
    .inner_size(600.0, 450.0)
    .resizable(false)
    .center()
    .build()
    .map_err(|error| error.to_string())?;

  Ok(())
}
