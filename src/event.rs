use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{gateway::{Ready}},
    prelude::*,
};
use crate::Handler;
use crate::amazon::{send_amazon_embeds, AMAZON_REGEX};
use crate::twitter::{send_twitter_buttons, show_images};
use crate::percent_encoding::{replace_all_match, send_replaced};
use serenity::model::prelude::*;
use regex::Regex;
use serenity::gateway::ActivityData;
use crate::decoder::{Decoder, EucJpDecoder};
use crate::message_url::{get_message_urls, send_message_previews};

lazy_static!{
    static ref TWITTER_REGEX: Regex = Regex::new(
        r"https?://twitter.com/(?P<username>[^/\s]+)/status/(?P<tweetId>[0-9]+)"
    ).unwrap();
    static ref WIKIPEDIA_REGEX: Regex = Regex::new(
        r"https?://ja\.wikipedia\.org/wiki/[^/\s]+"
    ).unwrap();
    static ref SEESAAWIKI_REGEX: Regex = Regex::new(
        r"https://seesaawiki.jp/(?P<wikiname>[^/\s]+)/d/[^/\s]+"
    ).unwrap();
}

fn replace_all_regex(_: &Context, message: &Message) -> Option<String> {
    replace_all_match(
        vec![&WIKIPEDIA_REGEX, &SEESAAWIKI_REGEX],
        message.content.clone(),
        |content| {
            let s = content.clone().split("/").last().unwrap();
            if WIKIPEDIA_REGEX.is_match(content) {
                return content.to_string();
            }
            if SEESAAWIKI_REGEX.is_match(content.clone()) {
                return format!("https://bardbot.net/api/seesaa/{}/{}", SEESAAWIKI_REGEX.replace(content.clone(), "$wikiname"), EucJpDecoder::decode(s.clone()));
            }
            return "".to_string();
        }
    )
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.webhook_id.is_some() { return; }
        if message.author.bot { return; }
        if message.guild_id.is_some() {
            let messages = get_message_urls(&ctx, message.content.clone().as_str(), message.guild_id.unwrap()).await;
            if !messages.is_empty() {
                send_message_previews(&ctx, &message, messages).await;
            }
        }

        if AMAZON_REGEX.is_match(&*message.content) {
            let result = send_amazon_embeds(&ctx, &message).await;
            if result.is_ok() {
                let _ = message.delete(&ctx.http).await;
            }
        };
        if message.embeds.len() != 0 {
            send_twitter_buttons(&ctx, &message).await;
        }
        if let Some(replaced) = replace_all_regex(&ctx, &message) {
            send_replaced(&ctx, &message, replaced).await;
            let _ = message.delete(&ctx.http).await;
        }
    }

    async fn message_update(&self, ctx: Context,
                            _old_if_available: Option<Message>,
                            _new: Option<Message>,
                            message: MessageUpdateEvent) {
        if message.embeds.is_some() {
            let embeds = message.clone().embeds.unwrap();
            if embeds.len() != 0 {
                let base_message = ctx.http.get_message(message.channel_id.0.get(), message.id.0.get()).await;
                if base_message.is_err() { return; }
                send_twitter_buttons(&ctx, &base_message.unwrap()).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Some(ActivityData::playing("猫の手も借りたい")));
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::MessageComponent(component) => {
                match &*component.data.custom_id {
                    "twitter-image" => {
                        show_images(&ctx, &component).await;
                    }
                    _ => {}
                }
            }
            Interaction::ApplicationCommand(command) => {
                match &*command.data.name {
                    "テキストの内容をファイルにする" => {
                        crate::message_command::send_message_content_as_file(&ctx, &command).await;
                    }
                    "Twitterでいいねする" => {
                        crate::message_command::twitter::twitter_add_like(&ctx, &command).await;
                    }
                    "Twitterでリツイートする" => {
                        crate::message_command::twitter::twitter_add_retweet(&ctx, &command).await;
                    }
                    "oauth" => {
                        crate::slash_command::twitter_oauth(&ctx, &command).await;
                    }
                    _ => {}
                }
            }
            _ => {}
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
