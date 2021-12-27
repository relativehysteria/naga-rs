use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// The "clear" command.
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
        // Get the guild_id
        let guild_id = command.guild_id;
        if guild_id.is_none() {
            return response(command, &ctx.http, "Command not used from a guild")
                .await;
        }
        let guild_id = guild_id.unwrap();

        // Get the songbird manager
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird VC placed in at initialization")
            .clone();

        // Clear the queue
        if let Some(lock) = manager.get(guild_id) {
            let handler = lock.lock().await;
            let queue   = handler.queue();
            queue.stop();

            response(command, &ctx.http, "Queue cleared").await
        } else {
            response(command, &ctx.http, "Not in a voice channel").await
        }
    }

}
