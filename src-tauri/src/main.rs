// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use cached::{Cached, UnboundCache};
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use strava_client::request_builder::RequestBuilder;
use strava_client::strava_scraper::{Date, Scraper, User};

static SCRAPER: Lazy<Scraper> = Lazy::new(|| Scraper::new());
static mut CACHE: Lazy<
    UnboundCache<String, IndexMap<String, IndexMap<String, (bool, HashSet<String>)>>>,
> = Lazy::new(|| UnboundCache::new());
#[tauri::command]
fn get_menu_data() -> IndexMap<String, IndexMap<String, (bool, HashSet<String>)>> {
    SCRAPER.login();
    print!("test");
    let menu = SCRAPER.scraper_user_menu();
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
fn main() {
    let user = User {
        username: "turekj",
        password: "68AspiK20",
        cantine: "5763",
    };
    let request_builder = RequestBuilder::new();
    let xx = request_builder.test_do_get("https://www.strava.cz/strava/");

    let z = request_builder.login(&user);
    let page = request_builder.get_user_menu(); // debug
    let mut menu = IndexMap::new();
    let days_selector = Selector::parse(".objednavka").unwrap();
    let date_selector = Selector::parse(".den").unwrap();
    let dishes_name_selector = Selector::parse(".nazev").unwrap();
    let allergens_selector = Selector::parse(".alergeny").unwrap();
    let order_state_selector = Selector::parse("input[autocomplete='off']").unwrap();

    let mut days = page.select(&days_selector);
    // println!("{:?}", days);

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
    let x = request_builder.do_post(
        "
    https://www.strava.cz/Strava5/Objednavky/Prihlas",
        &map,
    );
    let y = request_builder.do_post("https://www.strava.cz/Strava5/Objednavky/Odesli", &map);
    //x.unwrap().cookies().for_each(|x| println!("{:?}", x));
    println!("{:?}", x.unwrap());
    println!();
    println!("{:?}", y.unwrap());
    //xx.unwrap().cookies().for_each(|x| println!("{:?}", x));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_menu_data, sort_menus_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
