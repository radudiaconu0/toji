use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, ServiceExt};
use axum::routing::get;
use serde::Serialize;
use tokio::sync::Mutex;
use crate::options::Options;
use crate::ws_handler::WSHandler;

mod message;
mod app;

mod options;
mod log;
mod web_socket;
mod token;
mod utils;
mod ws_handler;
mod namespace;
mod channels;
mod adapters;
mod http_handler;

use adapters::local_adapter::LocalAdapter;
use crate::web_socket::WebSocket;

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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6001").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct PusherWebsocketQuery {
    protocol: Option<u8>,
    client: Option<String>,
    version: Option<String>,
    flash: Option<bool>,
}
