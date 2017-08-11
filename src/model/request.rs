use std::collections::HashMap;

extern crate reqwest;

use self::reqwest::{Response};
use model::card::Card;

pub enum Lang {
    en_US,
    de_DE,
    es_ES,
    es_MX,
    fr_FR,
    it_IT,
    ja_JP,
    pl_PL,
    pt_BR,
    ru_RU,
}

impl Lang {
    pub fn as_str(&self) -> &str {
        match self {
            &Lang::en_US => "en-US",
            &Lang::de_DE => "de-DE",
            &Lang::es_ES => "es-ES",
            &Lang::es_MX => "es-MX",
            &Lang::fr_FR => "fr-FR",
            &Lang::it_IT => "it-IT",
            &Lang::ja_JP => "ja-JP",
            &Lang::pl_PL => "pl-PL",
            &Lang::pt_BR => "pt-BR",
            &Lang::ru_RU => "ru-RU",
        }
    }
}

pub struct CardPageRequest {
    pub lang: Lang,
    pub limit: u64,
    pub name: Option<String>,
    pub offset: u64,
    pub if_modified_since: Option<String>,
}

impl CardPageRequest {
    pub fn default() -> CardPageRequest {
        CardPageRequest {
            lang: Lang::en_US,
            limit: 20,
            name: None,
            offset: 0,
            if_modified_since: None
        }
    }

    pub fn to_hash(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        hash.insert(String::from("lang"), String::from(self.lang.as_str()));
        hash.insert(String::from("limit"), self.limit.to_string());
        if self.name.is_some() {
            let name = self.name.clone().unwrap();
            if name.len() >= 3 {
                hash.insert(String::from("name"), name);
            }
        }
        hash.insert(String::from("offset"), self.offset.to_string());
        hash
    }
}

pub struct CardRequest {
    cardID: String,
    lang: Lang,
}

impl CardRequest {
    pub fn default(cardID: &str) -> CardRequest {
        CardRequest { cardID: String::from(cardID), lang: Lang::en_US}
    }

    pub fn to_hash(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        hash.insert(String::from("lang"), String::from(self.lang.as_str()));
        hash.insert(String::from("cardID"), self.cardID.clone());
        hash
    }
}

pub struct QueryBuilder {}

impl QueryBuilder {
    pub fn query(uri: &str, args: HashMap<String, String>) -> Result<String, &'static str> {
        let mut query = String::from(uri);
        query.push_str("?");
        for (key, val) in &args {
            query.push_str(key);
            query.push_str("=");
            query.push_str(val);
            query.push_str("&");
        }
        Ok(query)
    }

    pub fn get_card(c_req: CardRequest) -> Result<Card, &'static str> {
        let query = QueryBuilder::query("https://api.gwentapi.com/v0/cards", c_req.to_hash())?;
        let mut resp = reqwest::get(query.as_str()).unwrap();

        match resp.json::<Card>() {
            Err(e) => Err("ERROR"),
            Ok(card) => Ok(card)
        }
    }
}