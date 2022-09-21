use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use moka::future::Cache;
use serenity::builder::{CreateActionRow, CreateAllowedMentions, CreateButton, CreateComponents, CreateInteractionResponse, CreateInteractionResponseData, CreateMessage};
use serenity::model::application::component::ButtonStyle;
use serenity::model::channel::{MessageFlags, ReactionType};
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::prelude::{Embed, Message};
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::Timestamp;
use serenity::prelude::Context;


lazy_static!{
    static ref TWITTER_REGEX_ALL: Regex = Regex::new(
        r"^https?://twitter.com/(?P<username>[^/\s]+)/status/(?P<tweetId>[0-9]+)(\?s=[0-9]+|)"
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
    if !message.reactions.iter().filter(|x| x.reaction_type == ReactionType::Unicode("\u{1f9f2}".to_string())).collect::<Vec<_>>().is_empty() {
        return;
    }
    let twitter_urls = get_twitter_urls(&message.embeds);
    'outer: for (tweet, urls) in twitter_urls.iter() {
        if urls.len() <= 1 { continue 'outer; }
        let _ = message.channel_id.send_message(
            &ctx.http,
            CreateMessage::new()
                .reference_message(message)
                .components(CreateComponents::new()
                    .set_action_row(
                        CreateActionRow::new()
                            .add_button(
                                CreateButton::new()
                                    .custom_id("twitter-image")
                                    .style(ButtonStyle::Primary)
                                    .label(format!("Show images ({})", urls.len()))
                            )
                    )
                ).content(format!("<{}>", tweet.clone().as_ref().unwrap()))
                .allowed_mentions(
                    CreateAllowedMentions::new()
                        .replied_user(false)
                )
        ).await;
    }
    let _ = message.react(
        &ctx,
        ReactionType::Unicode("\u{1f9f2}".to_string())
    ).await;
}


pub async fn show_images(ctx: &Context, component: &MessageComponentInteraction) {
    let reference = component.clone().message.message_reference.unwrap();
    let embeds = reference.channel_id.message(&ctx.http, reference.message_id.unwrap()).await.unwrap().embeds;
    let all_twitter_urls = get_twitter_urls(
        &embeds
    );
    let tweet_url = component.clone().message
        .content.replace("<", "").replace(">", "");
    let image_urls = all_twitter_urls.get(&Some(tweet_url)).unwrap();
    let _ = component.create_interaction_response(&ctx.http, CreateInteractionResponse::new()
        .interaction_response_data(
            CreateInteractionResponseData::new()
                .flags(MessageFlags::EPHEMERAL)
                .content(image_urls.join("\n"))
        )).await;
}
