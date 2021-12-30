use crate::{
    utils::*,
    commands::*,
};
use rand::prelude::*;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    model::prelude::application_command::ApplicationCommandInteraction,
};

/// Shuffles the currently enqueued songs.
///
/// Like literally, those that are ENQUEUED, not those that are still being
/// enqueued. :|
pub struct Shuffle;

#[async_trait]
impl ApplicationCommandImplementation for Shuffle {
    fn alias(&self) -> String {
        "shuffle".to_string()
    }

    fn description(&self) -> String {
        "Shuffles the currently enqueued songs.".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        let guild_id = command.guild_id.unwrap();

        // Shuffle the queue
        let _ = response(command, &ctx.http, "Shuffling...").await;
        {
            let lock = match manager.get(guild_id) {
                Some(lock) => lock,
                None       => {
                    command.edit_original_interaction_response(&ctx.http, |r| {
                        r
                            .content("Aborted!")
                    }).await?;
                    return Ok(());
                },
            };
            lock.lock().await.queue().modify_queue(|q| {
                q.make_contiguous()[1..].shuffle(&mut rand::thread_rng());
            });
        }
        command.edit_original_interaction_response(&ctx.http, |resp| {
            resp
                .content("Shuffled!")
        }).await?;
        Ok(())
    }
}
