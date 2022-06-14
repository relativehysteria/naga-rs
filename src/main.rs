use serenity::Client;
use std::fs::read_to_string;

#[tokio::main]
async fn main() {
    // Read the token
    let token = read_to_string("TOKEN")
        .expect("Couldn't read TOKEN.");
    let token = token.trim();

    // Read the APPLICATION_ID
    let app_id = read_to_string("APPLICATION_ID")
        .expect("Couldn't read APPLICATION_ID")
        .trim()
        .parse::<u64>()
        .expect("Couldn't parse APPLICATION_ID as u64");

    // Build the client
    let mut client = Client::builder(&token)
        .application_id(app_id)
        .await
        .expect("Couldn't build the client");

    // Start the client
    if let Err(e) = client.start().await {
        eprintln!("Client error: {e:?}");
    }
}
