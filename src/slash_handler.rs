//! Serenity command handler for ApplicationCommands (slash commands)
use tracing::{ warn, debug, error, info };
use serenity::client::{ EventHandler, Context };
use serenity::model::interactions::Interaction;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::gateway::Ready;
use serenity::async_trait;
use crate::commands::{ get_bot_commands, ApplicationCommandImplementation };

/// Handler for slash commands
pub struct Handler {
    /// Commands registered to this handler
    commands: Vec<Box<dyn ApplicationCommandImplementation + Sync + Send>>,
}

impl Handler {
    /// Builds the handler and registers all the commands returned by
    /// `get_bot_commands` as global commands
    pub fn new() -> Self {
        // We're not actually registering anything yet, but it is done
        // later on whenever the bot is `ready()`
        Self { commands: get_bot_commands(), }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Only interact with application commands
        debug!("Got interaction: {interaction:?}");
        let slash_cmd = match interaction {
            Interaction::ApplicationCommand(int) => int,
            _ => {
                debug!("Unhandled interaction.");
                return
            },
        };

        // Get the command struct corresponding to the interaction
        let name = &slash_cmd.data.name;
        let command = match self.commands.iter().find(|x| &x.alias() == name) {
            Some(cmd) => cmd,
            None => {
                warn!("Global slash command doesn't exist: '{name}'. \
                      Perhaps it is registered but not globally?.");
                return
            },
        };

        // All commands have to be used from inside a guild
        match slash_cmd.guild_id {
            Some(id) => id,
            None => { debug!("Not in a guild."); return },
        };

        // Some commands require the user to be in a voice channel.
        if command.requires_voice_chat() {
            // TODO: IMPLEMENT THIS!
            debug!("Not in a voice channel.");
            return;
        }

        // Handle the interaction
        let result = command.handle_interaction(&ctx, &slash_cmd).await;
        if let Err(e) = result {
            error!("While handling a slash command: {e:?}");
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        // All old registered commands are deleted and re-registered.
        // Discord is fucky if we don't do it

        // Get the currently registered commands
        let cmds = ApplicationCommand::get_global_application_commands(&ctx.http)
            .await
            .unwrap();

        // And delete them...
        for cmd in cmds.iter() {
            debug!("Deleting command: {cmd:?}");
            ApplicationCommand::delete_global_application_command(&ctx.http, cmd.id)
                .await
                .unwrap();
        }

        // Now create our commands and register them
        for (idx, cmd) in self.commands.iter().enumerate() {
            info!("Registering command {:02}/{:02}: {}",
                  idx+1, self.commands.len(), cmd.alias());

            let result = ApplicationCommand::create_global_application_command(
                &ctx.http, |app_cmd| { cmd.command_signature(app_cmd) }
            ).await;
            if let Err(e) = result {
                error!("Command creation failed: {e:?}");
            }
        }
    }
}
