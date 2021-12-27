use crate::commands::*;
use serenity::{
    builder::CreateEmbed,
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
        let metadata = current.metadata().clone();

        // Get the fields that we care about
        let source    = metadata.source_url.unwrap();
        let channel   = metadata.channel;
        let duration  = metadata.duration;
        let thumbnail = metadata.thumbnail;
        let title     = metadata.title
            .and_then(|title| Some(format!("[{}]({})", title, source)));

        // Don't create an embed if there is no title
        if title.is_none() {
            return response(command, &ctx.http, "The song has no title").await;
        }

        // Create the embed
        let mut embed = &mut CreateEmbed(std::collections::HashMap::new());
        embed = embed
            .title("Currently playing song")
            .description(title.unwrap());

        // Add the duration
        if duration.is_some() {
            embed = embed.field("Duration",
                                parse_duration(duration.unwrap()), true);
        }

        // Add the channel name
        if channel.is_some() {
            embed = embed.field("Uploader", channel.unwrap(), true);
        }

        // Add the thumbnail
        if thumbnail.is_some() {
            embed = embed.thumbnail(thumbnail.unwrap());
        }

        command.create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|msg| {
                msg.add_embed(embed.clone())
            })
        }).await
    }
}
