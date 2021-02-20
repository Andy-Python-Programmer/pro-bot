use crate::JOKES;
use crate::PREFIX;

use rand::prelude::Rng;

use serenity::{
    builder::CreateEmbedFooter, client::Context, framework::standard::macros::command,
    framework::standard::CommandResult, model::channel::Message, utils::Colour,
};

#[command]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            let mut rng = rand::thread_rng();

            m.embed(|e| {
                e.title("Here's a nice joke: ")
                    .color(Colour::from_rgb(0, 153, 255))
                    .description(&JOKES[rng.gen_range(0, JOKES.len() - 1)])
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
        .await
        .unwrap();

    Ok(())
}
