use tracing::{ Level, error };
use serenity::prelude::*;
use std::fs::read_to_string;
use naga_rs::SlashHandler;

/// Minimum log level of output messages
#[cfg(debug_assertions)]
pub const LOG_LEVEL: tracing::Level = Level::DEBUG;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL: tracing::Level = Level::INFO;

#[tokio::main]
async fn main() {
    // Set up the logger
    setup_logger();

    // Start the client
    let mut client = create_discord_client().await;
    if let Err(e) = client.start().await {
        error!("Client: {e:?}");
    }
}

/// Sets up the global tracing logger for Naga.
fn setup_logger() {
    tracing_subscriber::fmt()
        .with_max_level(LOG_LEVEL)
        .init();
}

/// Sets up and returns the Discord client for Naga.
/// This function doesn't return Errors -- only panics.
async fn create_discord_client() -> Client {
    // Read the token
    let token = read_to_string("TOKEN")
        .expect("Couldn't read TOKEN.");
    let token = token.trim();

    // Read the APPLICATION_ID
    let app_id = read_to_string("APPLICATION_ID")
        .expect("Couldn't read APPLICATION_ID")
        .trim()
        .parse::<u64>()
        .expect("Couldn't parse APPLICATION_ID as u64");

    // Prepare the intents
    let intents = GatewayIntents::empty();

    // Build the client
    Client::builder(token, intents)
        .application_id(app_id)
        .event_handler(SlashHandler::new())
        .await
        .expect("Couldn't build the client")
}
