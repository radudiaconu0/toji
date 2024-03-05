use crate::log::Log;
use crate::ws_handler::WSHandler;
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::response::{IntoResponse, Response};

use hyper::{HeaderMap, StatusCode};
use serde::Serialize;
use serde_json::json;

use crate::adapters::adapter::Adapter;
use crate::AppState;
use sysinfo::System;

pub struct ChannelResponse {
    subscription_count: u64,
    user_count: Option<u64>,
    occupied: bool,
}

pub struct HttpHandler {}

impl HttpHandler {
    pub async fn health_check() -> impl IntoResponse {
        "OK"
    }

    pub async fn channel(
        Path(app_id): Path<String>,
        Path(channel_name): Path<String>,
    ) -> impl IntoResponse {
        println!(
            "WebSocket connection for app {} and channel {}",
            app_id, channel_name
        );
    }

    pub async fn channels(
        Path(app_id): Path<u32>,
        State(mut state): State<AppState>,
    ) -> impl IntoResponse {
        todo!("Implement channels")
    }

    pub async fn ready() -> impl IntoResponse {
        Log::info("Server is ready".to_string());
        "OK"
    }

    pub async fn events() -> impl IntoResponse {
        "Events"
    }

    pub fn send_json(
        data: serde_json::Value,
        status: StatusCode,
    ) -> (StatusCode, HeaderMap, String) {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        (status, headers, data.to_string())
    }

    pub async fn usage() -> impl IntoResponse {
        let mut sys = System::new_all();
        sys.refresh_memory();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let free_memory = total_memory - used_memory;
        let percent_usage = (used_memory as f64 / total_memory as f64) * 100.0;
        HttpHandler::send_json(
            json!({
                "total_memory": total_memory,
                "used_memory": used_memory,
                "free_memory": free_memory,
                "percent_usage": percent_usage,
            }),
            StatusCode::OK,
        )
    }

    // pub async fn metrics(&self, query: Query<PrometheusQuery>) -> impl IntoResponse {
    //     let server = self.server.upgrade();
    //     if let Some(server) = server {
    //         let metrics = server.metrics.lock().await;
    //         if let Some(metrics) = metrics.as_ref() {
    //             if let Some(json) = query.json {
    //                 if json {
    //                     return match metrics.get_metrics_as_json().await {
    //                         Ok(metrics) => (StatusCode::OK, HeaderMap::new(), metrics.to_string())
    //                             .into_response(),
    //                         Err(e) => (
    //                             StatusCode::INTERNAL_SERVER_ERROR,
    //                             HeaderMap::new(),
    //                             format!("Error: {}", e),
    //                         )
    //                             .into_response(),
    //                     };
    //                 }
    //             }
    //             return match metrics.get_metrics_as_plaintext().await {
    //                 Ok(metrics) => (StatusCode::OK, HeaderMap::new(), metrics).into_response(),
    //                 Err(e) => (
    //                     StatusCode::INTERNAL_SERVER_ERROR,
    //                     HeaderMap::new(),
    //                     "Error: No metrics".to_string(),
    //                 )
    //                     .into_response(),
    //             };
    //         }
    //     }
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         HeaderMap::new(),
    //         "No metrics".to_string(),
    //     )
    //         .into_response()
    // }
    // pub async fn error() -> impl IntoResponse {
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         HeaderMap::new(),
    //         "Internal server error".to_string(),
    //     )
    // }
}

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct PrometheusQuery {
    json: Option<bool>,
}
