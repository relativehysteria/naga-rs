use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// Clears the currently played queue.
pub struct Clear;

#[async_trait]
impl ApplicationCommandImplementation for Clear {
    fn alias(&self) -> String {
        "clear".to_string()
    }

    fn description(&self) -> String {
        "Clears the currently played queue..".to_string()
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
            // Clear the queue
            if let Some(lock) = manager.get(guild_id) {
                let handler = lock.lock().await;
                let queue   = handler.queue();
                queue.stop();

                response(command, &ctx.http, "Queue cleared").await
            } else {
                response(command, &ctx.http, "Not in a voice channel").await
            }
        } else {
            response(command, &ctx.http, "Command not used from a guild")
                .await
        }
    }

}
