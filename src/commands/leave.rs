use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// The "leave" command. Leaves a voice channel if the bot is in it.
pub struct Leave;

#[async_trait]
impl ApplicationCommandImplementation for Leave {
    fn alias(&self) -> String {
        "leave".to_string()
    }

    fn description(&self) -> String {
        "Leaves the currently joined voice channel.".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the manager
        let manager = get_songbird(ctx).await;

        // Get the guild id
        let guild_id = match command.guild_id {
            Some(id) => id,
            None     => return response(command, &ctx.http,
                                 "Command not used from a guild").await,
        };

        // Check that we are in a voice channel
        if manager.get(guild_id).is_none() {
            return response(command, &ctx.http, "Not in a voice channel.")
                .await;
        }

        // Attempt to leave the voice channel
        if let Err(e) = manager.remove(guild_id).await {
            let err = format!(
                "Error while trying to leave the voice channel: {:?}", e
            );
            eprintln!("{}", err);
            response(command, &ctx.http, &err).await
        } else {
            response(command, &ctx.http, "👍").await
        }
    }

}
