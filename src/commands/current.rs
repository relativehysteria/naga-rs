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
        let guild_id = command.guild_id.unwrap();

        // Get the VC lock
        let handler_lock = manager.get(guild_id).unwrap();

        // Get the metadata about the currently playing song
        let current  = {
            match handler_lock.lock().await.queue().current() {
                Some(current) => current,
                None          => return response(command, &ctx.http,
                                                 "No song is playing").await,
            }
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
