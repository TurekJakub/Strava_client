use std::collections::HashMap;

use crate::strava_scraper::User;
use indexmap::IndexMap;
use once_cell::sync::{Lazy, OnceCell};
use reqwest::{blocking::Client, blocking::Response, Error};
use scraper::Html;
use serde_json::Value;
pub struct RequestBuilder {
    client: Client,
    canteen_id: OnceCell<String>,
    sid: OnceCell<String>,
    url: OnceCell<String>,
}
impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            client: Client::builder().cookie_store(true).build().unwrap(),
            sid: OnceCell::new(),
            canteen_id: OnceCell::new(),
            url: OnceCell::new(),
        }
    }
    // authenticate user and retun errors if occured
    pub fn login(&self, user: &User) -> Result<(), Error> {
        self.do_get("https://app.strava.cz/prihlasit-se?jidelna");
        match self.do_post(
            "https://app.strava.cz/api/login",
            serde_json::to_string(&user).unwrap(),
        ) {
            Ok(res) => match res.error_for_status() {
                Ok(res) => {
                    let res_json =
                        serde_json::from_str::<serde_json::Value>(&res.text().unwrap()).unwrap();
                    self.sid
                        .set(res_json.get("sid").unwrap().as_str().unwrap().to_string())
                        .unwrap();
                    self.url
                        .set(res_json.get("s5url").unwrap().as_str().unwrap().to_string())
                        .unwrap();
                    self.canteen_id.set(user.cantine.to_string()).unwrap();
                    Ok(())
                }
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
    // do get request for given cantine menu page and return it
    pub fn get_cantine_menu(&self, cantinecode: &str) -> Html {
        self.do_get(
            ("https://www.strava.cz/Strava/Stravnik/Jidelnicky?zarizeni=".to_owned() + cantinecode)
                .as_str(),
        )
    }
    // do get request for loqged users menu page and return it
    pub fn get_user_menu(&self) -> Value {
        let request_args = format!(
            r#""sid":"{}","s5url":"{}","cislo":"{}","konto":"0","podminka":"","resetTables":"true""#,
            self.sid.get().unwrap(),
            self.url.get().unwrap(),
            self.canteen_id.get().unwrap()
        );
       let res = self.do_order_post("https://app.strava.cz/api/objednavky", request_args);
       serde_json::from_str::<serde_json::Value>(&res.unwrap().text().unwrap()).unwrap()

    }
    pub fn do_get(&self, url: &str) -> Html {
        let res = self.client.get(url).send();
        Html::parse_document(res.unwrap().text().unwrap().as_str())
    }
    pub fn do_post(&self, url: &str, body: String) -> Result<Response, Error> {
        self.client.post(url).body(body).send()
    }
    pub fn do_order_post(&self, url: &str, body_args: String) -> Result<Response, Error> {
        let body = format!(r#"{{"lang":"EN","ignoreCert":"false",{}}}"#, body_args);
        println!("{}", body);
        self.client.post(url).body(body).send()
    }
}
