//! Implementation of commands.
//!
//! Here's an example of how to implement the `ping` command.
//! 1. Create a new file called `ping.rs`.
//! 2. In there, create a new struct called `Ping`
//! _(though it can be called anything..)_
//! 3. Implement the `ApplicationCommandImplementation` trait for the struct.
//! 4. Export the file in `mod.rs` (`mod ping`).
//! 5. Publicly re-export the command struct in `mod.rs` (`pub use ping::Ping;`).
//! 6. Add the struct to the vector returned by `get_bot_commands()`.
use std::sync::Arc;
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    builder::{
        CreateEmbed,
        CreateApplicationCommand
    },
    model::prelude::application_command::ApplicationCommandInteraction,
};
use songbird::{
    tracks::TrackHandle,
    Songbird,
};

mod ping;
mod join;
mod leave;
mod play;
mod clear;
mod skip;
mod pause;
mod current;
mod song_loop;
pub use ping::Ping;
pub use join::Join;
pub use leave::Leave;
pub use play::Play;
pub use clear::Clear;
pub use skip::Skip;
pub use pause::Pause;
pub use current::Current;
pub use song_loop::SongLoop;

/// Returns a `Vec` of _all_ the `ApplicationCommandImplementation`s this bot
/// has.
/// _New commands have to be registered here._
pub fn get_bot_commands() -> Vec<Box<dyn ApplicationCommandImplementation + Sync + Send>> {
    vec![
        Box::new(Ping),
        Box::new(Join),
        Box::new(Leave),
        Box::new(Play),
        Box::new(Clear),
        Box::new(Skip),
        Box::new(Pause),
        Box::new(Current),
        Box::new(SongLoop),
    ]
}


/// Every command shall implement this trait so that it can be passed
/// to the `EventHandler` in `main.rs`.
#[async_trait]
pub trait ApplicationCommandImplementation {
    /// Returns the command alias of this... command
    fn alias(&self) -> String;

    /// Returns the description of this command.
    /// Empty by default.
    fn description(&self) -> String {
        "".to_string()
    }

    /// The function that is passed to `create_global_application_command`
    fn command_signature<'a>(
        &self,
        command: &'a mut CreateApplicationCommand
    ) -> &'a mut CreateApplicationCommand {
        command
            .name(self.alias())
            .description(self.description())
    }

    /// The function that is called when a command is called in discord.
    /// Example:
    /// ```
    /// if let Interaction::ApplicationCommand(cmd) = interaction {
    ///     match command.data.name.as_str() {
    ///         "something" => Something.handle_interaction(command),
    ///         "other"     => Other.handle_interaction(command),
    ///         _           => unimplemented!(),
    ///     }
    /// }
    /// ```
    async fn handle_interaction(
        &self,
        ctx: &Context, cmd: &ApplicationCommandInteraction
    ) -> Result<(), SerenityError>;
}


/// Creates and sends a simple text interaction response.
async fn response(
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
async fn get_songbird(ctx: &Context) -> Arc<Songbird> {
    songbird::get(ctx)
    .await
    .expect("Songbird Voice client placed in at initialisation.")
    .clone()
}

/// Parses a duration into a readable string.
fn parse_duration(duration: std::time::Duration) -> String {
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
///
/// `title` is the title for the embed.
fn create_embed_for_track(
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
