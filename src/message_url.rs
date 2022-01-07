use chrono::Utc;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::builder::Timestamp;
use serenity::model::id::GuildId;
use serenity::model::prelude::{ChannelId, Message, MessageId};
use serenity::prelude::Context;

lazy_static!{
    static ref MESSAGE_URL_REGEX: Regex = Regex::new(
        r"https?://(?:(ptb|canary|www)\.)?discord(?:app)?\.com/channels/(?P<guild_id>[0-9]{15,20}|@me)/(?P<channel_id>[0-9]{15,20})/(?P<message_id>[0-9]{15,20})/?"
    ).unwrap();
}

pub async fn get_message_urls(ctx: &Context, content: &str, guild_id: GuildId) -> Vec<Message> {
    let mut messages: Vec<Message> = vec![];
    for capture in MESSAGE_URL_REGEX.captures_iter(content) {
        if capture.name("guild_id").unwrap().as_str().to_string() != guild_id.to_string() {
            continue
        }
        let channel_id = ChannelId::from(capture.name("channel_id").unwrap().as_str().parse::<u64>().unwrap());
        let message_id = MessageId::from(capture.name("message_id").unwrap().as_str().parse::<u64>().unwrap());
        let message = ctx.cache.message(channel_id.clone(), message_id.clone()).await.unwrap_or(
                channel_id.message(&ctx.http, message_id.clone()).await.unwrap()
            );
        messages.push(message);
    }
    messages
}

pub async fn send_message_previews(ctx: &Context, reference: &Message, messages: Vec<Message>) {
    let guild = reference.guild(ctx).await;
    let mut sent: Vec<MessageId> = vec![];

    for message in messages {
        if sent.contains(&message.id) {
            continue
        }
        let _ = reference.channel_id.send_message(
            &ctx.http,
            |msg| msg
                .reference_message(reference)
                .allowed_mentions(
                    |mentions| mentions.replied_user(false)
                )
                .embed(
                    |embed| {
                        if message.content.len() > 100 {
                            embed.description(format!("{}...", message.content.chars().take(100).collect::<String>()));
                        } else {
                            embed.description(message.content.clone());
                        }
                        embed
                            .author(
                                |author| author
                                    .name(message.author.name.clone())
                                    .icon_url(format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=1024", message.author.id, message.clone().author.avatar.unwrap()))
                                    .url(message.link())
                            )
                            .footer(
                                |footer| footer
                                    .icon_url(guild.clone().unwrap().icon_url().unwrap_or("".to_string()))
                                    .text(guild.clone().unwrap().name)
                            )
                            .timestamp(Timestamp::from(message.timestamp.to_rfc3339()))
                    }
                )
        ).await;
        sent.push(message.id.clone());
    }
}
