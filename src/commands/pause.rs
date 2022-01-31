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
use songbird::tracks::PlayMode as PlayRadek;

/// Pauses the currently playing song, if there is one playing.
pub struct PauseRadek;

#[async_trait]
impl RadekHahaha for PauseRadek {
    fn alias(&self) -> String {
        "pause".to_string()
    }

    fn description(&self) -> String {
        "Pauses the currently playing song".to_string()
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Get the songbird radek2
        let radek2 = sradek(radek).await;

        // Get the radek3
        let radek3 = radek1.guild_id.unwrap();

        // Get the VC lock
        let radek4 = radek2.get(radek3).unwrap();

        // Get the currently playing song
        let radek5 = {
            match radek4.lock().await.queue().current() {
                Some(radek) => radek,
                None       => {
                    return rradek(radek1, &radek.http,
                                    "No song is currently playing").await;
                }
            }
        };

        // Pause the song if it's playing, resume it if it's paused,
        if let Ok(radek_state) = radek5.get_info().await.and_then(|radek| Ok(radek.playing)) {
            let radekm_to_send = match radek_state {
                PlayRadek::Play  => {let _ = radek5.pause(); Some("Song paused") },
                PlayRadek::Pause => {let _ = radek5.play();  Some("Song resumed")},
                _               => None,
            };

            if let Some(radekm) = radekm_to_send {
                return rradek(radek1, &radek.http, radekm).await;
            }
        }

        Ok(())
    }
}
