extern crate reqwest;

use model::card::{Card, CardPage};
use model::request::{CardPageRequest, Lang};

macro_rules! get_card {
    ($query: expr, $query_type: ty) => (
        {
            let mut resp = reqwest::get($query)?;
            println!("Query: {:?}", $query);
            println!("Status: {:?}", resp.status());
            match resp.json::<$query_type>() {
                Err(e) => Err(e),
                Ok(card) => Ok(card)
            }
        }
    )
}

pub struct Client { }

impl Client {
    pub fn get_card_by_name(card_name: &str) -> reqwest::Result<Card> {
        let search_card = CardPageRequest {
            lang: Lang::en_US,
            limit: 1,
            name: Some(String::from(card_name)),
            offset: 0,
            if_modified_since: None
        };
        let search_query = search_card.query();
        let mut search_result = get_card!(search_query.as_str(), CardPage).unwrap();
        let card_uri = if search_result.count >= 1 {
                let uri = search_result.results.pop().unwrap();
                uri.href.clone()
            }
            else {
                String::new()
            };
        get_card!(card_uri.as_str(), Card)
    }
}