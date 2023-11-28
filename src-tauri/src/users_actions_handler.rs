use crate::request_builder::RequestBuilder;
use crate::strava_scraper::Scraper;
use indexmap::IndexMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Settings {
    data_source: String,
}
struct ApiClient {
    request_builder: RequestBuilder,
    menu: OnceCell<IndexMap<String, IndexMap<String, (bool, String, Vec<String>)>>>,
    screaper: OnceCell<Scraper>,
    settings: Settings,
}
impl ApiClient {
    pub fn new() -> Result<ApiClient, String> {
        Ok(ApiClient {
            request_builder: RequestBuilder::new(),
            menu: OnceCell::new(),
            screaper: OnceCell::new(),
            settings: match ApiClient::load_settings() {
                Ok(settings) => settings,
                Err(e) => return Err(e),
            },
        })
    }
    pub fn load_settings() -> Result<Settings, String> {
        match toml::from_str(fs::read_to_string("../settings.toml").unwrap().as_str()) {
            Ok(settings) => Ok(settings),
            Err(e) => Err("Chyba při načítání nastavení ze souboru ../settings.toml".to_string()),
        }
    }
    pub async fn get_menu(
        &self,
    ) -> Result<IndexMap<String, IndexMap<String, (bool, String, Vec<String>)>>, String> {
        match self.menu.get() {
            Some(menu) => Ok(menu.clone()),
            None => {
                let menu = match self.settings.data_source.as_str() {
                    "api" =>  self.request_builder.get_user_menu()?,
                    "scraper" => self.screaper.get().unwrap().scraper_user_menu(&self.request_builder).await?,
                    _ => return Err("Chybná konfigurace způsobu získání dat v souboru config.toml".to_string()),
                };
                self.menu.set(menu.clone()).unwrap();
                Ok(menu)
            }
        }
    }
}
