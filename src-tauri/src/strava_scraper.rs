use crate::request_builder::RequestBuilder;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};

fn main(){
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
pub struct Scraper {
    request_builder: RequestBuilder,
}
impl Scraper {
    pub fn new() -> Scraper {
        Scraper {
            request_builder: RequestBuilder::new(),
        }
    }
    pub fn login(&self) {
        let user = User {
            username: "breburdak",
            password: "breburdak",
            cantine: "5763",
        };
        self.request_builder.login(&user);
    }
    // parse given html to menu represented by following structure HashMap<date: String, HashMap<dish_name: String, (is_ordered: bool, allergens: HashSet<String>)>>
    pub fn scraper_user_menu(&self) -> HashMap<String, HashMap<String, (bool, HashSet<String>)>> {
        let page = self.request_builder.get_user_menu(); // debug
        let mut menu = HashMap::new();

        let days_selector = Selector::parse(".objednavka-obalka").unwrap();
        let date_selector = Selector::parse(".objednavka-den-obalka").unwrap();
        let dishes_name_selector = Selector::parse(".objednavka-jidlo-nazev").unwrap();
        let allergens_selector = Selector::parse(".objednavka-jidlo-alergeny-udaje").unwrap();
        let order_state_selector = Selector::parse("input[autocomplete='off']").unwrap();

        let days = page.select(&days_selector);

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
            println!("{}", date);
            for dish in dishes_of_day {
                let allergens = self.get_allergens(dishes_allergens.next().unwrap().inner_html());
                println!("{}", dish.inner_html());
                print!(" Allergens: {:?} ", allergens);
                daily_menu.insert(
                    dish.inner_html(),
                    (
                        orders_state.next().unwrap().value().attr("value").unwrap()
                            != "nezaskrtnuto",
                        allergens,
                    ),
                );
                print!(
                    "{}\n",
                    daily_menu.get(&dish.inner_html()).unwrap().0.to_string()
                );
            }
            menu.insert(date, daily_menu);
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
