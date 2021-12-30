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
                // All commands have to be used from inside a guild.
                let guild_id = match interaction.guild_id {
                    Some(id) => id,
                    None     => {
                        let _ = response(&interaction, &ctx.http,
                                 "Command not used from a guild").await;
                        return;
                    },
                };

                // Some commands require the user to be in a voice channel.
                if command.requires_voice_chat() {
                    if get_songbird(&ctx).await.get(guild_id).is_none() {
                        let _ = response(&interaction, &ctx.http,
                                 "Not in a voice channel").await;
                        return;
                    }
                }

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
        for (idx, cmd) in self.commands.iter().enumerate() {
            println!(
                "Registering command {:02}/{:02}: {} ",
                idx+1, self.commands.len(), cmd.alias(),
            );
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
