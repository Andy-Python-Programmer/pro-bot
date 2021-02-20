use std::fs;
use std::fs::File;

use lazy_static::lazy_static;

use ron::de::from_reader;

use serde::Deserialize;

use ansi_term::Colour;

use serenity::{
    async_trait, framework::standard::macros::group, framework::StandardFramework,
    model::gateway::Ready, prelude::*,
};

mod commands {
    pub mod joke;
    pub mod ping;
    pub mod scissors;
    pub mod help;
    pub mod points;
    pub mod toss;
}

use commands::joke::*;
use commands::ping::*;
use commands::scissors::*;
use commands::help::*;
use commands::points::*;
use commands::toss::*;

lazy_static! {
    static ref JOKES: Vec<String> = fs::read_to_string("jokes.txt")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&c| c.to_owned())
        .collect();
}

#[derive(Deserialize)]
struct Secrets {
    bot_token: String,
}

#[group]
#[description("General bot commands.")]
#[commands(ping, joke, points)]
struct General;

#[group]
#[description("All of the avaliable games.")]
#[commands(scissors, toss)]
struct Games;

const PREFIX: &'static str = "pro!";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!(
            "{} Beep boop boop beep!\n       {} {}\n       {} #{}\n       {} {}",
            Colour::Green.bold().paint("Ready:"),
            Colour::RGB(32, 155, 206).bold().paint("Name:"),
            ready.user.name,
            Colour::RGB(32, 155, 206).bold().paint("Tag:"),
            ready.user.tag().split("#").collect::<Vec<&str>>()[1],
            Colour::RGB(32, 155, 206).bold().paint("Servers:"),
            ready.guilds.len()
        )
    }
}

#[tokio::main]
async fn main() {
    let secrets: Secrets = from_reader(
        File::open(&format!(
            "{}/config/secrets.ron",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("Error opening secrets file."),
    )
    .unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(PREFIX))
        .help(&HELP)
        
        .group(&GENERAL_GROUP)
        .group(&GAMES_GROUP);

    let mut client = Client::builder(&secrets.bot_token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating the client.");

    if let Err(msg) = client.start().await {
        println!("Client error: {:?}", msg)
    }
}
