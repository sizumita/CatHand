pub mod twitter;

use std::borrow::Cow;
use serenity::http::AttachmentType;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags};
use serenity::prelude::Context;

pub async fn send_message_content_as_file(ctx: &Context, command: &ApplicationCommandInteraction) {
    let message = command.data.resolved.messages.values().next().unwrap().clone();
    let _ = command.create_interaction_response(
        &ctx.http,
        |response| response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(
                |message| message
                    .content("ファイルに変換します...\n生成されたファイルはDMに送信されます。送られてこない場合はDMの受信設定を確認してください。")
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            )
    ).await;
    let data = Cow::from(message.content.as_bytes().to_vec());
    let filename = format!("{}-{}.txt", message.channel_id, message.id);
    let _ = command.user.direct_message(&ctx.http, |m| m
        .add_file(
            AttachmentType::Bytes {
                data,
                filename,
            }
        )
    ).await;
}
