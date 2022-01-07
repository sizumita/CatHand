use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{gateway::{Ready, Activity}},
    prelude::*,
};
use serenity::model::prelude::*;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;


lazy_static!{
    static ref TWITTER_REGEX: Regex = Regex::new(
        r"https?://twitter.com/(?P<username>[^/\s]+)/status/(?P<tweetId>[0-9]+)(\?s=[0-9]+|)"
    ).unwrap();
    static ref TWITTER_REGEX_ALL: Regex = Regex::new(
        r"^https?://twitter.com/(?P<username>[^/\s]+)/status/(?P<tweetId>[0-9]+)(\?s=[0-9]+|)$"
    ).unwrap();
}

fn is_twitter_embed(embed: &Embed) -> bool {
    match embed.clone().url {
        Some(url) => {
            TWITTER_REGEX_ALL.is_match(url.as_str())
        },
        None => false
    }
}

fn get_twitter_urls(embeds: &Vec<Embed>) -> HashMap<&Option<String>, Vec<String>> {
    let mut map = HashMap::new();
    for embed in embeds {
        if is_twitter_embed(&embed) {
            if !map.contains_key(&embed.url) {
                if embed.clone().image.is_some() {
                    map.insert(&embed.url, vec![embed.clone().image.unwrap().url]);
                }
            } else {
                map.get_mut(&embed.url).unwrap().push(embed.clone().image.unwrap().url)
            }
        }
    }
    map
}

pub async fn send_twitter_buttons(ctx: &Context, message: &Message) {
    let twitter_urls = get_twitter_urls(&message.embeds);
    'outer: for (tweet, urls) in twitter_urls.iter() {
        if urls.len() <= 1 { continue 'outer; }
        let _ = message.channel_id.send_message(
            &ctx.http,
            |msg| {
                msg
                    .reference_message(message)
                    .components(|components| {
                        components
                            .create_action_row(|row| {
                                row
                                    .create_button(|button| {
                                        button
                                            .custom_id("twitter-image")
                                            .style(ButtonStyle::Primary)
                                            .label(format!("Show images ({})", urls.len()))
                                    })
                            })
                    }).content(format!("<{}>", tweet.clone().as_ref().unwrap()))
                    .allowed_mentions(|allowed| {
                        allowed.replied_user(false)
                    })
            }
        ).await;
    }
}


pub async fn show_images(ctx: &Context, interaction: &Interaction, component: &MessageComponent) {
    let reference = interaction.clone().message.unwrap().regular().unwrap().message_reference.unwrap();
    let embeds = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await.unwrap().embeds;
    let all_twitter_urls = get_twitter_urls(
        &embeds
    );
    let tweet_url = interaction.clone().message.unwrap().regular().unwrap()
        .content.replace("<", "").replace(">", "");
    let image_urls = all_twitter_urls.get(&Some(tweet_url)).unwrap();
    let _ = interaction.create_interaction_response(&ctx.http, |response| {
        response.interaction_response_data(|data| {
            data.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                .content(image_urls.join("\n"))
        })
    }).await;
}
