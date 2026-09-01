use socketioxide::{ extract::{ Data, SocketRef }, SocketIo };
use enigo::{ Direction, Enigo, Key, Keyboard, Settings };
use axum::{ response::Html, routing::get, Router };
use tauri::{ AppHandle, Emitter, Manager, State };
use tower_http::cors::CorsLayer;
use local_ip_address::local_ip;
use tokio::net::TcpListener;
use std::sync::Mutex;

struct ServerState {
  port: Mutex<u16>,
}

const REMOTE_HTML: &str = include_str!("../static/remote.html");

fn trigger_slide_action(action: &str) {
  let enigo: Result<Enigo, enigo::NewConError> = Enigo::new(&Settings::default());
  if enigo.is_err() {
    eprintln!("Failed to initialize Enigo: {:?}", enigo.err());
    return;
  }
  let mut enigo: Enigo = enigo.unwrap();

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

async fn start_server(app_handle: AppHandle) {
  let (layer, io) = SocketIo::new_layer();

  let connect_app_handle: AppHandle = app_handle.clone();
  io.ns("/", move |socket: SocketRef| {
    connect_app_handle.emit("connected", ()).unwrap();

    socket.on("slide-control", |_socket: SocketRef, Data::<String>(action)| {
      trigger_slide_action(&action);
    });

    let disconnect_app_handle: AppHandle = connect_app_handle.clone();
    socket.on_disconnect(move |_socket: SocketRef| {
      disconnect_app_handle.emit("disconnected", ()).unwrap();
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
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .manage(ServerState {
      port: Mutex::new(0),
    })
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![get_server_url])
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  let start_server_handle: AppHandle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    start_server(start_server_handle).await;
  });

  app.run(|_, _| {});
}
