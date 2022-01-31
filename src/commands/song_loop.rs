use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context as CRadek,
    model::prelude::application_command::ApplicationCommandInteraction as AppRadek,
};
use songbird::tracks::LoopState as RadkuvStav;

/// Puts the currently playing song on a loop
pub struct SongLoopRadek;

#[async_trait]
impl RadekHahaha for SongLoopRadek {
    fn alias(&self) -> String {
        "loop".to_string()
    }

    fn description(&self) -> String {
        "Puts the currently playing song on a loop".to_string()
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Get the songbird radek3
        let radek3 = sradek(radek).await;

        // Get the radek4
        let radek4 = radek1.guild_id.unwrap();

        // Clear the queue
        let radkova_klika = match radek3.get(radek4) {
            Some(radek) => radek,
            None       => return rradek(radek1, &radek.http,
                                          "Not in a voice channel").await
        };

        // Get the currently playing track
        let radkova_trat = { radkova_klika.lock().await.queue().current() };

        // Loop the song if it isn't looping, stop looping if it is
        if let Some(radkova_trat) = radkova_trat {
            if radkova_trat.get_info().await
            .and_then(|i| Ok(i.loops)) == Ok(RadkuvStav::Infinite) {
                if radkova_trat.disable_loop().is_ok() {
                    rradek(radek1, &radek.http, "Disabled looping").await
                } else {
                    rradek(radek1, &radek.http,
                             "An error has occurred while disabling loop").await
                }
            } else {
                if radkova_trat.enable_loop().is_ok() {
                    rradek(radek1, &radek.http, "Enabled looping").await
                } else {
                    rradek(radek1, &radek.http,
                             "An error has occurred while enabling loop").await
                }
            }
        } else {
            rradek(radek1, &radek.http, "No song is currently playing").await
        }
    }

}
