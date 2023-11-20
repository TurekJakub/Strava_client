// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use cached::{Cached, UnboundCache};
use chrono::Datelike;
use fantoccini::{error::CmdError, ClientBuilder, Locator, key};
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use reqwest::cookie::Cookie;
use scraper::{Html, Selector};
use tauri::utils::consume_unused_variable;
use std::{
    collections::{HashMap, HashSet},
    process::Command,
};
use strava_client::request_builder::{RequestBuilder, self};
use strava_client::strava_scraper::{Date, Scraper, User};
use url::Url;
use tokio::sync::OnceCell;

static SCRAPER: OnceCell<Scraper> = OnceCell::const_new();


static mut CACHE: Lazy<
    UnboundCache<String, IndexMap<String, IndexMap<String, (bool, Vec<String>)>>>,
> = Lazy::new(|| UnboundCache::new());
#[tauri::command]
async fn get_menu_data() -> IndexMap<String, IndexMap<String, (bool, Vec<String>)>> {
    let u = User {
        username: "",
        password: "",
        cantine: "5763",
        lang: "CZ",
        stay_logged: false,
    };
    SCRAPER.get_or_init(||Scraper::new()).await.login(&u).await;
    print!("  test");
    let menu = SCRAPER.get_or_init(||Scraper::new()).await.scraper_user_menu().await;
    unsafe {
        CACHE.cache_set("menu".to_owned(), menu.clone());
    }
    menu
}
#[tauri::command]
fn sort_menus_keys(keys: Vec<&str>) -> Vec<String> {
    //let mut keys_as_date: Vec<Date>  = keys.iter().map(|x| x.replace(".", "").split(" ").map(|s| s.to_owned()).collect()).map(|y: Vec<_>| Date { day:y[1].parse().unwrap(), month:y[2].parse().unwrap(), day_of_week:y[0].to_string() }).collect();
    //keys_as_date.sort();
    //keys_as_date.iter().map(|x| x.to_string()).collect()
    unsafe {
        CACHE
            .cache_get("menu")
            .unwrap()
            .keys()
            .map(|x| x.to_string())
            .collect()
    }
}
pub fn get_allergens(dish_descriptin: String) -> HashSet<String> {
    let mut allergens = HashSet::new();
    // print!("{}", x);
    for c in dish_descriptin.chars().filter(|c| c.is_digit(10)) {
        if c != '0' {
            allergens.insert(c.to_string());
        }
    }
    allergens
}
//#[tokio::main]
 fn main() {
    let date = chrono::Local::now();
    
    //let s = Scraper::new().await;
    let user = User {
        username: "turekj",
        password: "",
        cantine: "",
        lang: "CZ",
        stay_logged: false,
    };
    println!("{}", serde_json::to_string(&user).unwrap());
    let b = RequestBuilder::new();
    let x = b.login(&user).unwrap();
    let mut z = b.get_user_menu();
    let y = z.as_object_mut().unwrap();
   /*
    for key in  y.keys() {
        println!("{}", y.get(key).unwrap().get("datum").unwrap());
    }
    */
    
   // let y = x.text().unwrap();
    //let z = serde_json::from_str::<serde_json::Value>(&y).unwrap();
    println!("{:?}", z);
  
    //s.login(&user).await;
    //s.scraper_user_menu().await;
    /*
    let form = c.form(Locator::Css(r#"form"#)).await?;
    let w =x.set(Locator::Css(r#"input[placeholder*="Heslo"]"#), "password")
        .await?
        .set(Locator::Css(r#"input[placeholder*="Uživatel"]"#), "user")
        .await?
        .set(Locator::Css(r#"input[placeholder*="Číslo"]"#), "5763")
        .await?
        .submit_with(Locator::Css(r#"button[type="submit"]"#))
        .await?;
    */
    // c.wait().for_element(Locator::Css(r#"div[id*="Day""#)).await?;

    //println!("{}",z);

    //gecko.kill().expect("!kill");
    //firefox.kill().expect("!kill");

    /* login using api
    let user = User {
        username: "user",
        password: "password",
        cantine: "5763",
    };
    let request_builder = RequestBuilder::new();
    let xx = request_builder.test_do_get("https://app.strava.cz/prihlasit-se?jidelna");

    let t = request_builder.login(&user);
    let mut userr: IndexMap<&str, &str> = IndexMap::new();

    userr.insert("cislo", "5763");
    userr.insert("heslo", "password");
    userr.insert("jmeno", "user");
    userr.insert("lang", "CZ");
    userr.insert("zustatPrihlasen", "false");
    println!("{}", serde_json::to_string(&userr).unwrap());
    let y = request_builder.do_post(
        "https://app.strava.cz/api/login
    ",
        &userr,
    );

    let page = Html::parse_document(&z); // debug
                                                // println!("{:?}",page.html());
                                                // let mut menu = IndexMap::new();
    let days_selector = Selector::parse(r#"div[id*='2023']"#).unwrap();
    let date_selector = Selector::parse("h2").unwrap();
    let dishes_name_selector = Selector::parse("span >span").unwrap();
    let allergens_selector = Selector::parse("button > span").unwrap();
    let order_state_selector = Selector::parse("input[autocomplete='off']").unwrap();

    let mut days = page.select(&days_selector);
    //let c = days.next();
    //println!("{}",c.unwrap().html());

    //days.for_each(|x| println!("{:?}", x));
    */
    /*
       for day in days {
           let daily_menu_html = Html::parse_fragment(day.html().as_str());
           let dishes_of_day = daily_menu_html.select(&dishes_name_selector);
           let mut dishes_allergens = daily_menu_html.select(&allergens_selector);
           let mut daily_menu = IndexMap::new();
           let mut orders_state = daily_menu_html.select(&order_state_selector);
           let date = daily_menu_html
               .select(&date_selector)
               .next()
               .unwrap()
               .value()
               .attr("title")
               .unwrap()
               .to_string();
           for dish in dishes_of_day {
               let allergens_element = dishes_allergens.next();
               let dish_description = match allergens_element {
                   Some(x) => x,
                   _ => continue,
               };
               let allergens = get_allergens(dish_description.inner_html());
               // print!(" Allergens: {:?} ", allergens);
               daily_menu.insert(
                   dish.inner_html(),
                   (
                       orders_state.next().unwrap().value().attr("value").unwrap() != "nezaskrtnuto",
                       allergens,
                   ),
               );
           }
           menu.insert(date, daily_menu);
       }
       //let z =request_builder.login(&user);

       // let param = [("pocet", "1"), ("veta", "3")];
       let mut map: HashMap<&str, &str> = HashMap::new();
       map.insert("veta", "3");
       map.insert("pocet", "1");
      // let x = request_builder.do_post( " https://www.strava.cz/Strava5/Objednavky/Prihlas", &map,    );
      // let y = request_builder.do_post("https://www.strava.cz/Strava5/Objednavky/Odesli", &map);
       //x.unwrap().cookies().for_each(|x| println!("{:?}", x));
       // println!("{:?}", x.unwrap());
       //   println!();
       //   println!("{:?}", y.unwrap());
       //xx.unwrap().cookies().for_each(|x| println!("{:?}", x));
    */
    // let u = c.close().await;
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_menu_data, sort_menus_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    
}
