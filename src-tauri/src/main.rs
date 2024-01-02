// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use dotenv::dotenv; // debug only
use indexmap::IndexMap;
use strava_client::data_struct::{Date, DishInfo, User};
use strava_client::strava_client::StravaClient;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<StravaClient> = OnceCell::const_new();
/*
static mut CACHE: OnceCell<
    IndexMap<String, IndexMap<String, IndexMap<String, (bool, String, Vec<String>)>>>,
> = OnceCell::new();
*/
#[tauri::command]
async fn get_menu_data() -> Result<(Vec<Date>, IndexMap<Date, IndexMap<String, DishInfo>>), String>
{
    /* debug only
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
    */
    let menu = CLIENT.get().unwrap().get_menu().await?;
    Ok((menu.keys().cloned().collect(), menu))
}
#[tauri::command]
async fn login(
    username: String,
    password: String,
    cantine: String,
    stay_logged: bool,
) -> Result<(), String> {
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
        .await?;
    Ok(())
}
#[tauri::command]
async fn order_dish(dish_id: String, ordered: bool) -> Result<(), String> {
    CLIENT.get().unwrap().order_dish(dish_id, ordered).await?;
    Ok(())
}
#[tauri::command]
async fn save_orders() -> Result<(), String> {
    CLIENT.get().unwrap().save_orders().await?;
    Ok(())
}
#[tokio::main]
async fn main() {
    /*
    let menu = get_menu_data().await;
    menu.keys()
        .for_each(|x| println!("{:?}, {:?}", x, menu.get(x).unwrap().keys()));

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_menu_data, login, order_dish, save_orders])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    */
    keytar::set_password("strava_client", "username", "password").unwrap();
    keytar::set_password("strava_client", "username1", "password1").unwrap();
    let x = keytar::find_password("strava_client").unwrap();
    println!("{}", x.password);
    println!(
        "{}",
        keytar::get_password("strava_client", "username")
            .unwrap()
            .password
    )
}
