//! Utility functions
use serenity::{
    prelude::*,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// Creates and sends a simple text interaction response.
pub async fn response(
    command: &ApplicationCommandInteraction,
    http: &serenity::http::Http,
    message: &str,
) -> Result<(), SerenityError>  {
    command.create_interaction_response(http, |response| {
        response.interaction_response_data(|msg| {
            msg.content(message)
        })
    }).await
}
