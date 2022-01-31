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

/// Skips the currently playing song
pub struct SkipRadek;

#[async_trait]
impl RadekHahaha for SkipRadek {
    fn alias(&self) -> String {
        "skip".to_string()
    }

    fn description(&self) -> String {
        "Skips the currently playing song.".to_string()
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

        // Get the lock
        let radek_lock = radek2.get(radek3).unwrap();

        // Skip the song
        if let Err(e) = { let r=radek_lock.lock().await.queue().skip(); r } {
            eprintln!("Error while trying to skip: {:?}", e);
            rradek(radek1, &radek.http,
                     "An error has occurred while trying to skip").await
        } else {
            rradek(radek1, &radek.http, "Skipped.").await
        }
    }
}
