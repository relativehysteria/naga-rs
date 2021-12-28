use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

use songbird::tracks::LoopState;

/// Puts the currently playing song on a loop
pub struct SongLoop;

#[async_trait]
impl ApplicationCommandImplementation for SongLoop {
    fn alias(&self) -> String {
        "loop".to_string()
    }

    fn description(&self) -> String {
        "Puts the currently playing song on a loop".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        let guild_id = match command.guild_id {
            Some(id) => id,
            None     => return response(command, &ctx.http,
                                        "Command not used from a guild").await,
        };

        // Clear the queue
        let handler = match manager.get(guild_id) {
            Some(lock) => lock,
            None       => return response(command, &ctx.http,
                                          "Not in a voice channel").await
        };

        // Get the currently playing track
        let track = { handler.lock().await.queue().current() };

        // Loop the song if it isn't looping, stop looping if it is
        if let Some(track) = track {
            if track.get_info().await
            .and_then(|i| Ok(i.loops)) == Ok(LoopState::Infinite) {
                if track.disable_loop().is_ok() {
                    response(command, &ctx.http, "Disabled looping").await
                } else {
                    response(command, &ctx.http,
                             "An error has occurred while disabling loop").await
                }
            } else {
                if track.enable_loop().is_ok() {
                    response(command, &ctx.http, "Enabled looping").await
                } else {
                    response(command, &ctx.http,
                             "An error has occurred while enabling loop").await
                }
            }
        } else {
            response(command, &ctx.http, "No song is currently playing").await
        }
    }

}
