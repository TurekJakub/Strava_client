// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use strava_client::data_struct::{Date, DishInfo, User};
use dotenv::dotenv;
use indexmap::IndexMap;
use strava_client::strava_client::StravaClient;
use tokio::sync::OnceCell;


static CLIENT: OnceCell<StravaClient> = OnceCell::const_new();
/*
static mut CACHE: OnceCell<
    IndexMap<String, IndexMap<String, IndexMap<String, (bool, String, Vec<String>)>>>,
> = OnceCell::new();
*/
#[tauri::command]
async fn get_menu_data() -> IndexMap<Date, IndexMap<String, DishInfo>> {
    dotenv().ok();
    let username = std::env::var("STRAVA_USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    let cantine = std::env::var("CANTINE").unwrap();
    let u = User {
        username: &username,
        password: &password,
        cantine: &cantine,
        lang: "CZ",
        stay_logged: false,
    };
    CLIENT
        .get_or_init(|| async { StravaClient::new().await.unwrap() })
        .await
        .login(&u)
        .await
        .unwrap();
    CLIENT.get().unwrap().get_menu().await.unwrap()
}
#[tauri::command]
async fn login(username: String, password: String, cantine: String, stay_logged: bool) -> bool {
    let u = User {
        username: &username,
        password: &password,
        cantine: &cantine,
        lang: "CZ",
        stay_logged: stay_logged,
    };
    CLIENT
        .get_or_init(|| async { StravaClient::new().await.unwrap() })
        .await
        .login(&u)
        .await
        .unwrap();
    return true;
}

#[tokio::main]
async fn main() {
  
    /*
    let menu = get_menu_data().await;
    menu.keys()
        .for_each(|x| println!("{:?}, {:?}", x, menu.get(x).unwrap().keys()));
    */
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_menu_data,login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
