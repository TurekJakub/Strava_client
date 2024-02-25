// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use indexmap::IndexMap;
use strava_client::data_struct::{Date, DishInfo, User, UserInfo};
use strava_client::strava_client::StravaClient;
use tokio::sync::Mutex;
use tokio::sync::OnceCell;

static CLIENT: Mutex<OnceCell<StravaClient>> = Mutex::const_new(OnceCell::const_new());
mod auto_orders_canceling;

#[tauri::command]
async fn get_menu_data() -> Result<(Vec<Date>, IndexMap<Date, IndexMap<String, DishInfo>>), String>
{
    let menu = CLIENT.lock().await.get().unwrap().get_menu().await?; // TODO: solve proble with unordered map
    Ok((menu.keys().cloned().collect(), menu))
}
#[tauri::command]
async fn login(
    username: String,
    password: String,
    cantine: String,
    stay_logged: bool,
) -> Result<UserInfo, String> {
    let u = User {
        username: &username,
        password: &password,
        cantine: &cantine,
        lang: "EN",
        stay_logged: false,
    };
    CLIENT
        .lock()
        .await
        .get_or_init(|| async { StravaClient::new().await.unwrap() })
        .await
        .login(&u)
        .await
}
#[tauri::command]
async fn order_dish(dish_id: String, ordered: bool) -> Result<f64, String> {
    CLIENT
        .lock()
        .await
        .get_mut()
        .unwrap()
        .order_dish(dish_id, ordered)
        .await
}
#[tauri::command]
async fn save_orders() -> Result<(), (String, f64)> {
    CLIENT.lock().await.get_mut().unwrap().save_orders().await
}
#[tokio::main]
async fn main() {
        tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        get_menu_data,
        login,
        order_dish,
        save_orders
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
