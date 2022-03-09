use crate::{
    utils::*,
    commands::*,
};
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
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        let guild_id = command.guild_id.unwrap();

        // Get the search term
        let term = command.data.options.get(0)
            .and_then(|term| term.resolved.as_ref());
        let term = if let Some(ACIDOV::String(term)) = term {
            term.clone()
        } else {
            return response(
                command, &ctx.http,
                "An error has occurred while trying to get the song"
            ).await;
        };

        // Before we look for the source of the song, we first have to inform
        // the user that we are doing something at all.
        // This has to be done within 3 seconds of the interaction spawn
        // otherwise Discord thinks we're dead...
        // https://github.com/goverfl0w/discord-interactions/issues/30#issuecomment-753583597
        response(command, &ctx.http, "Queueing...").await?;

        // Get the urls
        let urls = if term.starts_with("http") {
            // A url. If it is a playlist, get the urls to the tracks
            extract_urls(&term)
        } else {
            // Not an url, just a search term
            vec![term]
        };

        // Enqueue the urls
        let url_len = urls.len();
        for (i, url) in urls.into_iter().enumerate() {
            let i = i + 1;
            // Get the restartable ytdl thing
            let ytdl = if url.starts_with("http") {
                Restartable::ytdl(url, true).await
            } else {
                Restartable::ytdl_search(url, true).await
            };
            let ytdl = match ytdl {
                Ok(ytdl) => ytdl,
                Err(e)   => {
                    eprintln!("Error while queueing a song: {:?}", e);
                    continue;
                },
            };

            // Get the handler lock. This could fail if a bot leaves a voice
            // channel when it's still queueing.
            let handler_lock = match manager.get(guild_id) {
                Some(lock) => lock,
                None       => {
                    command.edit_original_interaction_response(&ctx.http, |r| {
                        r
                            .content("Aborted.")
                    }).await?;
                    return Ok(());
                },
            };

            // Enqueue the source
            let mut handler = handler_lock.lock().await;
            handler.enqueue_source(ytdl.into());

            // If there is only one song being enqueued, create an embed.
            // If there's more of them, show how many we've enqueued
            // and whether we're done.
            if url_len == 1 {
                let queue = handler.queue().current_queue();
                drop(handler);
                let song = queue.last().unwrap();

                // Create the embed
                let embed = create_embed_for_track(&song, "Enqueued").unwrap();

                command.edit_original_interaction_response(&ctx.http, |resp| {
                    resp
                        .content("Queued up!")
                        .add_embed(embed)
                }).await?;
            } else if url_len != i {
                command.edit_original_interaction_response(&ctx.http, |resp| {
                    resp
                        .content(format!("Queueing.. ({}/{})", i, url_len))
                }).await?;
            } else {
                command.edit_original_interaction_response(&ctx.http, |resp| {
                    resp
                        .content(format!("Finished! ({}/{})", i, url_len))
                }).await?;
            }
        }

        Ok(())
    }
}
