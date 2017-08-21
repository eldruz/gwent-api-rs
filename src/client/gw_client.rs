extern crate reqwest;

use model::card::{Card, CardPage, Variation};
use self::reqwest::Url;
use std::collections::HashMap;

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

pub struct QueryBuilder(pub HashMap<String, String>);

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder(HashMap::new())
    }

    pub fn as_url(&self, base_url: &str) -> Result<Url, reqwest::UrlError> {
        Url::parse_with_params(base_url, &self.0)
    }

    pub fn lang<'a>(&'a mut self, language: &str) -> &'a mut Self {
        self.0.insert(String::from("lang"), String::from(language));
        self
    }

    pub fn card_name<'a>(&'a mut self, card_name: &str) -> &'a mut Self {
        self.0.insert(String::from("name"), String::from(card_name));
        self
    }

    pub fn with_arg<'a>(&'a mut self, param: &str, value: &str) -> &'a mut Self {
        self.0.insert(String::from(param), String::from(value));
        self
    }
}

pub struct Client { }

impl Client {
    fn get_query_card_by_name(card_name: &str) -> Result<Url, reqwest::UrlError> {
        QueryBuilder::new()
            .card_name(card_name)
            .as_url("https://api.gwentapi.com/v0/cards/")
    }

    pub fn get_card_by_name(card_name: &str) -> reqwest::Result<Card> {
        let search_query = Self::get_query_card_by_name(card_name).unwrap();
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

    pub fn get_card_default_art(card: &Card) -> reqwest::Result<Variation> {
        let search_query = card.variations[0].href.clone();
        get_card!(search_query.as_str(), Variation)
    }
}