#[macro_use] extern crate serenity;

use serenity::client::Client;
use std::env;

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"));
    client.with_framework(|f| f
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .on("ping", ping));

    // start listening for events by starting a single shard
    let _ = client.start();
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});