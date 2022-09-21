extern crate regex;

mod event;
mod amazon;
mod twitter;
mod percent_encoding;
mod webhook;
mod decoder;
mod message_url;
mod message_command;
mod slash_command;

use std::env;

use serenity::{
    prelude::*,
};
use serenity::model::prelude::{ChannelId, GatewayIntents, MessageId};
use moka::future::Cache;
use serenity::model::Timestamp;

struct Handler {
    twitter_cache: Cache<(ChannelId, MessageId), Timestamp>
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS | GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler {
            twitter_cache: Cache::new(100000)
        })
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}