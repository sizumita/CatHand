use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    model::{gateway::{Ready, Activity}},
    prelude::*,
};
use super::Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing("/help")).await;
    }
}
