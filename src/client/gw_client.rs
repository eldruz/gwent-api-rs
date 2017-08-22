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

/// Structure used to construct an url to query the gwent api.
///
/// Functions on the QueryBuilder are chainable, provided it has been
/// initialized first.
///
/// Links to the [gwent api](https://gwentapi.com/) and its [documentation](https://gwentapi.com/swagger/index.html).
pub struct QueryBuilder(pub HashMap<String, String>);

impl QueryBuilder {
    /// Constructs a new QueryBuilder. It is necessary to do so before chaining other operations.
    pub fn new() -> QueryBuilder {
        QueryBuilder(HashMap::new())
    }

    /// Returns an Url with all parameters set.
    ///
    /// # Arguments
    ///
    /// * `base_url` - the url targetted by the request
    ///
    /// # Examples
    ///
    /// ```
    /// use reqwest::Url;
    ///
    /// let url = QueryBuilder::new()
    ///     .card_name("Alchemist")
    ///     .as_url("https://api.gwentapi.com/v0/cards/");
    /// ```
    pub fn as_url(&self, base_url: &str) -> Result<Url, reqwest::UrlError> {
        Url::parse_with_params(base_url, &self.0)
    }

    /// Sets the language of the returned response.
    ///
    /// # Arguments
    ///
    /// * `language` - string ∈ {en-US, de-DE, es-ES, es-MX, fr-FR, it-IT, ja-JP, pl-PL, pt-BR, ru-RU}
    ///
    /// # Examples
    ///
    /// ```
    /// use reqwest::Url;
    ///
    /// let url = QueryBuilder::new()
    ///     .lang("fr-FR")
    ///     .as_url("https://api.gwentapi.com/v0/cards/bLkeCcb0VeaNNS2J73VLVA");
    /// ``` 
    pub fn lang<'a>(&'a mut self, language: &str) -> &'a mut Self {
        self.0.insert(String::from("lang"), String::from(language));
        self
    }

    /// Sets the name of the searched card.
    ///
    /// # Arguments
    ///
    /// * `card_name` - name of the card
    ///
    /// # Examples
    ///
    /// ```
    /// let url = QueryBuilder::new()
    ///     .card_name("Geralt: Igni")
    ///     .as_url("https://api.gwentapi.com/v0/cards/");
    /// ```
    ///
    /// # Errors
    ///
    /// Not available on certain API requests.
    ///
    /// Will fail with:
    ///
    /// * GET /v0/cards
    pub fn card_name<'a>(&'a mut self, card_name: &str) -> &'a mut Self {
        self.0.insert(String::from("name"), String::from(card_name));
        self
    }

    /// Generic parameter setting.
    ///
    /// # Arguments
    ///
    /// * `param` - name of the request parameter
    /// * `value` - value of the request parameter
    pub fn with_arg<'a>(&'a mut self, param: &str, value: &str) -> &'a mut Self {
        self.0.insert(String::from(param), String::from(value));
        self
    }
}

/// Gateway offering specific functions through the gwent api.
pub struct Client { }

impl Client {
    fn get_query_card_by_name(card_name: &str) -> Result<Url, reqwest::UrlError> {
        QueryBuilder::new()
            .card_name(card_name)
            .as_url("https://api.gwentapi.com/v0/cards/")
    }

    /// Retrieves a Card object based on its name.
    ///
    /// # Arguments
    ///
    /// * `card_name` - the name of the desired card
    ///
    /// # Errors
    ///
    /// Will fail if the provided string doesn't correspons to any card in the API.
    /// All pattern matching and partial recognition is handled by the server.
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

    /// Retrieves the default Variation object of a given card.
    ///
    /// This object contains a link to the art representing the card in the game.
    ///
    /// # Arguments
    ///
    /// * `card`: a reference to the Card
    pub fn get_card_default_art(card: &Card) -> reqwest::Result<Variation> {
        let search_query = card.variations[0].href.clone();
        get_card!(search_query.as_str(), Variation)
    }
}