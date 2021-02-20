use serenity::{
    builder::CreateEmbedFooter, client::Context, framework::standard::macros::command,
    framework::standard::CommandResult, model::channel::Message, utils::Colour,
};

use crate::PREFIX;

#[command]
pub async fn points(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Points")
                    .color(Colour::from_rgb(0, 153, 255))
                    .set_footer(
                        CreateEmbedFooter::default()
                            .text(format!(
                                "For more info about all of the commands type {}help",
                                PREFIX
                            ))
                            .to_owned(),
                    );

                return e;
            });

            return m;
        })
        .await?;

    Ok(())
}