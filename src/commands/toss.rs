use rand::seq::SliceRandom;
use std::time::Duration;

use serenity::{
    builder::CreateEmbedFooter, client::Context, framework::standard::macros::command,
    framework::standard::CommandResult, model::channel::Message, model::channel::ReactionType,
    utils::Colour,
};

use crate::PREFIX;

#[command]
pub async fn toss(ctx: &Context, msg: &Message) -> CommandResult {
    let reactions = vec!["🟠", "❓"];
    let computer = vec!["heads", "tails"]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    let mut m = msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Toss")
                    .color(Colour::from_rgb(0, 153, 255))
                    .description(r#"
                    How to play: Simply just react with a message and also with in 5 seconds as a nice challange.

                    :orange_circle: is heads.

                    :question: is tails.                    
                    
                    "#)
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

    for e in &reactions {
        m.react(&ctx, ReactionType::Unicode(e.to_owned().to_owned()))
            .await?;
    }

    if let Some(v) = &m
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(5))
        .author_id(msg.author.id)
        .await
    {
        let mut user = v.as_inner_ref().emoji.as_data();

        if user == reactions[0] {
            user = String::from("heads");
        } else if user == reactions[1] {
            user = String::from("tails");
        }

        let title: &str;
        let description: String;

        if user == computer {
            title = "You win!";
            description = "You guessed the right answer!".to_owned();
        }

        else {
            title = "You loose!";
            description = "That was not the right answer!".to_owned();
        }

        m.edit(&ctx, |m| {
            m.embed(|e| {
                e.title(&title)
                    .color(Colour::from_rgb(0, 153, 255))
                    .description(&description)
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
    } else {
        m.delete_reactions(&ctx).await?;

        m.edit(&ctx, |m| {
            m.embed(|e| {
                e.title("Time is up")
                    .color(Colour::from_rgb(0, 153, 255))
                    .description(r#"
                    Time is up! You were not able to complete this game. Maybe try playing the game once again!
                    "#)
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
        }).await?;
    }

    Ok(())
}
