extern crate gwent_api;

use gwent_api::client::gw_client::Client;
use gwent_api::model::card::CardPage;
use gwent_api::model::request::*;

fn main() {
    println!{"Basic get example"};

    let client = Client::new();

    let c_request = CardPageRequest {
        lang: Lang::fr_FR,
        limit: 150,
        name: None,
        offset: 3,
        if_modified_since: None
    };
    let query = QueryBuilder::query("https://api.gwentapi.com/v0/cards", c_request.to_hash());

    println!("{:?}", query);

    let mut response = client.get(query.unwrap().as_str()).unwrap();

    println!("{}", response.status());
    println!("{:?}", response);

    match response.json::<CardPage>() {
        Err(e) => println!("ERROR: {}", e),
        Ok(card) => println!("{:#?}", card)
    };
}