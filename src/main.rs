use std::fs::read_to_string;
use serenity::{
    async_trait,
    prelude::*,
    model::{
        prelude::*,
        interactions::application_command::ApplicationCommand,
    },
};
use songbird::SerenityInit;
use naga_rs::commands::*;


/// Handler for slash commands
struct SlashHandler {
    /// Commands registered into this handler
    commands: Vec<Box<dyn ApplicationCommandImplementation + Sync + Send>>,
}

impl SlashHandler {
    /// Builds the handler and puts all the commands returned by
    /// `get_bot_commands` into the inner `commands` vec
    fn new() -> Self {
        Self { commands: get_bot_commands(), }
    }
}

#[async_trait]
impl EventHandler for SlashHandler {
    /// Handle slash commands
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(interaction) = interaction {
            // Find the command, if it exists.
            let name    = &interaction.data.name;
            let command = self.commands.iter().find(|x| &x.alias() == name);

            // And execute it.
            if let Some(command) = command {
                // Errors are handled by the CALLER.
                let res = command.handle_interaction(&ctx, &interaction).await;
                if let Err(e) = res {
                    eprintln!("Error while handling a slash command: {:?}", e);
                }
            }
        }
    }

    /// Register the `ApplicationCommand`s into the bot when its internal
    /// `GuildId` cache is ready.
    async fn ready(&self, ctx: Context, _ready: Ready) {
        for cmd in &self.commands {
            println!("Registering command: {}", cmd.alias());
            let result = ApplicationCommand::create_global_application_command(
                &ctx.http, |app_cmd| {
                    cmd.command_signature(app_cmd)
                }
            ).await;

            if let Err(e) = result {
                eprintln!("{:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Read the token
    let token = read_to_string("TOKEN")
        .expect("Couldn't read TOKEN.");
    let token = token.trim();

    // Read the application id
    let app_id = read_to_string("APPLICATION_ID")
        .expect("Couldn't read APPLICATION_ID")
        .trim()
        .parse::<u64>()
        .expect("Couldn't parse APPLICATION_ID to u64");

    // Build the client
    let mut client = Client::builder(&token)
        .application_id(app_id)
        .event_handler(SlashHandler::new())
        .register_songbird()
        .await
        .expect("Error while building the client.");

    // Start the client
    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
