//! Utility functions.
use std::{
    sync::Arc,
    process::Command,
};
use songbird::{
    Songbird,
    tracks::TrackHandle,
};
use serenity::{
    prelude::*,
    model::prelude::application_command::ApplicationCommandInteraction,
    client::Context,
    builder::CreateEmbed,
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

/// Returns the songbird manager
pub async fn get_songbird(ctx: &Context) -> Arc<Songbird> {
    songbird::get(ctx)
    .await
    .expect("Songbird Voice client placed in at initialisation.")
    .clone()
}

/// Parses a duration into a readable string.
pub fn parse_duration(duration: std::time::Duration) -> String {
    let mut seconds = duration.as_secs();
    let days        = seconds / (24 * 3600);
    seconds        %= 24 * 3600;
    let hours       = seconds / 3600;
    seconds        %= 3600;
    let minutes     = seconds / 60;
    seconds        %= 60;

    let mut result = "".to_string();
    if days    != 0 { result.push_str(&format!("{:02}:", days))    }
    if hours   != 0 { result.push_str(&format!("{:02}:", hours))   }
    result.push_str(&format!("{:02}:{:02}", minutes, seconds));

    result
}

/// Creates an embed for the given `TrackHandle`
pub fn create_embed_for_track(
    track: &TrackHandle,
    embed_title: &str
) -> Option<CreateEmbed> {
    let metadata = track.metadata().clone();

    let source    = metadata.source_url?;
    let channel   = metadata.channel;
    let duration  = metadata.duration;
    let thumbnail = metadata.thumbnail;
    let title     = metadata.title
        .and_then(|title| Some(format!("[{}]({})", title, source)))?;

    // Create the embed
    let mut embed = &mut CreateEmbed(std::collections::HashMap::new());
    embed = embed
        .title(embed_title)
        .description(title);

    // Add the duration
    if let Some(duration) = duration {
        embed = embed.field("Duration", parse_duration(duration), true);
    }

    // Add the channel name
    if let Some(channel) = channel {
        embed = embed.field("Uploader", channel, true);
    }

    // Add the thumbnail
    if let Some(thumbnail) = thumbnail {
        embed = embed.thumbnail(thumbnail);
    }

    Some(embed.clone())
}

/// Extracts the urls from a playlist. Returns a single url if the url doesn't
/// contain a playlist.
pub fn extract_urls(term: &str) -> Vec<String> {
    let extractor = "src/get_playlist_urls.py";
    let output = Command::new(extractor)
        .arg(term)
        .output()
        .expect(&format!("Failed to execute url extractor: {}", extractor));

    let stdout = std::str::from_utf8(&output.stdout)
        .expect("Couldn't parse extracted urls");

    stdout.lines().map(|line| line.trim().to_string()).collect()
}
