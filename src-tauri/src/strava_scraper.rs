use crate::request_builder::RequestBuilder;
use fantoccini::{error::CmdError, Client, ClientBuilder, Locator};
use indexmap::IndexMap;
use scraper::{Html, Selector};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::time::Duration;
use std::{
    collections::HashSet,
    process::{Child, Command},
};
use url::Url;
fn main() {
    let scraper = Scraper::new();
    scraper.login();
    let menu = scraper.scraper_user_menu();
    println!("{:?}", menu);
}
/*
fn main() {
    let request = RequestBuilder::new();
    let user = User {
        username: "test",
        password: "test123",
        cantine: "0000",
    };
    request.login(&user);
    let page = request.test();
    scraper_user_menu(page);
}
*/
// structure representing user
pub struct User<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub cantine: &'a str,
}
// structure representing dish
pub struct Dish<'a> {
    pub name: &'a str,
    pub allergens: Vec<&'a str>,
}
#[derive(Eq, Debug, Hash, Clone)]
pub struct Date {
    pub day: i8,
    pub month: i8,
    pub day_of_week: String,
}
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month
    }
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.month > other.month {
            return Some(std::cmp::Ordering::Greater);
        }
        if self.month < other.month {
            return Some(std::cmp::Ordering::Less);
        }
        Some(self.day.cmp(&other.day))
    }
}
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Date {
    pub fn to_string(&self) -> String {
        format!("{} {}. {}.", self.day_of_week, self.day, self.month)
    }
}

pub struct Scraper {
    client: Client,
    gecko: Child,
    firefox: Child,
}
impl Scraper {
    pub async fn new() -> Scraper {
        Scraper {
            firefox: Command::new("firefox")
                .env("PATH", "./bin/firefox")
                .args(["--marionette", "--headless"])
                .spawn()
                .expect("failed to execute process"),
            gecko: Command::new("geckodriver")
                .env("PATH", "./bin")
                .args(["--marionette-port", "2828", "--connect-existing"])
                .spawn()
                .expect("UwU"),
            client: ClientBuilder::native()
                .connect("http://localhost:4444")
                .await
                .expect("failed to connect to WebDriver"),
        }
    }
    pub async fn login(&self, user: &User<'_>) {
        let cookie_button = self
            .client
            .wait()
            .at_most(Duration::from_millis(1000))
            .for_element(Locator::Css(
                r#"button[id*="CybotCookiebotDialogBodyButtonDecline"]"#,
            ))
            .await;
        match cookie_button {
            Ok(x) => x.click().await.unwrap(),
            Err(_) => (),
        };
        self.client
            .find(Locator::Css(r#"input[placeholder*="Heslo"]"#))
            .await
            .unwrap()
            .send_keys(user.password)
            .await
            .unwrap();
        self.client
            .find(Locator::Css(r#"input[placeholder*="Uživatel"]"#))
            .await
            .unwrap()
            .send_keys(user.username)
            .await
            .unwrap();
        self.client
            .find(Locator::Css(r#"input[placeholder*="Číslo"]"#))
            .await
            .unwrap()
            .send_keys(user.cantine)
            .await
            .unwrap();

        self.client
            .find(Locator::Css(r#"button[type="submit"]"#))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();
    }
    // parse given html to menu represented by following structure HashMap<date: String, HashMap<dish_name: String, (is_ordered: bool, allergens: HashSet<String>)>>
    pub fn scraper_user_menu(&self) -> IndexMap<String, IndexMap<String, (bool, HashSet<String>)>> {
        let page = self.request_builder.get_user_menu(); // debug
        let mut menu = IndexMap::new();
        let days_selector = Selector::parse(".objednavka").unwrap();
        let date_selector = Selector::parse(".den").unwrap();
        let dishes_name_selector = Selector::parse(".nazev").unwrap();
        let allergens_selector = Selector::parse(".alergeny").unwrap();
        let order_state_selector = Selector::parse("input[autocomplete='off']").unwrap();

        let days = page.select(&days_selector);
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
                let allergens = self.get_allergens(dish_description.inner_html());
                // print!(" Allergens: {:?} ", allergens);
                daily_menu.insert(
                    dish.inner_html(),
                    (
                        orders_state.next().unwrap().value().attr("value").unwrap()
                            != "nezaskrtnuto",
                        allergens,
                    ),
                );
            }
            menu.insert(date.clone(), daily_menu);
        }
        /*
        let mut menu2 = HashMap::new();
        let mut me = HashMap::new();
        let mut x = HashSet::new();
        x.insert("test".to_string());
        me.insert("test".to_string(), (true, x));
        menu2.insert("test".to_string(), me);
        menu2*/
        menu
    }
    // extract and return list of allergens from given dish description
    pub fn get_allergens(&self, dish_descriptin: String) -> HashSet<String> {
        let mut allergens = HashSet::new();
        // print!("{}", x);
        for c in dish_descriptin.chars().filter(|c| c.is_digit(10)) {
            if c != '0' {
                allergens.insert(c.to_string());
            }
        }
        allergens
    }
}
