use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    model::{gateway::{Ready, Activity}},
    prelude::*,
};
use super::Handler;
use serenity::model::prelude::*;
use regex::Regex;


// static AMAZON_REGEX: Regex = Regex::new(
// r"https?://.*?amazon\\.co\\.jp.*/(gp(/product)?|dp|ASIN)/([^/?]{10,})\\S*"
// ).unwrap();


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.webhook_id.is_some() { return; }

    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing("/help")).await;
    }
}
