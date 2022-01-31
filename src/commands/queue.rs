use std::collections::HashMap;
use crate::{
    utils::*,
    commands::*,
};
use serenity::{
    prelude::SerenityError as ErrRadek,
    async_trait,
    client::Context as CRadek,
    builder::{
        CreateEmbed as CreateRadek,
        CreateButton as CreateAnotherRadek,
    },
    futures::StreamExt,
    model::{
        prelude::application_command::ApplicationCommandInteraction as AppRadek,
        interactions::message_component::ButtonStyle as RadkuvStyl,
    },
};
use songbird::{
    tracks::TrackHandle as RadekKlika,
    input::Metadata as RadkuvDenik,
};

/// Shows the currently enqueued songs. 5 songs per page.
pub struct QueueRadek;

impl QueueRadek {
    fn button(radek: &str, radek_id: &str, postizenej_radek: bool) -> CreateAnotherRadek {
        let mut radkuv_zadek = CreateAnotherRadek(HashMap::new());
        let radkuv_zadek = radkuv_zadek
            .style(RadkuvStyl::Primary)
            .label(radek)
            .custom_id(radek_id);
        if postizenej_radek {
            radkuv_zadek.disabled(true).clone()
        } else {
            radkuv_zadek.clone()
        }
    }
    fn prvni_radek(&self, radek: usize) -> CreateAnotherRadek {
        QueueRadek::button("<<", "first", radek == 1)
    }
    fn druhy_radek(&self, radek: usize) -> CreateAnotherRadek {
        QueueRadek::button("<", "prev", radek == 1)
    }
    fn treti_radek(&self, max_radek: usize) -> CreateAnotherRadek {
        QueueRadek::button(">", "next", max_radek == 1)
    }
    fn ctvrty_radek(&self, max_radek: usize) -> CreateAnotherRadek {
        QueueRadek::button(">>", "last", max_radek == 1)
    }
}

#[async_trait]
impl RadekHahaha for QueueRadek {
    fn alias(&self) -> String {
        "queue".to_string()
    }

    fn description(&self) -> String {
        "Shows the currently enqueued songs. 5 songs per page.".to_string()
    }

