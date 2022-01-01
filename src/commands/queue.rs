use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    builder::CreateEmbed,
    model::prelude::application_command::ApplicationCommandInteraction,
};
use songbird::{
    tracks::TrackHandle,
    input::Metadata,
};

/// Shows the currently enqueued songs. 5 songs per page.
pub struct Queue;

#[async_trait]
impl ApplicationCommandImplementation for Queue {
    fn alias(&self) -> String {
        "queue".to_string()
    }

    fn description(&self) -> String {
        "Shows the currently enqueued songs. 5 songs per page.".to_string()
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError> {
        // Send a response within 3 seconds
        let _ = response(command, &ctx.http, "Fetching the queue...").await;

        // Get the songbird manager
        let manager = get_songbird(ctx).await;

        // Get the guild_id
        let guild_id = command.guild_id.unwrap();

        // Get the current queue
        let handler_lock = manager.get(guild_id).unwrap();
        let queue = {
            handler_lock.lock().await.queue().current_queue()
        };

        // If the queue is empty, return
        if queue.is_empty() {
            command.edit_original_interaction_response(&ctx.http, |resp| {
                resp.content("Queue is empty!")
            }).await?;
            return Ok(());
        }

        let embed = create_queue_embed(&queue, 1);
        if let Some(embed) = embed {
            let _ = command.edit_original_interaction_response(&ctx.http, |r| {
                r.add_embed(embed)
            }).await;
        }
        Ok(())
    }
}

/// Creates the queue embed for a single page.
///
/// This function _does_ check whether the `page` can be created at all.
/// In the case it can't, it returns `None`. Tracks with no title will be shown,
/// but with no title..
fn create_queue_embed(queue: &[TrackHandle], page: usize)
-> Option<CreateEmbed> {
    let tracks_per_page = 5;

    // Get the `page` bounds
    let min_page = 1;
    let max_page = (queue.len() as f32 / tracks_per_page as f32).ceil() as usize;

    // Check the `page` bounds
    if page < min_page || page > max_page || max_page == 0 {
        return None;
    }

    // Get the slice range for the `Vec<TrackHandle>` page.
    // `+ 1` in here because we skip the first (current) song.
    let min_track = (page * tracks_per_page) - tracks_per_page + 1;
    let max_track = usize::min(min_track + tracks_per_page, queue.len());

    // Get the lines to show in the queue page
    let track_lines = &queue[min_track..max_track].iter().enumerate()
        .map(|(i, tr)| format!("`{}` {}\n", i+1, get_queue_line(tr.metadata())))
        .collect::<String>();

    let mut embed = CreateEmbed(std::collections::HashMap::new());
    embed.field("Current", get_queue_line(queue[0].metadata()), false);
    if track_lines != "" {
        embed.field("Next up", track_lines, false);
    }
    Some(embed)
}

/// Creates and returns a single line in the `queue()` page
fn get_queue_line(metadata: &Metadata) -> String {
    let source   = &metadata.source_url;
    let duration = &metadata.duration;
    let title    = &metadata.title;

    let mut ret_str = "".to_string();

    if let Some(dur) = duration {
        ret_str.push_str(&format!("`{}` ", parse_duration(*dur)));
    }

    if let Some(title) = title {
        if let Some(source) = source {
            ret_str.push_str(&format!("[{}]({})", title, source));
        } else {
            ret_str.push_str(&title);
        }
    }

    ret_str
}
