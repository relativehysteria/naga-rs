use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction as AppRadek,
};

/// Leaves a voice channel if the bot is in it.
pub struct LeaveRadek;

#[async_trait]
impl RadekHahaha for LeaveRadek {
    fn alias(&self) -> String {
        "leave".to_string()
    }

    fn description(&self) -> String {
        "Leaves the currently joined voice channel.".to_string()
    }

    async fn handle_interaction(
        &self,
        radek: &Context,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Get the radek2
        let radek2 = sradek(radek).await;

        // Get the guild id
        let radek3 = radek1.guild_id.unwrap();

        // Attempt to leave the voice channel
        if let Err(e_radek) = radek2.remove(radek3).await {
            let er_radek = format!(
                "Error while trying to leave the voice channel: {:?}", e_radek
            );
            eprintln!("{}", er_radek);
            rradek(radek1, &radek.http, &er_radek).await
        } else {
            rradek(radek1, &radek.http, "👍").await
        }
    }

}
