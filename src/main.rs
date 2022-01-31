use naga_rs::{
    commands::*,
    utils::*,
};
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


/// Handler for slash commands
struct Radek {
    /// Commands registered into this handler
    radek: Vec<Box<dyn RadekHahaha + Sync + Send>>,
}

impl Radek {
    /// Builds the handler and puts all the commands returned by
    /// `get_bot_commands` into the inner `commands` vec
    fn new() -> Self {
        Self { radek: get_bot_commands(), }
    }
}

#[async_trait]
impl EventHandler for Radek {
    /// Handle slash commands
    async fn interaction_create(&self, radek1: Context, radek2: Interaction) {
        if let Interaction::ApplicationCommand(radek2) = radek2 {

            // Find the command, if it exists.
            let radek3 = &radek2.data.name;
            let radek4 = self.radek.iter().find(|x| &x.alias() == radek3);

            // And execute it.
            if let Some(radek4) = radek4 {
                // All radek4s have to be used from inside a guild.
                let radek5 = match radek2.guild_id {
                    Some(id) => id,
                    None     => {
                        let _ = rradek(&radek2, &radek1.http,
                                 "radek4 not used from a guild").await;
                        return;
                    },
                };

                // Some radek4s require the user to be in a voice channel.
                if radek4.requires_voice_chat() {
                    if sradek(&radek1).await.get(radek5).is_none() {
                        let _ = rradek(&radek2, &radek1.http,
                                 "Not in a voice channel").await;
                        return;
                    }
                }

                // Errors are handled by the CALLER.
                let radek6 = radek4.handle_interaction(&radek1, &radek2).await;
                if let Err(radek7) = radek6 {
                    eprintln!("Error while handling a slash radek4: {:?}", radek7);
                }
            }
        }
    }

    /// Register the `ApplicationCommand`s into the bot when its internal
    /// `GuildId` cache is ready.
    async fn ready(&self, radek1: Context, _ready: Ready) {
        for (radek2, radek3) in self.radek.iter().enumerate() {
            println!(
                "Registering command {:02}/{:02}: {} ",
                radek2+1, self.radek.len(), radek3.alias(),
            );
            let radek5 = ApplicationCommand::create_global_application_command(
                &radek1.http, |radek4| {
                    radek3.command_signature(radek4)
                }
            ).await;

            if let Err(radek5) = radek5 {
                eprintln!("{:?}", radek5);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Read the token
    let radek = read_to_string("TOKEN")
        .expect("Couldn't read TOKEN.");
    let radek = radek .trim();

    // Read the application id
    let radek1 = read_to_string("APPLICATION_ID")
        .expect("Couldn't read APPLICATION_ID")
        .trim()
        .parse::<u64>()
        .expect("Couldn't parse APPLICATION_ID to u64");

    // Build the client
    let mut radek2 = Client::builder(&radek)
        .application_id(radek1)
        .event_handler(Radek::new())
        .register_songbird()
        .await
        .expect("Error while building the client.");

    // Start the client
    if let Err(radek3) = radek2.start().await {
        println!("Client error: {:?}", radek3);
    }
}
