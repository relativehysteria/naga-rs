use tracing::{ debug, error };
use serenity::client::{ EventHandler, Context };
use serenity::model::interactions::Interaction;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::gateway::Ready;
use serenity::async_trait;

/// Handler for slash commands
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Only interact with application commands
        debug!("Got interaction: {interaction:?}");
        let interaction = match interaction {
            Interaction::ApplicationCommand(int) => int,
            _ => {
                debug!("Unhandled interaction.");
                return
            },
        };

        if &interaction.data.name == "join" {
            let r = interaction.create_interaction_response(&ctx.http, |resp| {
                resp.interaction_response_data(|msg| {
                    msg.content("Yay!")
                })
            })
            .await;

            if r.is_err() {
                error!("Interaction failed: {r:?}");
            }
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

        // TODO: Now create our commands and register them
        let result = ApplicationCommand::create_global_application_command(
            &ctx.http, |cmd| {
                let cmd = cmd
                    .name("join")
                    .description("join your voice channel");
                debug!("Creating command: {cmd:?}");
                cmd
            }
        ).await;

        if let Err(e) = result {
            error!("Command creation failed: {e:?}");
        }
    }
}
