//! This is a template implementation for slash commands.
use crate::commands::ApplicationCommandImplementation;
use crate::utils::response;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};


/// A very simple ping command.
/// The source code of this command can be used as a template for other
/// commands.
pub struct Ping;

#[async_trait]
impl ApplicationCommandImplementation for Ping {
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
        ctx: &Context,
        cmd: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        response(cmd, &ctx.http, "Pong!").await
    }
}
