//! Implementation of commands.
//!
//! Here's an example of how to implement the `ping` command.
//! 1. Create a new file called `ping.rs`.
//! 2. In there, create a new struct called `Ping`
//! _(though it can be called anything..)_
//! 3. Implement the `ApplicationCommandImplementation` trait for the struct.
//! 4. Export the file in `mod.rs` (`mod ping`).
//! 5. Publically re-export the command struct in `mod.rs` (`pub use ping::Ping;`).
//! 6. Add the struct to the vector returned by `get_bot_commands()`.
use serenity::{
    prelude::SerenityError,
    async_trait,
    client::Context,
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction,
};

mod ping;
mod join;
pub use ping::Ping;
pub use join::Join;

/// Returns a `Vec` of _all_ the `ApplicationCommandImplementation`s this bot
/// has.
/// _New commands have to be registered here._
pub fn get_bot_commands() -> Vec<Box<dyn ApplicationCommandImplementation + Sync + Send>> {
    vec![
        Box::new(Ping),
        Box::new(Join),
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
