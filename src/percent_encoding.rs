use regex::Regex;
use serenity::builder::ExecuteWebhook;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::webhook::get_or_create_webhook;

pub fn get_all_match<F>(regexes: Vec<&Regex>, content: String, decoder: F) -> Vec<(String, String)>
    where F: Fn(&str) -> String {
    let mut matchs: Vec<(String, String)> = vec![];
    for regex in regexes {
        regex.find_iter(&*content).for_each(
            |m| {
                matchs.push(
                    (m.clone().as_str().to_string(), decoder(m.clone().as_str()))
                );
            }
        );
    }
    return matchs;
}

pub fn replace_all_match<F>(regexes: Vec<&Regex>, content: String, decoder: F) -> Option<String> where F: Fn(&str) -> String {
    let matches = get_all_match(regexes, content.clone(), decoder);
    if matches.is_empty() {
        return None;
    }
    let mut replaced = content.clone();
    for (before, after) in matches {
        replaced = replaced.replace(&*before, &*urlencoding::decode(&*after).unwrap().to_string())
    }
    return Some(replaced);
}

pub async fn send_replaced(ctx: &Context, message: &Message, replaced: String) {
    let webhook = get_or_create_webhook(&ctx, &message).await;
    match webhook {
        Some(webhook) => {
            let _ = webhook.execute(&ctx.http, false, ExecuteWebhook::new()
                .content(replaced)
                .avatar_url(message.author.avatar_url().unwrap_or(message.author.default_avatar_url().clone()))
                .username(format!("{}#{}", message.author.name, message.author.discriminator))
            ).await;
        }
        _ => {
            println!("permission error")
        }
    }
}
