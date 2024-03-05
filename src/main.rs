use crate::options::Options;
use crate::ws_handler::WSHandler;
use axum::routing::get;
use axum::{Router, ServiceExt};
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

mod app;
mod message;

mod adapters;
mod channels;
mod http_handler;
mod log;
mod namespace;
mod options;
mod token;
mod utils;
mod web_socket;
mod ws_handler;

use crate::web_socket::WebSocket;
use adapters::local_adapter::LocalAdapter;

#[derive(Default)]
pub struct AppState {
    options: Options,
    closing: bool,
    sockets: Mutex<HashMap<String, WebSocket>>,
}
#[tokio::main]
async fn main() {
    let app_state = AppState {
        options: Options::new(),
        closing: false,
        sockets: Mutex::new(HashMap::new()),
    };
    pub type SharedState = Arc<Mutex<AppState>>;
    let app_state = SharedState::new(Mutex::new(app_state));
    let app = Router::new()
        .route("/app/:app_id", get(WSHandler::ws_handler))
        .with_state(Arc::clone(&app_state));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6001")
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct PusherWebsocketQuery {
    protocol: Option<u8>,
    client: Option<String>,
    version: Option<String>,
    flash: Option<bool>,
}
