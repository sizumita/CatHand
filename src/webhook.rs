use serenity::builder::CreateWebhook;
use serenity::model::prelude::{Message, Webhook};
use serenity::prelude::Context;

pub async fn get_or_create_webhook(ctx: &Context, message: &Message) -> Option<Webhook> {
    let webhooks_r = message.channel_id.webhooks(&ctx.http).await;
    match webhooks_r {
        Ok(webhooks) => {
            if webhooks.is_empty() {
                let new_webhook = message.channel_id.create_webhook(&ctx.http, CreateWebhook::new("猫の手")).await;
                match new_webhook {
                    Ok(webhook) => Some(webhook),
                    _ => None
                }
            } else {
                if webhooks.clone().into_iter().all(|x| x.token == None) {
                    let new_webhook = message.channel_id.create_webhook(&ctx.http, CreateWebhook::new("猫の手")).await;
                    match new_webhook {
                        Ok(webhook) => Some(webhook),
                        _ => None
                    }
                } else {
                    webhooks.first().cloned()
                }
            }
        },
        _ => None
    }
}
