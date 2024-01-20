use indexmap::IndexMap;
use reqwest::{Client, Error};
use strava_client::data_struct::{
    Cantine, CantineDBEntry, CantineData, Date, DishDBEntry, DishInfo, OrdersCancelingSettings,
    UserDBEntry,
};
pub struct Crawler {
    client: reqwest::Client,
}
impl Crawler {
    pub fn new() -> Self {
        Self {
            client: Client::builder().cookie_store(true).build().unwrap(),
        }
    }
    pub async fn get_cantines(&self) -> Result<Vec<Cantine>, String> {
        let res_text = match self
            .client
            .post("https:/app.strava.cz/api/zarAMesta")
            .body(r#"{"lang":"CZ"}"#)
            .header("Content-Length", 13)
            .send()
            .await
        {
            Ok(res) => match res.text().await {
                Ok(res_text) => res_text,
                Err(_) => return Err("Failed to get cantines data".to_string()),
            },

            Err(_) => return Err("Failed to get cantines data".to_string()),
        };
        let cantines_data: Vec<CantineData> = match serde_json::from_str(&res_text) {
            Ok(cantines_data) => cantines_data,
            Err(_) => return Err("Failed to parse cantines data".to_string()),
        };
        let mut cantines: Vec<Cantine> = Vec::new();
        for cantine in cantines_data {
            cantines.push(Cantine {
                id: cantine.zarizeni.get(0).unwrap().clone(),
                name: format!(
                    "{}, {}, {}",
                    cantine.v_nazev.get(0).unwrap().clone(),
                    cantine.v_mesto.get(0).unwrap().clone(),
                    cantine.v_ulice.get(0).unwrap().clone()
                ),
            })
        }
        Ok(cantines)
    }
    pub async fn get_cantine_menu(&self, cantine_id: &str) -> Result<Vec<DishDBEntry>, String> {
        let res_text = match self
            .client
            .post("https://app.strava.cz/api/jidelnicky")
            .body(format!(r#"{{"cislo": "{}", "s5url": "https://wss52.strava.cz/WSStravne5_7/WSStravne5.svc","lang":"CZ","ignoreCert":false }}"#, cantine_id))
            .send()
            .await
        {
            Ok(res) => match res.text().await {
                Ok(res_text) => res_text,
                Err(_) => return Err("Failed to get cantine menu".to_string()),
            },

            Err(_) => return Err("Failed to get cantine menu".to_string()),
        };
        println!("{}", res_text);
        let cantine_menu: serde_json::Value = match serde_json::from_str(&res_text) {
            Ok(cantine_menu) => cantine_menu,
            Err(_) => return Err("Failed to parse cantine menu".to_string()),
        };

        Ok(parse_cantine_menu(cantine_menu))
    }
}
fn parse_cantine_menu(cantine_menu: serde_json::Value) -> Vec<DishDBEntry> {
    let mut menu = Vec::new();
    for (day, dishes) in cantine_menu.as_object().unwrap() {
        for dish in dishes.as_array().unwrap() {
            let mut allergens = Vec::new();
            for allergen in dish.get("alergeny").unwrap().as_array().unwrap() {
                allergens.push(
                    allergen
                        .as_array()
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                );
            }
            menu.push(DishDBEntry {
                name: dish.get("nazev").unwrap().as_str().unwrap().to_string(),
                allergens: allergens,
            })
        }
    }
    menu
}
