use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::{
        application_command::ApplicationCommandInteraction,
    }
};

/// Joins the bot into a voice channel.
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
        let guild_id = match command.guild_id {
            Some(id) => id,
            None     => return response(command, &ctx.http,
                                        "Command not used from a guild").await,
        };

        // Get the guild
        let guild = match ctx.cache.guild(guild_id).await {
            Some(guild) => guild,
            None        => return Ok(()),
        };

        // Get the voice channel id
        let voice_channel_id = match guild.voice_states
            .get(&command.user.id)
            .and_then(|voice| voice.channel_id)
        {
            Some(vc_id) => vc_id,
            None        => return response(command, &ctx.http,
                                           "Not in a voice channel").await,
        };

        // Bots shouldn't reconnect between channels
        if manager.get(guild_id).is_some() {
            return response(command, &ctx.http,
                            "Already connected to a voice channel").await;
        }

        // Join the VC
        let (_, status) = manager.join(guild_id, voice_channel_id).await;
        if let Err(e) = status {
            let err = format!("Error while joining a voice channel: {:?}", e);
            eprintln!("{}", err);
            response(command, &ctx.http, &err).await
        } else {
            response(command, &ctx.http, "I'm in B)").await
        }
    }
}
