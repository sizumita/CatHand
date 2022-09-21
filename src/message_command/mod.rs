pub mod twitter;

use std::borrow::Cow;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseData, CreateMessage};
use serenity::model::channel::MessageFlags;
use serenity::model::prelude::{AttachmentType, InteractionResponseType};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn send_message_content_as_file(ctx: &Context, command: &ApplicationCommandInteraction) {
    let message = command.data.resolved.messages.values().next().unwrap().clone();
    let _ = command.create_interaction_response(
        &ctx.http,
        CreateInteractionResponse::new()
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(
                CreateInteractionResponseData::new()
                    .content("ファイルに変換します...\n生成されたファイルはDMに送信されます。送られてこない場合はDMの受信設定を確認してください。")
                    .flags(MessageFlags::EPHEMERAL)
            )
    ).await;
    let data = Cow::from(message.content.as_bytes().to_vec());
    let filename = format!("{}-{}.txt", message.channel_id, message.id);
    let _ = command.user.direct_message(&ctx.http,
    CreateMessage::new()
        .add_file(
            AttachmentType::Bytes {
                data,
                filename,
            }
        )
    ).await;
}
