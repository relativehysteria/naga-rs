use crate::{
    utils::*,
    commands::*,
};
use rand::prelude::*;
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context as CRadek,
    model::prelude::application_command::ApplicationCommandInteraction as AppRadek,
};

/// Shuffles the currently enqueued songs.
///
/// Like literally, those that are ENQUEUED, not those that are still being
/// enqueued. :|
pub struct ShuffleRadek;

#[async_trait]
impl RadekHahaha for ShuffleRadek {
    fn alias(&self) -> String {
        "shuffle".to_string()
    }

    fn description(&self) -> String {
        "Shuffles the currently enqueued songs.".to_string()
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

        // Shuffle the queue
        let _ = rradek(radek1, &radek.http, "Shuffling...").await;
        {
            let radkuv_zamek = match radek2.get(radek3) {
                Some(radkuv_zamek) => radkuv_zamek,
                None       => {
                    radek1.edit_original_interaction_response(&radek.http, |radek| {
                        radek
                            .content("Aborted!")
                    }).await?;
                    return Ok(());
                },
            };
            radkuv_zamek.lock().await.queue().modify_queue(|radek| {
                radek.make_contiguous()[1..].shuffle(&mut rand::thread_rng());
            });
        }
        radek1.edit_original_interaction_response(&radek.http, |radek| {
            radek
                .content("Shuffled!")
        }).await?;
        Ok(())
    }
}
