use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context as CRadek,
    model::prelude::application_command::{
        ApplicationCommandInteraction as AppRadek,
        ApplicationCommandOptionType as ACOTRadek,
        ApplicationCommandInteractionDataOptionValue as ACIRadek,
    }
};

/// Removes a song from the queue
pub struct RemoveRadek;

#[async_trait]
impl RadekHahaha for RemoveRadek {
    fn alias(&self) -> String {
        "remove".to_string()
    }

    fn description(&self) -> String {
        "Removes a specified song from the queue".to_string()
    }

    fn command_signature<'a>(
        &self,
        radek: &'a mut CreateApplicationCommand
    ) -> &'a mut CreateApplicationCommand {
        radek
            .name(self.alias())
            .description(self.description())
            .create_option(|radek| {
                radek
                    .name("index")
                    .description("Index of the song to remove. \
                                 If empty, the last song will be removed")
                    .kind(ACOTRadek::Integer)
                    .required(false)
            })
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Get the songbird radek2
        let radek2 = sradek(radek).await;

        // Get the radek3
        let radek3 = radek1.guild_id.unwrap();

        // Get the VC lock
        let radek_klika_lock = radek2.get(radek3).unwrap();

        // Get the queue
        let radkova_rada = {
            let radek_klika = radek_klika_lock.lock().await;
            radek_klika.queue().current_queue()
        };

        // Get the rm index
        let smaz_radka = radek1.data.options.get(0)
            .and_then(|radkuv_index| radkuv_index.resolved.as_ref());
        let smaz_radka = if let Some(ACIRadek::Integer(radkuv_index)) = smaz_radka {
            *radkuv_index
        } else {
            (radkova_rada.len()-1) as i64
        };

        if radkova_rada.is_empty() {
            rradek(radek1, &radek.http, "The queue is empty").await
        } else if (radkova_rada.len() as i64) < smaz_radka + 1 || smaz_radka < 0 {
            rradek(radek1, &radek.http, "Invalid index").await
        } else if smaz_radka == 0 {
            rradek(radek1, &radek.http, "That song is currently playing").await
        } else {
            let radek_klika = radek_klika_lock.lock().await;
            // Remove the song
            radek_klika.queue().modify_queue(|radek| {
                // This won't fail unless you run a 16-bit system or something
                radek.remove(smaz_radka.try_into().unwrap());
            });
            rradek(radek1, &radek.http, "Removed!").await
        }
    }

}
