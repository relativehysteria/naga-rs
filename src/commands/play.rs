use crate::commands::*;
use serenity::{
    builder::CreateApplicationCommand,
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::{
        ApplicationCommandInteraction,
        ApplicationCommandOptionType,
        ApplicationCommandInteractionDataOptionValue as ACIDOV,
    },
};

use songbird::input::Restartable;

/// The "play" command.
/// Plays something in the voice chat that the bot is connected to.
pub struct Play;

#[async_trait]
impl ApplicationCommandImplementation for Play {
    fn alias(&self) -> String {
        "play".to_string()
    }

    fn description(&self) -> String {
        "Plays a song in your voice chat.".to_string()
    }

    fn command_signature<'a>(
        &self,
        command: &'a mut CreateApplicationCommand
    ) -> &'a mut CreateApplicationCommand {
        command
            .name(self.alias())
            .description(self.description())
            .create_option(|opt| {
                opt
                    .name("search")
                    .description("The song to play. \
                                 Can be a url or a term to search youtube for")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        let err_msg = "Must provide a URL or a search term.";

        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the search term
        let term = command.data.options.get(0)
            .and_then(|term| term.resolved.as_ref());
        let term = if let Some(ACIDOV::String(term)) = term {
            term.clone()
        } else {
            return response(command, &ctx.http, err_msg).await;
        };

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

        // Attempt to insert the song into the queue

        // First, get the handler
        let mut handler = handler_lock.lock().await;

        // Check that we have a valid URL
        // TODO: Change this to work with `ytdl_search`
        if !term.starts_with("http") {
            return response(command, &ctx.http, "Provide a valid URL.").await;
        }

        // Get the audio source
        let source = match Restartable::ytdl(term, true).await {
            Ok(source) => source,
            Err(e)     => {
                eprintln!("Error while queueing a song: {:?}", e);
                return response(command, &ctx.http,
                                "Couldn't fetch audio stream source.")
                    .await;
            }
        };

        // Enqueue the source
        handler.enqueue_source(source.into());

        response(command, &ctx.http, "Song added to the queue.").await
    }
}
