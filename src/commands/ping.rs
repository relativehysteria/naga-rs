//! This is a template implementation for slash commands.
use crate::commands::ApplicationCommandImplementation;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

pub struct Ping;

#[async_trait]
impl ApplicationCommandImplementation for Ping {
    fn alias(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "A ping command.".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        command.create_interaction_response(&ctx.http, |response| {
            response
                .interaction_response_data(|msg| {
                    msg.content("I'm alive! :)")
                })
        }).await
    }

}
