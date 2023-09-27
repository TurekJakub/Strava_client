// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use strava_client::request_builder::RequestBuilder;
use strava_client::strava_scraper::{Scraper, User};

static SCRAPER: Lazy<Scraper> = Lazy::new(|| Scraper::new());

#[tauri::command]
fn get_menu_data() -> HashMap<String, HashMap<String, (bool, HashSet<String>)>> {
    SCRAPER.login();
    print!("test");
    SCRAPER.scraper_user_menu()
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
        username: "breburdak",
        password: "breburdak",
        cantine: "5763",
    };
    let request_builder = RequestBuilder::new();
    request_builder.login(&user);
    let page = request_builder.get_user_menu(); // debug
    let mut menu = HashMap::new();
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
        let mut daily_menu = HashMap::new();
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
    // println!("{:?}", menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_menu_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
