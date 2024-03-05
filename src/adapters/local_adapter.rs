use crate::adapters::adapter::Adapter;
use crate::channels::channel::Channel;
use crate::channels::presence_channel_manager::PresenceMemberInfo;
use crate::message;
use crate::namespace::Namespace;
use crate::web_socket::WebSocket;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct LocalAdapter {
    pub namespaces: HashMap<String, Namespace>,
}
impl LocalAdapter {
    pub fn new() -> Self {
        LocalAdapter {
            namespaces: HashMap::new(),
        }
    }
}

impl Adapter for LocalAdapter {
    async fn get_namespace(&mut self, app_id: &str) -> Result<&Namespace, ()> {
        Ok(self.namespaces.get(app_id).unwrap())
    }

    async fn get_namespaces(&self) -> HashMap<String, &Namespace> {
        let namespaces = &self.namespaces;
        let mut result = HashMap::new();
        for (k, v) in namespaces.iter() {
            result.insert(k.clone(), v.clone());
        }
        result
    }

    async fn add_socket(&mut self, app_id: &str, ws: WebSocket) -> bool {
        let namespace = self.namespaces.get_mut(app_id).unwrap();
        namespace.add_socket(ws).await;
        true
    }

    async fn remove_socket(&mut self, app_id: &str, ws_id: &str) -> bool {
        todo!()
    }

    async fn add_to_channel(&mut self, app_id: &str, channel: &str, ws: WebSocket) -> usize {
        todo!()
    }

    async fn remove_from_channel(
        &mut self,
        app_id: &str,
        channel: Channel,
        ws_id: &str,
    ) -> Option<usize> {
        todo!()
    }

    async fn send(&mut self, app_id: &str, channel: &str, data: &str, excepting_id: Option<&str>) {
        todo!()
    }

    async fn terminate_user_connections(&mut self, app_id: &str, user_id: &str) {}

    async fn send_to_socket(namespace: Namespace, user_id: &str, message: serde_json::Value) {
        namespace.to_user_sockets(user_id, |ws| async { ws.send_json(message).await })
    }

    async fn disconnect(&self) {
        todo!()
    }

    async fn clear_namespace(&mut self, namespace_id: &str) {
        todo!()
    }

    async fn clear_namespaces(&mut self) {
        todo!()
    }

    async fn get_sockets(&mut self, app_id: &str, only_local: bool) -> HashMap<String, WebSocket> {
        todo!()
    }

    async fn get_sockets_count(&mut self, app_id: &str, only_local: bool) -> usize {
        todo!()
    }

    async fn get_channels(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, HashSet<String>> {
        todo!()
    }

    async fn get_channels_with_sockets_count(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, usize> {
        todo!()
    }

    async fn get_channel_sockets(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, &WebSocket> {
        todo!()
    }

    async fn get_channel_sockets_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        todo!()
    }

    async fn get_channel_members(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, PresenceMemberInfo> {
        todo!()
    }

    async fn get_channel_members_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        todo!()
    }

    async fn is_in_channel(
        &mut self,
        app_id: &str,
        channel: &str,
        ws_id: &str,
        only_local: bool,
    ) -> bool {
        todo!()
    }

    async fn add_user(&mut self, ws: WebSocket) {
        todo!()
    }

    async fn remove_user(&mut self, ws: WebSocket) {
        todo!()
    }

    async fn get_user_sockets(&mut self, app_id: &str, user_id: &str) -> HashSet<&WebSocket> {
        todo!()
    }
}
