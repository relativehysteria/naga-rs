use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context as CRadek,
    model::prelude::{
        application_command::ApplicationCommandInteraction as AppRadek,
    }
};

/// Joins the bot into a voice channel.
pub struct JoinRadek;

#[async_trait]
impl RadekHahaha for JoinRadek {
    fn alias(&self) -> String {
        "join".to_string()
    }

    fn description(&self) -> String {
        "Joins your voice channel.".to_string()
    }

    fn requires_voice_chat(&self) -> bool {
        false
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Get the songbird radek3
        let radek3 = sradek(radek).await;

        // Get the guild id
        let radek4 = radek1.guild_id.unwrap();

        // Get the guild
        let radek5 = match radek.cache.guild(radek4).await {
            Some(radek5) => radek5,
            None        => return Ok(()),
        };

        // Get the voice channel id
        let radek6 = match radek5.voice_states
            .get(&radek1.user.id)
            .and_then(|radek| radek.channel_id)
        {
            Some(radek) => radek,
            None        => return rradek(radek1, &radek.http,
                                           "Not in a voice channel").await,
        };

        // Bots shouldn't reconnect between channels
        if radek3.get(radek4).is_some() {
            return rradek(radek1, &radek.http,
                            "Already connected to a voice channel").await;
        }

        // Join the VC
        let (_, radek_stat) = radek3.join(radek4, radek6).await;
        if let Err(e) = radek_stat {
            let e_radek = format!("e_radekor while joining a voice channel: {:?}", e);
            eprintln!("{}", e_radek);
            rradek(radek1, &radek.http, &e_radek).await
        } else {
            rradek(radek1, &radek.http, "I'm in B)").await
        }
    }
}
