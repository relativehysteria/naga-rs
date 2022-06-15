//! `VoiceGatewayManager` implementation of Naga
// TODO: Get rid of this when done
#![allow(unused_variables)]
use futures_channel::mpsc::UnboundedSender as Sender;
use serenity::client::bridge::voice::VoiceGatewayManager;
use serenity::model::voice::VoiceState;
use serenity::model::id::{ UserId, GuildId };
use serenity::gateway::InterMessage;
use serenity::async_trait;

/// The `VoiceGatewayManager` implementation of Naga.
pub struct Manager;

impl Manager {
    /// Creates a new `Manager`.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl VoiceGatewayManager for Manager {
    async fn initialise(&self, shard_count: u64, user_id: UserId) {
    }

    async fn register_shard(&self, shard_id: u64, sender: Sender<InterMessage>){
    }

    async fn deregister_shard(&self, shard_id: u64) {
    }

    async fn server_update(
        &self,
        guild_id: GuildId,
        endpoint: &Option<String>,
        token: &str
    ) {
    }

    async fn state_update(&self, guild_id: GuildId, voice_state: &VoiceState) {
    }
}
