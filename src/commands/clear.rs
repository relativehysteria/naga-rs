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

/// Clears the currently played queue.
pub struct ClearRadek;

#[async_trait]
impl RadekHahaha for ClearRadek {
    fn alias(&self) -> String {
        "clear".to_string()
    }

    fn description(&self) -> String {
        "Clears the currently played queue..".to_string()
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

        // Clear the queue
        let radek4 = radek2.get(radek3).unwrap();
        { radek4.lock().await.queue().stop(); }
        rradek(radek1, &radek.http, "Queue cleared").await
    }
}
