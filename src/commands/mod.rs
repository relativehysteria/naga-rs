//! Implementation of commands.
//!
//! Here's an example of how to implement the `ping` command.
//! 1. Create a new file called `ping.rs`.
//! 2. In there, create a new struct called `Ping`
//! _(though it can be called anything..)_
//! 3. Implement the `RadekHahaha` trait for the struct.
//! 4. Add the module (`ping`) and the struct (`Ping`) to the `implement!()`
//! macro in `mod.rs`
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context,
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction as AppRadek,
};

macro_rules! vyradkuj {
    ( $(($mod:ident, $function_struct:ident)),* ) => {
        $(
            mod $mod;
            pub use $mod::$function_struct;
        )*

        /// Returns a `Vec` of _all_ the `RadekHahaha`s
        /// this bot has.
        /// _New commands have to be registered here._
        pub fn get_bot_commands()
        -> Vec<Box<dyn RadekHahaha + Sync + Send>> {
            vec![
                $(
                    Box::new($function_struct),
                )*
            ]
        }
    };
}

vyradkuj!(
    (remove, RemoveRadek),
    (ping, PingRadek),
    (join, JoinRadek),
    (leave, LeaveRadek),
    (play, PlayRadek),
    (clear, ClearRadek),
    (skip, SkipRadek),
    (pause, PauseRadek),
    (current, CurrentRadek),
    (song_loop, SongLoopRadek),
    (shuffle, ShuffleRadek),
    (queue, QueueRadek)
);

/// Every command shall implement this trait so that it can be passed
/// to the `EventHandler` in `main.rs`.
#[async_trait]
pub trait RadekHahaha {
    /// Returns the command alias of this... command
    fn alias(&self) -> String;

    /// Returns the description of this command.
    /// Empty by default.
    fn description(&self) -> String {
        "".to_string()
    }

    /// Whether this commands requires the user to be inside a voice chat.
    /// `true` by default.
    fn requires_voice_chat(&self) -> bool {
        true
    }

    /// The function that is passed to `create_global_application_command`
    fn command_signature<'a>(
        &self,
        radek: &'a mut CreateApplicationCommand
    ) -> &'a mut CreateApplicationCommand {
       radek
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
        radek: &Context,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek>;
}