    async fn handle_interaction(
        &self,
        radek: &CRadek,
        radek1: &AppRadek
    ) -> Result<(), ErrRadek> {
        // Send a response within 3 seconds
        let _ = rradek(radek1, &radek.http, "Fetching the queue...").await;

        // Get the songbird radek2
        let radek2 = sradek(radek).await;

        // Get the radek3
        let radek3 = radek1.guild_id.unwrap();

        // Get the current queue
        let radkuv_zamek = radek2.get(radek3).unwrap();
        let radkova_rada = {
            radkuv_zamek.lock().await.queue().current_queue()
        };

        // If the queue is empty, return
        if radkova_rada.is_empty() {
            radek1.edit_original_interaction_response(&radek.http, |resp| {
                resp.content("Queue is empty!")
            }).await?;
            return Ok(());
        }

        // Data needed for the queue page to work
        let radek_sifra      = 5;  // Tracks per page
        let radkovo_vyrazeni = std::time::Duration::from_secs(60);
        let mut radkova_stranka = 1;
        let max_radkova_stranka = (radkova_rada.len() as f32 / radek_sifra as f32).ceil() as usize;

        // Create the first radkova_stranka.
        // If the first radkova_stranka can't be created, just return.
        let mut radkuv_embed = match vytvor_embed_pro_radkovu_radu(&radkova_rada, radek_sifra, radkova_stranka) {
            Some(radkuv_embed) => radkuv_embed,
            None        => return Ok(()),
        };
        radek1.edit_original_interaction_response(&radek.http, |r| {
            r
                .content("")
                .add_embed(radkuv_embed)
                .components(|comps| {
                    comps.create_action_row(|row| {
                        row
                            .add_button(self.prvni_radek(radkova_stranka))
                            .add_button(self.druhy_radek(radkova_stranka))
                            .add_button(self.treti_radek(max_radkova_stranka))
                            .add_button(self.ctvrty_radek(max_radkova_stranka))
                    })
                })
        }).await?;

        // Create a stream for the button interactions
        let mut radek_je_uz_docela_unavenej_z_tohodle = radek1
            .get_interaction_response(&radek.http).await?
            .await_component_interactions(&radek.shard)
            .timeout(radkovo_vyrazeni)
            .await;

        // Capture interactions
        while let Some(radeeeeek) = radek_je_uz_docela_unavenej_z_tohodle.next().await {
            match radeeeeek.data.custom_id.as_str() {
                "first" => radkova_stranka = 1,
                "prev"  => radkova_stranka = usize::max(radkova_stranka - 1, 1),
                "next"  => radkova_stranka = usize::min(radkova_stranka + 1, max_radkova_stranka),
                "last"  => radkova_stranka = max_radkova_stranka,
                ______  => unreachable!(),
            }

            radkuv_embed = match vytvor_embed_pro_radkovu_radu(&radkova_rada, radek_sifra, radkova_stranka) {
                Some(radkuv_embed) => radkuv_embed,
                None        => continue,
            };
            let _ = rradek(radek1, &radek.http, "").await;
            let _ = radek1.edit_original_interaction_response(&radek.http, |r_jako_radek| {
                r_jako_radek
                    .content("")
                    .add_embed(radkuv_embed)
                    .components(|comps| {
                        comps.create_action_row(|row| {
                            row
                                .add_button(self.prvni_radek(radkova_stranka))
                                .add_button(self.druhy_radek(radkova_stranka))
                                .add_button(self.treti_radek(max_radkova_stranka))
                                .add_button(self.ctvrty_radek(max_radkova_stranka))
                        })
                    })
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
fn vytvor_embed_pro_radkovu_radu(
    radek: &[RadekKlika],
    radku_za_stranku: usize,
    radkova_stranka: usize
) -> Option<CreateRadek> {
    // Get the `radkova_stranka` bounds
    let min_radkova_stranka = 1;
    let max_radkova_stranka = (radek.len() as f32 / radku_za_stranku as f32).ceil() as usize;

    // Check the `radkova_stranka` bounds
    if radkova_stranka < min_radkova_stranka || radkova_stranka > max_radkova_stranka || max_radkova_stranka == 0 {
        return None;
    }

    // Get the slice range for the `Vec<RadekKlika>` radkova_stranka.
    // `+ 1` in here because we skip the first (current) song.
    let min_radkova_pisnicka = (radkova_stranka * radku_za_stranku) - radku_za_stranku + 1;
    let max_radkova_pisnicka = usize::min(min_radkova_pisnicka + radku_za_stranku, radek.len());

    // Get the lines to show in the radek radkova_stranka
    let radkova_pisnicka_lines = &radek[min_radkova_pisnicka..max_radkova_pisnicka].iter()
        .zip(min_radkova_pisnicka..max_radkova_pisnicka)
        .map(|(tr, i)| format!("`{}` {}\n", i, get_queue_line(tr.metadata())))
        .collect::<String>();

    let mut radkuv_embed = CreateRadek(HashMap::new());
    radkuv_embed.field("Current", get_queue_line(radek[0].metadata()), false);
    if radkova_pisnicka_lines != "" {
        radkuv_embed.field("Next up", radkova_pisnicka_lines, false);
    }
    Some(radkuv_embed)
}

/// Creates and returns a single line in the `radek()` radkova_stranka
fn get_queue_line(radkovy_osobni_data: &RadkuvDenik) -> String {
    let radkuv_zdroj   = &radkovy_osobni_data.source_url;
    let jak_dlouho_to_radek_vydrzi = &radkovy_osobni_data.duration;
    let radkuv_nazev    = &radkovy_osobni_data.title;

    let mut radkuv_vysledek = "".to_string();

    if let Some(dur) = jak_dlouho_to_radek_vydrzi {
        radkuv_vysledek.push_str(&format!("`{}` ", pradek(*dur)));
    }

    if let Some(radkuv_nazev) = radkuv_nazev {
        if let Some(radkuv_zdroj) = radkuv_zdroj {
            radkuv_vysledek.push_str(&format!("[{}]({})", radkuv_nazev, radkuv_zdroj));
        } else {
            radkuv_vysledek.push_str(&radkuv_nazev);
        }
    }

    radkuv_vysledek
}
