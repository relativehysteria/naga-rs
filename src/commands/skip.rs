use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// Skips the currently playing song
pub struct Skip;

#[async_trait]
impl ApplicationCommandImplementation for Skip {
    fn alias(&self) -> String {
        "skip".to_string()
    }

    fn description(&self) -> String {
        "Skips the currently playing song.".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        if let Some(guild_id) = command.guild_id {
            // Skip the song
            if let Some(lock) = manager.get(guild_id) {
                if let Err(e) = lock.lock().await.queue().skip() {
                    eprintln!("Error while trying to skip: {:?}", e);
                    response(command, &ctx.http,
                             "An error has occurred while trying to skip").await
                } else {
                    response(command, &ctx.http, "Skipped.").await
                }
            } else {
                response(command, &ctx.http, "Not in a voice channel").await
            }
        } else {
            response(command, &ctx.http, "Command not used from a guild").await
        }
    }
}
