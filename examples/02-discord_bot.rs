#[macro_use] extern crate serenity;

extern crate gwent_api;
extern crate serde;
extern crate serde_json;

use gwent_api::client::gw_client;

use serenity::client::Client;
use std::env;

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"));
    client.with_framework(|f| f
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .on("card", card));

    // start listening for events by starting a single shard
    let _ = client.start();
}

command!(card(_context, message) {
    // TODO clean up the magic number
    let card_name = message.content.clone().split_off(6);
    println!("{}", card_name);

    match gw_client::Client::get_card_by_name(card_name.as_str()) {
        Err(_) => {
            let _ = message.reply("Card name is not recognized.");
        }
        Ok(card) => {
            let json = serde_json::to_string_pretty(&card).unwrap();
            let _ = message.reply(json.as_str());
        }
    }

    
});