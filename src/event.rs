use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{gateway::{Ready, Activity}},
    prelude::*,
};
use crate::Handler;
use crate::amazon::{send_amazon_embeds, AMAZON_REGEX};
use crate::twitter::{send_twitter_buttons, show_images};
use serenity::model::prelude::*;
use regex::Regex;
use serde_json::Value;

lazy_static!{
    static ref TWITTER_REGEX: Regex = Regex::new(
        r"https?://twitter.com/(?P<username>[^/\s]+)/status/(?P<tweetId>[0-9]+)"
    ).unwrap();
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.webhook_id.is_some() { return; }
        if message.author.bot { return; }
        if AMAZON_REGEX.is_match(&*message.content) {
            let _ = send_amazon_embeds(&ctx, &message).await;
            let _ = message.delete(&ctx.http).await;
        };
        if message.embeds.len() != 0 {
            send_twitter_buttons(&ctx, &message).await;
        }
    }

    async fn message_update(&self, ctx: Context, message: MessageUpdateEvent) {
        if message.embeds.is_some() {
            let embeds = message.clone().embeds.unwrap();
            if embeds.len() != 0 {
                let base_message = ctx.http.get_message(message.channel_id.0, message.id.0).await;
                if base_message.is_err() { return; }
                send_twitter_buttons(&ctx, &base_message.unwrap()).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing("猫の手も借りたい")).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.kind == InteractionType::MessageComponent {
            if let Some(data) = interaction.data.as_ref() {
                match data {
                    InteractionData::MessageComponent(component) => {
                        match &*component.custom_id {
                            "twitter-image" => {
                                show_images(&ctx, &interaction, &component).await;
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
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
