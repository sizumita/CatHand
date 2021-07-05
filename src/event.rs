use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{gateway::{Ready, Activity}},
    prelude::*,
};
use crate::Handler;
use crate::amazon::fetch_amazon_data;
use serenity::model::prelude::*;
use regex::Regex;
use serde_json::Value;

lazy_static!{
    static ref AMAZON_REGEX: Regex = Regex::new(
        r"https?://.*?amazon\.co\.jp.*/(gp(/product)?|dp|ASIN)/(?P<asin>[^/?]{10,})\S*"
    ).unwrap();
}

async fn get_or_create_webhook(ctx: &Context, message: &Message) -> Option<Webhook> {
    let webhooks_r = message.channel_id.webhooks(&ctx.http).await;
    match webhooks_r {
        Ok(webhooks) => {
            if webhooks.is_empty() {
                let new_webhook = message.channel_id.create_webhook(&ctx.http, "猫の手").await;
                match new_webhook {
                    Ok(webhook) => Some(webhook),
                    _ => None
                }
            } else {
                webhooks.first().cloned()
            }
        },
        _ => None
    }
}

async fn find_amazon_urls(message: &Message) -> Vec<Value> {
    let mut amazon_data_list = Vec::new();
    for cap in AMAZON_REGEX.captures_iter(&*message.content) {
        let asin = cap.name("asin").unwrap().as_str();
        let data = fetch_amazon_data(format!("https://www.amazon.co.jp/gp/product/{}", asin).as_str()).await.unwrap();
        amazon_data_list.push(data.make_embed());
    };
    amazon_data_list
}

async fn send_amazon_embeds(ctx: &Context, message: &Message) {
    let data = find_amazon_urls(message).await;
    let webhook = get_or_create_webhook(&ctx, &message).await;
    let content = AMAZON_REGEX.replace_all(&message.content, "https://www.amazon.co.jp/gp/product/$asin").to_string();
    match webhook {
        Some(webhook) => {
            let _ = webhook.execute(&ctx.http, false, |hook| {
                hook.embeds(data)
                    .username(format!("{}#{}", &message.author.name, &message.author.discriminator))
                    .avatar_url(&message.author.avatar_url().unwrap_or("".to_string()))
                    .content(content)
            }).await;
        },
        _ => {
            println!("permission error")
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.webhook_id.is_some() { return; }
        if message.author.bot { return; }
        if AMAZON_REGEX.is_match(&*message.content) {
            send_amazon_embeds(&ctx, &message).await;
            message.delete(&ctx.http).await;
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing("/help")).await;
    }
}


#[cfg(test)]
mod tests {
    use super::AMAZON_REGEX;
    #[test]
    fn test_amazon_regex_1() {
        assert!(AMAZON_REGEX.is_match("https://www.amazon.co.jp/gp/product/B097XPYRQ2/"))
    }

    #[test]
    fn test_amazon_regex_2() {
        assert!(AMAZON_REGEX.is_match("https://www.amazon.co.jp/-/en/檜山-良昭-ebook/dp/B016K1K0AW/ref=sr_1_1?adgrpid=104219865276&dchild=1&gclid=Cj0KCQjw16KFBhCgARIsALB0g8IFXLOMIM86fOhk6X8tnvcnN4TzZ0EV4hoJbRG9RSF2QMcAJ8vM8u4aAhulEALw_wcB&hvadid=447991919909&hvdev=c&hvlocphy=20663&hvnetw=g&hvqmt=e&hvrand=11149647611329713892&hvtargid=kwd-739605420901&hydadcr=4073_10899431&jp-ad-ap=0&keywords=日本本土決戦+檜山良昭&qid=1621676966&sr=8-1"))
    }
}
