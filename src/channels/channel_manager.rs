use crate::channels::public_channel_manager::{JoinResponse, LeaveResponse};
use crate::message::PusherMessage;
use crate::web_socket::WebSocket;

pub trait ChannelManager {
    fn join(&self, ws: WebSocket, channel: &str, message: PusherMessage) -> JoinResponse;
    fn leave(&self, channel: &str) -> LeaveResponse;
}
