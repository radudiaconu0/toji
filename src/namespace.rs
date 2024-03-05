use crate::channels::channel::Channel;
use crate::channels::presence_channel_manager::{PresenceMember, PresenceMemberInfo};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use crate::web_socket::WebSocket;

pub struct Namespace {
    pub channels: HashMap<String, HashSet<String>>,
    pub users: HashMap<String, HashSet<String>>,
    pub app_id: String,
    pub sockets: Arc<Mutex<HashMap<String, WebSocket>>>,
}

impl Namespace {
    pub fn new(app_id: String) -> Self {
        Namespace {
            channels: HashMap::new(),
            users: HashMap::new(),
            app_id,
            sockets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_sockets(self) -> Result<Arc<Mutex<HashMap<String, WebSocket>>>, ()> {
        Ok(self.sockets)
    }

    pub async fn add_socket(&mut self, ws: WebSocket) -> bool {
        let ws_id = ws.id.clone().unwrap();
        self.sockets.lock().unwrap().insert(ws_id, ws);
        true
    }

    pub fn remove_socket(&mut self, id: String) -> Result<bool, ()> {
        self.sockets.lock().unwrap().remove(&id);
        Ok(true)
    }
    pub(crate) async fn remove_from_channel(
        &mut self,
        ws_id: &str,
        channel: Channel,
    ) -> Result<usize, ()> {
        let remove = |channel: &String, channels: &mut HashMap<String, HashSet<String>>| {
            if let Some(ws_set) = channels.get_mut(channel) {
                ws_set.remove(ws_id);
                if ws_set.is_empty() {
                    channels.remove(channel);
                }
            }
        };

        match channel {
            Channel::Vec(channel_vec) => {
                for ch in channel_vec {
                    remove(&ch, &mut self.channels);
                }
                Ok(0)
            }
            Channel::String(channel_string) => {
                remove(&channel_string, &mut self.channels);
                self.channels
                    .get(&channel_string)
                    .map(|ws_set| ws_set.len())
                    .ok_or(())
            }
        }
    }
    pub(crate) async fn add_to_channel(
        &mut self,
        channel: String,
        ws_id: String,
    ) -> Result<usize, ()> {
        let channel_set = self.channels.entry(channel).or_default();
        channel_set.insert(ws_id);
        Ok(channel_set.len())
    }

    pub fn is_in_channel(&self, ws_id: &str, channel: &str) -> Result<bool, ()> {
        Ok(self
            .channels
            .get(channel)
            .map(|subscribers| subscribers.contains(ws_id))
            .unwrap_or(false))
    }

    pub fn get_channels(&self) -> Result<HashMap<String, HashSet<String>>, ()> {
        Ok(self.channels.clone())
    }

    pub fn get_channels_with_sockets_count(&self) -> Result<HashMap<String, usize>, ()> {
        let channels = self.get_channels().unwrap();

        let mut list = HashMap::<String, usize>::new();
        for (channel, connections) in channels.iter() {
            list.insert(channel.to_string(), connections.len());
        }
        Ok(list)
    }

    pub fn add_user(&mut self, ws: WebSocket) -> Result<(), ()> {
        let user = ws.user.unwrap();
        let user_id = user.id;
        if let Some(channels) = self.users.get_mut(&user_id) {
            channels.insert(ws.id.clone().expect("Failed to get ws id"));
        } else {
            let mut set = HashSet::new();
            set.insert(ws.id.clone().unwrap());
            self.users.insert(user_id, set);
        }
        Ok(())
    }

    pub fn remove_user(&mut self, ws: WebSocket) -> Result<(), ()> {
        let user = ws.user;
        if let Some(user) = user {
            let user_id = user.id;
            if let Some(user) = self.users.get_mut(&user_id) {
                self.users.remove(&ws.id.clone().unwrap());
            }
            if let Some(user) = self.users.get_mut(&user_id) {
                if user.is_empty() {
                    self.users.remove(&user_id);
                }
            }
        }
        Ok(())
    }
    pub fn get_channel_sockets(&self, channel: &str) -> Result<HashMap<String, &WebSocket>, ()> {
        let channels = self.channels.clone();
        if channels.get(channel).is_none() {
            Ok(HashMap::new())
        } else {
            let ws_ids = channels.get(channel).unwrap();
            let mut sockets = HashMap::new();
            for ws_id in ws_ids.iter() {
                if let Some(ws) = self.sockets.lock().unwrap().get(ws_id) {
                    sockets.insert(ws_id.clone(), ws);
                }
            }
            Ok(sockets.clone())
        }
    }
    pub(crate) fn get_channel_sockets_count(&self, p0: &str) -> Result<usize, ()> {
        let channels = self.channels.clone();
        if channels.get(p0).is_none() {
            Ok(0)
        } else {
            let ws_ids = channels.get(p0).unwrap();
            Ok(ws_ids.len())
        }
    }

    pub fn get_channel_members(
        &self,
        channel: &str,
    ) -> Result<HashMap<String, PresenceMemberInfo>, ()> {
        // let sockets = self.get_channel_sockets(channel).unwrap();
        // let mut members = HashMap::<String, PresenceMemberInfo>::new();
        // for (ws_id, ws) in sockets {
        //     let channel = ws.presence_channels.as_ref().unwrap().get(channel);
        //     if channel.is_some() {
        //         let member: PresenceMember = PresenceMember {
        //             user_id: channel.unwrap().data.get("user_id").unwrap().to_string(),
        //             user_info: PresenceMemberInfo {
        //                 data: channel.unwrap().data.clone(),
        //             },
        //             socket_id: Option::from(
        //                 channel.unwrap().data.get("socket_id").unwrap().to_string(),
        //             ),
        //         };
        //         members.insert(member.user_id, member.user_info);
        //     }
        // }
        // Ok(members)
        todo!("Implement get_channel_members")
    }
    pub fn get_user_socket_ids(&self, user_id: &str) -> Result<Vec<String>, ()> {
        let mut socket_ids = Vec::new();
        if let Some(ws_ids) = self.users.get(user_id) {
            for ws_id in ws_ids.iter() {
                if self.sockets.lock().unwrap().contains_key(ws_id) {
                    socket_ids.push(ws_id.clone());
                }
            }
        }
        Ok(socket_ids)
    }
    pub fn get_user_sockets(&self, user_id: &str) -> Result<HashSet<&WebSocket>, ()> {
        let mut sockets = HashSet::new();
        if let Some(ws_ids) = self.users.get(user_id) {
            for ws_id in ws_ids.iter() {
                if let Some(ws) = self.sockets.lock().unwrap().get(ws_id) {
                    sockets.insert(ws);
                }
            }
        }
        Ok(sockets)
    }
}
