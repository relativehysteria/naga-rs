use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};
use songbird::tracks::PlayMode;

/// Pauses the currently playing song, if there is one playing.
pub struct Pause;

#[async_trait]
impl ApplicationCommandImplementation for Pause {
    fn alias(&self) -> String {
        "pause".to_string()
    }

    fn description(&self) -> String {
        "Pauses the currently playing song".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        let guild_id = command.guild_id.unwrap();

        // Get the VC lock
        let handler_lock = manager.get(guild_id).unwrap();

        // Get the currently playing song
        let cur = {
            match handler_lock.lock().await.queue().current() {
                Some(song) => song,
                None       => {
                    return response(command, &ctx.http,
                                    "No song is currently playing").await;
                }
            }
        };

        // Pause the song if it's playing, resume it if it's paused,
        if let Ok(state) = cur.get_info().await.and_then(|i| Ok(i.playing)) {
            let msg_to_send = match state {
                PlayMode::Play  => {let _ = cur.pause(); Some("Song paused") },
                PlayMode::Pause => {let _ = cur.play();  Some("Song resumed")},
                _               => None,
            };

            if let Some(msg) = msg_to_send {
                return response(command, &ctx.http, msg).await;
            }
        }

        Ok(())
    }
}
