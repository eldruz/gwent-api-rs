extern crate gwent_api;

use gwent_api::client::gw_client::Client;
use gwent_api::model::card::Card;

fn main() {
    println!{"Basic get example"};

    let client = Client::new();

    let mut response = client.get("https://api.gwentapi.com/v0/cards/gycZySRxV-2wgUFMzdNuFw").unwrap();

    println!("{}", response.status());
    println!("{:?}", response);

    match response.json::<Card>() {
        Err(e) => println!("ERROR: {}", e),
        Ok(card) => println!("{:#?}", card)
    };
}