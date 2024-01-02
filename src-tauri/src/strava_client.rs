use crate::request_builder::RequestBuilder;
use crate::strava_scraper::Scraper;
use crate::data_struct::{User, DishInfo, Date};
use indexmap::IndexMap;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs};

#[derive(Deserialize, Serialize)]
pub struct Config {
    settings: HashMap<String, String>,
}
pub struct StravaClient {
    request_builder: RequestBuilder,
    menu: OnceCell<IndexMap<Date, IndexMap<String, DishInfo>>>,
    screaper: tokio::sync::OnceCell<Scraper>,
    settings: Config,
}
impl StravaClient {
    pub async fn new() -> Result<StravaClient, String> {
        Ok(StravaClient {
            request_builder: RequestBuilder::new(),
            menu: OnceCell::new(),
            screaper: tokio::sync::OnceCell::new(),
            settings: match StravaClient::load_settings() {
                Ok(settings) => settings,
                Err(e) => return Err(e),
            },
        })
    }
    pub async fn new_with_settings(settings: Config) -> Result<StravaClient, String> {
        Ok(StravaClient {
            request_builder: RequestBuilder::new(),
            menu: OnceCell::new(),
            screaper: tokio::sync::OnceCell::new(),
            settings: settings,
        })
    }
    fn load_settings() -> Result<Config, String> {
        match toml::from_str(
            fs::read_to_string(env::current_dir().unwrap().as_path().join("../config.toml"))
                .unwrap()
                .as_str(),
        ) {
            Ok(settings) => Ok(settings),
            Err(_) => Err("Chyba při načítání nastavení ze souboru ../settings.toml".to_string()),
        }
    }
    pub async fn get_menu(
        &self,
    ) -> Result<IndexMap<Date, IndexMap<String, DishInfo>>, String> {
        match self.menu.get() {
            Some(menu) => Ok(menu.clone()),
            None => {
                let menu = match self.settings.settings.get("data_source").unwrap().as_str() {
                    "api" => self.request_builder.do_get_user_menu_request().await?,
                    "scraper" => {
                        self.screaper
                            .get()
                            .unwrap()
                            .scraper_user_menu(&self.request_builder)
                            .await?
                    }
                    _ => {
                        return Err(
                            "Chybná konfigurace způsobu získání dat v souboru config.toml"
                                .to_string(),
                        )
                    }
                };
                self.menu.set(menu.clone()).unwrap();
                Ok(menu)
            }
        }
    }
    pub async fn login(&self, user: &User<'_>) -> Result<(), String> {
        match self.settings.settings.get("data_source").unwrap().as_str() {
            "api" => (),
            "scraper" => {self.screaper.get_or_init(||Scraper::new()).await.login(&user).await?},
            _ => {
                return Err(
                    "Chybná konfigurace způsobu získání dat v souboru config.toml".to_string(),
                )
            }
        };
        self.request_builder.do_login_request(&user).await?;
        Ok(())
    }
    pub async fn order_dish(&self, dish_id: String, ordered: bool) -> Result<(), String> {
        let amount = if ordered {1} else {0};
        self.request_builder.do_order_dish_request(dish_id, amount).await?;
        Ok(())
    }
    pub async fn save_orders(&self) -> Result<(), String> {
        self.request_builder.do_save_orders_request().await?;
        Ok(())
    }
}
