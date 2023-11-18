use std::collections::HashMap;

use crate::strava_scraper::User;
use indexmap::IndexMap;
use reqwest::{blocking::Client, blocking::Response, Error};
use scraper::Html;
pub struct RequestBuilder {
    client: Client,
}
impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            client: Client::builder().cookie_store(true).build().unwrap(),
        }
    }
    // authenticate user and return response content as string
    pub fn login(&self, user: &User) -> Result<Response, Error> {       
        self.do_get("https://app.strava.cz/prihlasit-se?jidelna");
        self.do_post("https://app.strava.cz/api/login", serde_json::to_string(&user).unwrap())
    }    
    // do get request for given cantine menu page and return it
    pub fn get_cantine_menu(&self, cantinecode: &str) -> Html {
        self.do_get(
            ("https://www.strava.cz/Strava/Stravnik/Jidelnicky?zarizeni=".to_owned() + cantinecode)
                .as_str(),
        )
    }
    // do get request for loqged users menu page and return it
    pub fn get_user_menu(&self) -> Html {
        self.do_get("https://app.strava.cz/")
    }
    pub fn do_get(&self, url: &str) -> Html {
        let res = self.client.get(url).send();
        Html::parse_document(res.unwrap().text().unwrap().as_str())
    }
    pub fn test_do_get(&self, url: &str) -> Result<Response, Error> {
        let res = self.client.get(url).send();
        res
    }
    pub fn do_post(&self, url: &str, body: String) -> Result<Response, Error> {
        self.client.post(url).body(body).send()
    }
    pub fn do_post2(&self, url: &str) -> Result<Response, Error> {
        let res = self.client.post(url).send();
        res
    }
    pub fn test(&self) -> Html {
        self.do_get("https://www.strava.cz/Strava/Stravnik/Objednavky")
    }
}
