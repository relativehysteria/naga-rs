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
        let guild_id = command.guild_id.unwrap();

        // Clear the queue
        let handler_lock = manager.get(guild_id).unwrap();
        { handler_lock.lock().await.queue().stop(); }
        response(command, &ctx.http, "Queue cleared").await
    }
}
