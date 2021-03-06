extern crate gwent_api;

use gwent_api::client::gw_client::Client;

fn main() {
    println!{"Basic get example"};

    // Existing card
    match Client::get_card_by_name("Alchemist") {
        Err(e) => println!("ERROR: {}", e),
        Ok(card) => println!("{:#?}", card)
    };

    // Non existing card
    match Client::get_card_by_name("ProutProut") {
        Err(e) => println!("ERROR: {}", e),
        Ok(card) => println!("{:#?}", card)
    };

    // First variation of a card
    let card = Client::get_card_by_name("Cantarella");
    match Client::get_card_default_art(&card.unwrap()) {
        Err(e) => println!("ERROR: {}", e),
        Ok(variation) => println!("{:#?}", variation)
    };
}