use std::hash::Hash;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use toji::WS;
use crate::log::Log;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
}

pub struct WebSocket {
    pub(crate) ws: WS,
    pub(crate) id: Option<String>,
    pub app_key: Option<String>,
    pub(crate) user: Option<User>,
}

impl Hash for WebSocket {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl WebSocket {
    pub fn new(ws: WS) -> Self {
        WebSocket {
            ws,
            id: None,
            app_key: None,
            user: None,
        }
    }

    pub async fn send_json(&mut self, data: serde_json::Value) /* Result<(), Box<dyn Error>> */ {
        Log::websocket_title("Sending message to client");
        Log::websocket(&data.to_string());
        let message = match serde_json::to_string(&data) {
            Ok(message) => message,
            Err(e) => {
                Log::websocket(&format!("Error: {}", e));
                return;
            }
        };
        self.ws
            .send(message.as_str())
            .await
            .expect("TODO: handle errors: all network requests can fail");
    }
}
