//! This is a template implementation for slash commands.
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

/// A very simple ping command.
/// The source code of this command can be used as a template for other
/// commands.
pub struct PingRadek;

#[async_trait]
impl RadekHahaha for PingRadek {
    fn alias(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "A ping command.".to_string()
    }

    fn requires_voice_chat(&self) -> bool {
        false
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        rradek(radek1, &radek.http, "Pong").await
    }

}
