use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// Shows the metadata of the currently playing song
pub struct Current;

#[async_trait]
impl ApplicationCommandImplementation for Current {
    fn alias(&self) -> String {
        "current".to_string()
    }

    fn description(&self) -> String {
        "Shows the metadata of the currently playing song".to_string()
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

        // Get the VC lock
        let handler_lock = match manager.get(guild_id) {
            Some(lock) => lock,
            None       => return response(command, &ctx.http,
                                          "Not in a voice channel").await,
        };

        // Get the metadata about the currently playing song
        let handler  = handler_lock.lock().await;
        let current  = match handler.queue().current() {
            Some(current) => current,
            None          => return response(command, &ctx.http,
                                             "No song is playing").await,
        };

        // Create the embed
        let embed = create_embed_for_track(&current, "Currently playing song")
            .unwrap();

        command.create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|msg| {
                msg.add_embed(embed)
            })
        }).await
    }
}
