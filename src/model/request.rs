use std::collections::HashMap;

#[derive(Clone)]
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

    pub fn card_search_query(name: &str, lang: Option<Lang>) -> CardPageRequest {
        CardPageRequest {
            lang: lang.unwrap_or(Lang::en_US),
            limit: 1,
            name: Some(String::from(name)),
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

    pub fn query(&self) -> String {
        let mut query = String::from("https://api.gwentapi.com/v0/cards");
        query.push_str("?");
        let args = self.to_hash();
        for (key, val) in &args {
            query.push_str(key);
            query.push_str("=");
            query.push_str(val);
            query.push_str("&");
        }
        query
    }
}

pub struct CardRequest {
    card_id: String,
    lang: Option<Lang>,
}

impl CardRequest {
    pub fn default(card_id: &str) -> CardRequest {
        CardRequest {
            card_id: String::from(card_id),
            lang: Some(Lang::en_US),
        }
    }

    pub fn query(&self) -> String {
        let mut query = String::from("https://api.gwentapi.com/v0/cards/");
        query.push_str("?card_id=");
        query.push_str(self.card_id.as_str());
        query.push_str(self.lang.clone().unwrap_or(Lang::en_US).as_str());
        query
    }
}

pub struct VariationRequest {
    card_id: String,
    variation_id: String,
    lang: Option<Lang>,
}

impl VariationRequest {
    pub fn default(card_id: &str, variation_id: &str) -> VariationRequest {
        VariationRequest {
            card_id: String::from(card_id),
            variation_id: String::from(variation_id),
            lang: Some(Lang::en_US),
        }
    }

    pub fn query(&self) -> String {
        let mut query = String::from("https://api.gwentapi.com/v0/cards/");
        query.push_str(self.card_id.as_str());
        query.push_str("/variations/");
        query.push_str(self.variation_id.as_str());
        query.push_str(self.lang.clone().unwrap_or(Lang::en_US).as_str());
        query
    }
}