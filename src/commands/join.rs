use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::{
        application_command::ApplicationCommandInteraction,
    }
};

/// The "join" command. Joins the bot into a voice channel.
pub struct Join;

#[async_trait]
impl ApplicationCommandImplementation for Join {
    fn alias(&self) -> String {
        "join".to_string()
    }

    fn description(&self) -> String {
        "Joins your voice channel.".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild id
        let guild_id = command.guild_id;
        if guild_id.is_none() {
            return response(command, &ctx.http, "Command not used from a guild")
                .await;
        }
        let guild_id = guild_id.unwrap();

        // Get the guild
        let guild = ctx.cache.guild(guild_id).await;
        if guild.is_none() {
            eprintln!("Couldn't find GuildId ({:?}) in cache.", guild_id);
            return Ok(());
        }
        let guild = guild.unwrap();

        // Get the voice channel id
        let voice_channel_id = guild.voice_states
            .get(&command.user.id)
            .and_then(|voice| voice.channel_id);
        if voice_channel_id.is_none() {
            let err = "Not in a voice channel";
            return response(command, &ctx.http, &err).await;
        }
        let voice_channel_id = voice_channel_id.unwrap();

        // Join the VC
        let (lock, status) = manager.join(guild_id, voice_channel_id).await;

        if let Err(e) = status {
            let err = format!("Error while joining a voice channel: {:?}", e);
            eprintln!("{}", err);
            response(command, &ctx.http, &err).await
        } else {
            response(command, &ctx.http, "I'm in B)").await
        }
    }
}
