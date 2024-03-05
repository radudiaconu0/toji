use crate::adapters::adapter::Adapter;
use crate::log::Log;
use crate::message::{PusherMessage, UWebSocketMessage};
use crate::web_socket::WebSocket;
use crate::{message, AppState};
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::response::IntoResponse;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use toji::{WebSocketUpgrade, WS};
use tokio::sync::Mutex;
use web_socket::Event;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
}

impl Eq for WebSocket {}

impl PartialEq for WebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct WSHandler;

impl WSHandler {
    pub async fn on_open(ws: &mut WebSocket) {
        Log::websocket_title("WebSocket connection opened");
        ws.id = Some(Self::generate_socket_id());
        // if app_state.closing {
        //     ws.send_json(serde_json::json!({
        //             "event": "pusher:error",
        //             "data": serde_json::json!({
        //                 "code": 4200,
        //                 "message": "Server is closing. Please reconnect shortly.",
        //             }),
        //         }))
        //         .await;
        // }
        let broadcast_message = serde_json::json!({
            "event": "pusher:connection_established",
            "data": serde_json::json!({
                "socket_id": ws.id.as_ref().unwrap(),
                "activity_timeout": 120,
            }),
        });
        ws.send_json(broadcast_message).await;
    }

    fn generate_socket_id() -> String {
        let mut rng = rand::thread_rng(); // Get a random number generator

        // Define min and max as u64, since Rust requires specifying the integer type
        let min: u64 = 0;
        let max: u64 = 10000000000;

        // Rust's rand crate handles generating a random number between min and max differently
        let mut random_number = |min: u64, max: u64| -> u64 { rng.gen_range(min..=max) };

        // Format the random numbers into a String with a dot separator
        format!("{}.{}", random_number(min, max), random_number(min, max))
    }

    pub(crate) fn on_close(code: u16, message: String) {
        Log::websocket("❌ Connection closed:");
        Log::websocket(&format!("Code: {}", code));
        Log::websocket(&format!("Message: {}", message));
    }

    pub(crate) async fn on_message(
        message: PusherMessage,
        mut ws: &WebSocket,
        state: Arc<Mutex<AppState>>,
    ) {
        Log::websocket_title("Received message from client");
        match message.data {
            Some(data) => {
                Log::websocket(serde_json::to_string_pretty(&data).unwrap().as_str());
            }
            None => {
                Log::websocket("No data");
            }
        }
        match message.event {
            Some(event) => match event.as_str() {
                "pusher:subscribe" => {
                    WSHandler::subscribe_to_channel(ws, message.channel.unwrap()).await;
                    let ws_id = ws.id.as_ref().unwrap().clone();
                    state.lock().await.sockets.lock().await.insert(ws_id, ws);
                }
                "pusher:unsubscribe" => {
                    ws.send_json(serde_json::json!({
                        "event": "pusher:unsubscribe",
                        "data": serde_json::json!({}),
                    }))
                    .await;
                    Log::websocket("Unsubscribing from channel");
                }
                "pusher:ping" => {
                    WSHandler::handle_ping(&mut ws).await;
                }
                "pusher:pong" => {
                    WSHandler::handle_ping(&mut ws).await;
                }
                _ => {
                    Log::websocket("No event");
                }
            },
            None => {
                Log::websocket("No event");
            }
        }
    }

    pub async fn handle_pong(mut ws: &mut WebSocket) {
        Log::websocket("Received pong");
        ws.send_json(serde_json::json!({
            "event": "pusher:ping",
            "data": serde_json::json!({}),
        }))
        .await;
        Log::websocket_title("Received pong");
    }

    pub async fn handle_ping(ws: &mut WebSocket) {
        ws.send_json(serde_json::json!({
            "event": "pusher:pong",
            "data": serde_json::json!({}),
        }))
        .await;
        Log::websocket_title("Received ping");
    }

    pub async fn handle_socket(socket: WS, who: SocketAddr, state: Arc<Mutex<AppState>>) {
        println!("New WebSocket connection: {}", who);
        let mut ws = WebSocket::new(socket);
        WSHandler::on_open(&mut ws).await;
        while let Ok(ev) = ws.ws.recv().await {
            match ev {
                Event::Data { ty, data } => {
                    println!("Data: {:#?}", ty);
                    let data = String::from_utf8(data.to_vec()).unwrap();
                    let pusher_message: message::PusherMessage =
                        serde_json::from_str(&data).unwrap();
                    WSHandler::on_message(pusher_message, &ws, state.clone()).await;
                }
                Event::Ping(_) => {
                    WSHandler::handle_ping(&mut ws).await;
                }
                Event::Pong(_) => {
                    WSHandler::handle_pong(&mut ws).await;
                }
                Event::Error(_) => {
                    Log::websocket("Error");
                }
                Event::Close { .. } => {
                    Log::websocket_title("❌ Connection closed:");
                }
            }
        }
    }

    pub async fn ws_handler(
        Path(app_id): Path<String>,
        query: Query<PusherWebsocketQuery>,
        ws: WebSocketUpgrade,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        State(state): State<Arc<Mutex<AppState>>>,
    ) -> impl IntoResponse {
        Log::info(format!(
            "WebSocket connection for app {}. Protocol: {}, client: {}, version: {}, flash: {}",
            app_id,
            query.protocol.unwrap_or(0),
            query.client.as_deref().unwrap_or(""),
            query.version.as_deref().unwrap_or(""),
            query.flash.unwrap_or(false)
        ));
        ws.on_upgrade(move |socket| WSHandler::handle_socket(socket, addr, state))
    }
    pub async fn subscribe_to_channel(ws: &WebSocket, channel_name: String) {
        ws.send_json(serde_json::json!({
            "event": "pusher_internal:subscription_succeeded",
            "data": serde_json::json!({}),
        }))
        .await;
    }
}

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct PusherWebsocketQuery {
    protocol: Option<u8>,
    client: Option<String>,
    version: Option<String>,
    flash: Option<bool>,
}
