#[macro_use] extern crate serenity;

extern crate gwent_api;
extern crate serde;
extern crate serde_json;

use gwent_api::client::gw_client;

use serenity::client::Client;
use serenity::model::Mentionable;
use std::env;

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"));
    client.with_framework(|f| f
        .configure(|c| c.prefix("!"))
        .on("card", card));

    // start listening for events by starting a single shard
    let _ = client.start();
}

command!(card(_context, message) {
    let card_name = message.content.clone().split_off(6);
    println!("{}", card_name);

    match gw_client::Client::get_card_by_name(card_name.as_str()) {
        Err(_) => {
            let _ = message.reply("Card name is not recognized.");
        }
        Ok(card) => {
            let art_uri = gw_client::Client::get_card_default_art(&card).unwrap().art.thumbnail_image;
            let _ = message.channel_id.send_message(|m| m
                    .content(message.author.mention().as_str())
                    .embed(|e| e
                        .title(card.name.as_str())
                        .image(art_uri.as_str())
                        .description(card.info.as_str())));
        }
    }

    
});