use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    builder::CreateApplicationCommand as CreateRadek,
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context as CRadek,
    model::prelude::application_command::{
        ApplicationCommandInteraction as AppRadek,
        ApplicationCommandOptionType as ACOTRadek,
        ApplicationCommandInteractionDataOptionValue as ACIRadek,
    },
};
use songbird::input::Restartable;

/// Plays something in the voice chat that the bot is connected to.
pub struct PlayRadek;

#[async_trait]
impl RadekHahaha for PlayRadek {
    fn alias(&self) -> String {
        "play".to_string()
    }

    fn description(&self) -> String {
        "Plays a song in your voice chat.".to_string()
    }

    fn command_signature<'a>(
        &self,
        radek: &'a mut CreateRadek
    ) -> &'a mut CreateRadek {
        radek
            .name(self.alias())
            .description(self.description())
            .create_option(|radek| {
                radek
                    .name("search")
                    .description("The song to play. \
                                 Can be a url or a term to search youtube for")
                    .kind(ACOTRadek::String)
                    .required(true)
            })
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Get the songbird manager
        let radek2 = sradek(radek).await;

        // Get the radek3
        let radek3 = radek1.guild_id.unwrap();

        // Get the search radek_radek
        let radek_radek = radek1.data.options.get(0)
            .and_then(|radek_radek| radek_radek.resolved.as_ref());
        let radek_radek = if let Some(ACIRadek::String(radek_radek)) = radek_radek {
            radek_radek.clone()
        } else {
            return rradek(
                radek1, &radek.http,
                "An error has occurred while trying to get the song"
            ).await;
        };

        // Before we look for the source of the song, we first have to inform
        // the user that we are doing something at all.
        // This has to be done within 3 seconds of the interaction spawn
        // otherwise Discord thinks we're dead...
        // https://github.com/goverfl0w/discord-interactions/issues/30#issuecomment-753583597
        rradek(radek1, &radek.http, "Queueing...").await?;

        // Get the uradeks
        let uradeks = if radek_radek.starts_with("http") {
            // A uradek. If it is a playlist, get the uradeks to the tracks
            eradek(&radek_radek)
        } else {
            // Not an uradek, just a search radek_radek
            vec![radek_radek]
        };

        // Enqueue the uradeks
        let uradek_len = uradeks.len();
        for (radek_iter, uradek) in uradeks.into_iter().enumerate() {
            let radek_iter = radek_iter + 1;
            // Get the restartable ytdl thing
            let radekytdl = if uradek.starts_with("http") {
                Restartable::ytdl(uradek, true).await
            } else {
                Restartable::ytdl_search(uradek, true).await
            };
            let radekytdl = match radekytdl {
                Ok(radekytdl) => radekytdl,
                Err(e)   => {
                    eprintln!("Error while queueing a song: {:?}", e);
                    continue;
                },
            };

            // Get the handler lock. This could fail if a bot leaves a voice
            // channel when it's still queueing.
            let radkuv_zamek = match radek2.get(radek3) {
                Some(radekxdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd) => radekxdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd,
                None       => {
                    radek1.edit_original_interaction_response(&radek.http, |r| {
                        r
                            .content("Aborted.")
                    }).await?;
                    return Ok(());
                },
            };

            // Enqueue the source
            let mut radkuv_klic_nebo_neco = radkuv_zamek.lock().await;
            radkuv_klic_nebo_neco.enqueue_source(radekytdl.into());

            // If there is only one song being enqueued, create an embed.
            // If there's more of them, show how many we've enqueued
            // and whether we're done.
            if uradek_len == 1 {
                let spanelskej_radek = radkuv_klic_nebo_neco.queue().current_queue();
                drop(radkuv_klic_nebo_neco);
                let radkova_pisen = spanelskej_radek.last().unwrap();

                // Create the embed
                let em_radek = cradek(&radkova_pisen, "Enqueued").unwrap();

                radek1.edit_original_interaction_response(&radek.http, |radek| {
                    radek
                        .content("Queued up!")
                        .add_embed(em_radek)
                }).await?;
            } else if uradek_len != radek_iter {
                radek1.edit_original_interaction_response(&radek.http, |radek| {
                    radek
                        .content(format!("Queueing.. ({}/{})", radek_iter, uradek_len))
                }).await?;
            } else {
                radek1.edit_original_interaction_response(&radek.http, |radek| {
                    radek
                        .content(format!("Finished! ({}/{})", radek_iter, uradek_len))
                }).await?;
            }
        }

        Ok(())
    }
}
