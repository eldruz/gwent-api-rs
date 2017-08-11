extern crate reqwest;

use self::reqwest::{Response};

static API_URI: &str = "https://api.gwentapi.com/v0";

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        let client = reqwest::Client::new().expect("Couldn't create client");
        Client { client }
    }
    pub fn get(&self, uri: &str) -> reqwest::Result<Response> {
        self.client.get(uri)?
            .send()
    }
}