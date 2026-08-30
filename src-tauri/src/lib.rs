use local_ip_address::local_ip;

use socketioxide::{ extract::{ Data, SocketRef }, SocketIo };
use enigo::{ Direction, Enigo, Key, Keyboard, Settings };
use axum::{ routing::get, response::Html, Router };
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;
use std::sync::Mutex;
use tauri::{ State, Manager, AppHandle };

struct ServerState {
  port: Mutex<u16>,
}

const REMOTE_HTML: &str = include_str!("../static/remote.html");

fn trigger_slide_action(action: &str) {
  let mut enigo: Enigo = Enigo::new(&Settings::default()).expect("Failed to initialize Enigo");

  match action {
    "NEXT" => {
      let _ = enigo.key(Key::RightArrow, Direction::Click);
    }
    "PREV" => {
      let _ = enigo.key(Key::LeftArrow, Direction::Click);
    }
    _ => {}
  }
}

async fn start_server(app_handle: tauri::AppHandle) {
  let (layer, io) = SocketIo::new_layer();

  io.ns("/", |socket: SocketRef| {
    println!("Client connected: {:?}", socket.id);

    socket.on("slide-control", |_socket: SocketRef, Data::<String>(action)| {
      println!("Received slide action: {}", action);
      trigger_slide_action(&action);
    });
  });

  let app: Router = Router::new()
    .route(
      "/",
      get(|| async { Html(REMOTE_HTML) })
    )
    .layer(layer)
    .layer(CorsLayer::permissive());

  let listener: TcpListener = TcpListener::bind("0.0.0.0:0").await.unwrap();
  let assigned_port: u16 = listener.local_addr().unwrap().port();

  let state: State<'_, ServerState> = app_handle.state::<ServerState>();
  *state.port.lock().unwrap() = assigned_port;

  axum::serve(listener, app).await.unwrap();
}

#[tauri::command]
fn get_server_url(state: State<'_, ServerState>) -> Result<String, String> {
  let port: u16 = *state.port.lock().unwrap();
  if port == 0 {
    return Err("Server is still initializing...".into());
  }

  match local_ip() {
    Ok(ip) => Ok(format!("http://{}:{}", ip, port)),
    Err(err) => Err(format!("Could not determine local IP: {}", err)),
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let app: tauri::App = tauri::Builder
    ::default()
    .manage(ServerState {
      port: Mutex::new(0),
    })
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![get_server_url])
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  let handle: AppHandle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    start_server(handle).await;
  });

  app.run(|_, _| {});
}
