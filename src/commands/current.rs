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

/// Shows the metadata of the currently playing song
pub struct CurrentRadek;

#[async_trait]
impl RadekHahaha for CurrentRadek {
    fn alias(&self) -> String {
        "current".to_string()
    }

    fn description(&self) -> String {
        "Shows the metadata of the currently playing song".to_string()
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

        // Get the metadata about the radek5ly playing song
        let radek5  = {
            match radek4.lock().await.queue().current() {
                Some(radek5) => radek5,
                None          => return rradek(radek1, &radek.http,
                                                 "No song is playing").await,
            }
        };

        // Create the radek6
        let radek6 = cradek(&radek5, "Currently playing song")
            .unwrap();

        radek1.create_interaction_response(&radek.http, |radek| {
            radek.interaction_response_data(|radek| {
                radek.add_embed(radek6)
            })
        }).await
    }
}
