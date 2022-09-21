use std::env;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseData};
use serenity::model::channel::MessageFlags;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::{InteractionResponseType};
use serenity::prelude::Context;

pub async fn twitter_oauth(ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(x) = command.data.options.get(0).unwrap().value.clone() {
        let token = env::var("BARD_API_TOKEN").unwrap();
        let user_id = command.user.id.0.clone();
        let result = reqwest::get(format!("https://bardbot.net/api/twitter/connect?token={token}&user_id={user_id}&doc_id={x}")).await;
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    let _ = command.create_interaction_response(
                        &ctx.http,
                        CreateInteractionResponse::new()
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(
                                CreateInteractionResponseData::new()
                                    .flags(MessageFlags::EPHEMERAL)
                                    .content("紐つけられました！")
                            )
                    ).await;
                    return;
                }
            }
            _ => {}
        }
        let _ = command.create_interaction_response(
            &ctx.http,
            CreateInteractionResponse::new()
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(
                    CreateInteractionResponseData::new()
                        .flags(MessageFlags::EPHEMERAL)
                        .content("紐つけに失敗しました。再度お試しください。")
                )
        ).await;
    }
}
