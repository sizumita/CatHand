use std::borrow::Cow;
use std::fmt::format;
use serenity::http::AttachmentType;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{Attachment, InteractionApplicationCommandCallbackDataFlags, Message};
use serenity::prelude::Context;

pub async fn send_message_content_as_file(ctx: &Context, command: &ApplicationCommandInteraction) {
    let message = command.data.resolved.messages.values().next().unwrap();
    let _ = command.create_interaction_response(
        &ctx.http,
        |response| response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(
                |message| message
                    .content("ファイルに変換します...")
            )
    ).await;
    let _ = command.channel_id.send_message(&ctx.http, |msg| msg
        .add_file(
            AttachmentType::Bytes {
                data: Cow::from(message.content.as_bytes()),
                filename: format!("{}-{}.txt", message.channel_id, message.id)
            }
        )
    ).await;
}
