use crate::db_client::DbClient;

use reqwest::Client;
use std::error::Error;
use strava_client::data_struct::{
    Cantine, CantineDBEntry, CantineData, DishDBEntry,
};
pub struct Crawler {
    client: Client,
    db_client: DbClient,
}
impl Crawler {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: Client::builder().cookie_store(true).build()?,
            db_client: DbClient::new().await?,
        })
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
            Ok(res) => match res.error_for_status() {
                Ok(res) =>{
                    match res.text().await {
                        Ok(res_text) => res_text,
                        Err(_) => return Err("Failed to get cantine menu".to_string()),
                    }
                }
                Err(_) => {
                    return Err("Failed to get cantine menu".to_string())}
            }
           

            Err(_) => return Err("Failed to get cantine menu".to_string()),
        };
        let cantine_menu: serde_json::Value = match serde_json::from_str(&res_text) {
            Ok(cantine_menu) => cantine_menu,
            Err(_) => return Err("Failed to parse cantine menu".to_string()),
        };

        Ok(parse_cantine_menu(cantine_menu))
    }
    pub async fn update_cantines_history(&self) -> Result<(), Box<dyn Error>> {
        let cantines = self.get_cantines().await?;
        for cantine in cantines {
            println!("Updating cantine {}", cantine.name);
            let cantine_dishes = match self.get_cantine_menu(&cantine.id).await {
                Ok(cantine_dishes) => cantine_dishes,
                Err(_) => continue,
            };                 
            let cantine_history = match  self.db_client.insert_dishes(cantine_dishes).await {
                Ok(cantine_history) => cantine_history,
                Err(_) => continue,
            };
            println!("Items added {}", cantine_history.len());
            self.db_client
                .insert_cantine(CantineDBEntry {
                    cantine_id: cantine.id,
                    name: cantine.name,
                    cantine_history: cantine_history,
                })
                .await?;
        }
        Ok(())
    }
}
fn parse_cantine_menu(cantine_menu: serde_json::Value) -> Vec<DishDBEntry> {
    let mut menu = Vec::new();
    for (_day, dishes) in cantine_menu.as_object().unwrap() {
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
