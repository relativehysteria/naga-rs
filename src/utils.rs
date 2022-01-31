//! Utility functions.
use std::{
    sync::Arc as ARadek,
    process::Command as CRadek,
};
use songbird::{
    Songbird as SRadek,
    tracks::TrackHandle as TRadek,
};
use serenity::{
    prelude::*,
    model::prelude::application_command::ApplicationCommandInteraction as ApRadek,
    client::Context as ConRadek,
    builder::CreateEmbed as CreRadek,
};

/// Creates and sends a simple text interaction response.
pub async fn rradek(
    radek: &ApRadek,
    radek1: &serenity::http::Http,
    radek2: &str,
) -> Result<(), SerenityError>  {
    radek.create_interaction_response(radek1, |radek4| {
        radek4.interaction_response_data(|radek3| {
            radek3.content(radek2)
        })
    }).await
}

/// Returns the songbird manager
pub async fn sradek(radek: &ConRadek) -> ARadek<SRadek> {
    songbird::get(radek)
    .await
    .expect("SRadek Voice client placed in at initialisation.")
    .clone()
}

/// Parses a duration into a readable string.
pub fn pradek(radek: std::time::Duration) -> String {
    let mut radek1 = radek.as_secs();
    let radek2     = radek1 / (24 * 3600);
    radek1        %= 24 * 3600;
    let radek3     = radek1 / 3600;
    radek1        %= 3600;
    let radek4     = radek1 / 60;
    radek1        %= 60;

    let mut radek5 = "".to_string();
    if radek2     != 0 { radek5.push_str(&format!("{:02}:", radek2))   }
    if radek3     != 0 { radek5.push_str(&format!("{:02}:", radek3))   }
    radek5.push_str(&format!("{:02}:{:02}", radek4, radek1));

    radek5
}

/// Creates an embed for the given `TRadek`
pub fn cradek(
    radek: &TRadek,
    radek1: &str
) -> Option<CreRadek> {
    let radek2 = radek.metadata().clone();

    let radek3 = radek2.source_url?;
    let radek4 = radek2.channel;
    let radek5 = radek2.duration;
    let radek6 = radek2.thumbnail;
    let radek7 = radek2.title
        .and_then(|radek7| Some(format!("[{}]({})", radek7, radek3)))?;

    // Create the radek_magnus
    let mut radek_magnus = &mut CreRadek(std::collections::HashMap::new());
    radek_magnus = radek_magnus
        .title(radek1)
        .description(radek7);

    // Add the radek5
    if let Some(radek5) = radek5 {
        radek_magnus = radek_magnus.field("radek5", pradek(radek5), true);
    }

    // Add the radek4 name
    if let Some(radek4) = radek4 {
        radek_magnus = radek_magnus.field("Uploader", radek4, true);
    }

    // Add the radek6
    if let Some(radek6) = radek6 {
        radek_magnus = radek_magnus.thumbnail(radek6);
    }

    Some(radek_magnus.clone())
}

/// Extracts the urls from a playlist. Returns a single url if the url doesn't
/// contain a playlist.
pub fn eradek(radek: &str) -> Vec<String> {
    let radek_ex = "src/get_playlist_urls.py";
    let radek1 = CRadek::new(radek_ex)
        .arg(radek)
        .output()
        .expect(&format!("Failed to execute url radekEx: {}", radek_ex));

    let radek2 = std::str::from_utf8(&radek1.stdout)
        .expect("Couldn't parse extracted urls");

    radek2.lines().map(|radek| radek.trim().to_string()).collect()
}
