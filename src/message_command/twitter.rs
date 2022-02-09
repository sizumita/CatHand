use lazy_static::lazy_static;
use std::env;
use regex::Regex;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::prelude::Context;

lazy_static!{
    static ref TWITTER_REGEX: Regex = Regex::new(
        r"https?://twitter.com/(?P<username>[^/\s]+)/status/(?P<tweetId>[0-9]+)"
    ).unwrap();
}

pub async fn twitter_add_like(ctx: &Context, command: &ApplicationCommandInteraction) {
    let token = env::var("BARD_API_TOKEN").unwrap();
    let user_id = command.user.id.0.clone();
    let message = command.data.resolved.messages.values().next().unwrap().clone();
    let _ = command.create_interaction_response(
        ctx.http.as_ref(),
        |response| response
            .kind(InteractionResponseType::DeferredChannelMessageWithSource).interaction_response_data(
            |d| d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
        )
    ).await;
    for capture in TWITTER_REGEX.find_iter(message.content.as_str()) {
        let tweet_id = TWITTER_REGEX.replace(capture.as_str(), "$tweetId");
        let result = reqwest::get(format!("https://bardbot.net/api/twitter/execute?token={token}&user_id={user_id}&tweet_id={tweet_id}&type=like")).await;
        match result {
            Ok(res) => {
                if res.status().is_success() {
                    let _ = command.create_followup_message(
                        ctx.http.as_ref(),
                        |message| message
                            .content(format!("{} をいいねしました。", capture.as_str()))
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    ).await;
                    continue
                }
            }
            _ => {}
        }
        let _ = command.create_followup_message(
            ctx.http.as_ref(),
            |message| message
                .content(format!("{} をいいねできませんでした。認証してください。-> https://bardbot.net/api/twitter/oauth2", capture.as_str()))
                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
        ).await;
    }
}

pub async fn twitter_add_retweet(ctx: &Context, command: &ApplicationCommandInteraction) {
    let token = env::var("BARD_API_TOKEN").unwrap();
    let user_id = command.user.id.0.clone();
    let message = command.data.resolved.messages.values().next().unwrap().clone();
    let _ = command.create_interaction_response(
        ctx.http.as_ref(),
        |response| response
            .kind(InteractionResponseType::DeferredChannelMessageWithSource).interaction_response_data(
            |d| d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
        )
    ).await;
    for capture in TWITTER_REGEX.find_iter(message.content.as_str()) {
        let tweet_id = TWITTER_REGEX.replace(capture.as_str(), "$tweetId");
        let result = reqwest::get(format!("https://bardbot.net/api/twitter/execute?token={token}&user_id={user_id}&tweet_id={tweet_id}&type=retweet")).await;
        match result {
            Ok(res) => {
                if res.status().is_success() {
                    let _ = command.create_followup_message(
                        ctx.http.as_ref(),
                        |message| message
                            .content(format!("{} をリツイートしました。", capture.as_str()))
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    ).await;
                    continue
                }
            }
            _ => {}
        }
        let _ = command.create_followup_message(
            ctx.http.as_ref(),
            |message| message
                .content(format!("{} をリツイートできませんでした。認証してください。-> https://bardbot.net/api/twitter/oauth2", capture.as_str()))
                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
        ).await;
    }
}
