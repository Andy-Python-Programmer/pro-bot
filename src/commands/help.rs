use std::collections::HashSet;

use serenity::{utils::Colour, client::Context, framework::standard::CommandResult, framework::standard::{Args, CommandGroup, HelpOptions, macros::help}, framework::standard::help_commands, model::channel::Message, model::id::UserId};

#[help]
#[strikethrough_commands_tip_in_guild("")]
#[individual_command_tip("To get help with an individual command or category, pass its name as an argument to this command.")]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let mut help_opts = help_options.clone();

    // Changing the color of the embed
    help_opts.embed_success_colour = Colour::from_rgb(0, 153, 255);
    help_opts.embed_error_colour = Colour::from_rgb(255, 30, 30);

    let _ = help_commands::with_embeds(context, msg, args, &help_opts, groups, owners).await;

    Ok(())
}