use std::env;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::prelude::Context;

pub async fn twitter_oauth(ctx: &Context, command: &ApplicationCommandInteraction) {
    if let ApplicationCommandInteractionDataOptionValue::String(x) = command.data.options.get(0).unwrap().resolved.as_ref().unwrap() {
        let token = env::var("BARD_API_TOKEN").unwrap();
        let user_id = command.user.id.0.clone();
        let result = reqwest::get(format!("https://bardbot.net/api/twitter/connect?token={token}&user_id={user_id}&doc_id={x}")).await;
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    let _ = command.create_interaction_response(
                        &ctx.http,
                        |response| response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(
                                |message| message
                                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
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
            |response| response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(
                    |message| message
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        .content("紐つけに失敗しました。再度お試しください。")
                )
        ).await;
    }
}
