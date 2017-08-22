extern crate gwent_api;
use self::gwent_api::client::gw_client::Client;

#[test]
fn it_retrieves_a_card_by_its_name() {
    match Client::get_card_by_name("Alchemist") {
        Err(e) => panic!("Could not retrieve the card: {}", e),
        Ok(card) => assert_eq!(card.name.as_str(), "Alchemist")
    };
}

#[test]
#[should_panic]
fn it_fails_to_retrieve_a_non_existing_card() {
    match Client::get_card_by_name("Pere Fourras") {
        Err(e) => panic!("Could not retrieve the card: {}", e),
        Ok(card) => assert_eq!(card.name.as_str(), "Pere Fourras")
    };
}

#[test]
fn it_retrieves_default_card_art() {
    let card = Client::get_card_by_name("Cantarella");
    match Client::get_card_default_art(&card.unwrap()) {
        Err(e) => panic!("Card art was not retrieved: {}", e),
        Ok(variation) => assert_eq!(variation.art.thumbnail_image.as_str(), "https://api.gwentapi.com/media/cantarella-thumbnail.png")
    };
}