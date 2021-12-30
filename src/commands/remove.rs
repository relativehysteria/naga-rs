use crate::commands::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::{
        ApplicationCommandInteraction,
        ApplicationCommandOptionType,
        ApplicationCommandInteractionDataOptionValue as ACIDOV,
    }
};

/// Removes a song from the queue
pub struct Remove;

#[async_trait]
impl ApplicationCommandImplementation for Remove {
    fn alias(&self) -> String {
        "remove".to_string()
    }

    fn description(&self) -> String {
        "Removes a specified song from the queue".to_string()
    }

    fn command_signature<'a>(
        &self,
        command: &'a mut CreateApplicationCommand
    ) -> &'a mut CreateApplicationCommand {
        command
            .name(self.alias())
            .description(self.description())
            .create_option(|opt| {
                opt
                    .name("index")
                    .description("Index of the song to remove. \
                                 If empty, the last song will be removed")
                    .kind(ApplicationCommandOptionType::Integer)
                    .required(false)
            })
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        let guild_id = match command.guild_id {
            Some(id) => id,
            None     => return response(command, &ctx.http,
                                        "Command not used from a guild").await,
        };

        // Get the VC lock
        let handler_lock = match manager.get(guild_id) {
            Some(lock) => lock,
            None       => return response(command, &ctx.http,
                                          "Not in a voice channel").await,
        };

        // Get the queue
        let queue = {
            let handler = handler_lock.lock().await;
            handler.queue().current_queue()
        };

        // Get the rm index
        let rm_idx = command.data.options.get(0)
            .and_then(|idx| idx.resolved.as_ref());
        let rm_idx = if let Some(ACIDOV::Integer(idx)) = rm_idx {
            *idx
        } else {
            (queue.len()-1) as i64
        };

        if queue.is_empty() {
            response(command, &ctx.http, "The queue is empty").await
        } else if (queue.len() as i64) < rm_idx + 1 || rm_idx < 0 {
            response(command, &ctx.http, "Invalid index").await
        } else if rm_idx == 0 {
            response(command, &ctx.http, "That song is currently playing").await
        } else {
            let handler = handler_lock.lock().await;
            // Remove the song
            handler.queue().modify_queue(|q| {
                // This won't fail unless you run a 16-bit system or something
                q.remove(rm_idx.try_into().unwrap());
            });
            response(command, &ctx.http, "Removed!").await
        }
    }

}
