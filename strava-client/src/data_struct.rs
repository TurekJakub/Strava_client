use bson::de;
use bson::oid::ObjectId;
use chrono::prelude::*;
use serde::Deserialize;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::time::SystemTime;

// structure representing date - consists of DateTime representing date and day of week in czech represented by String
#[derive(Eq, Debug, Hash, Clone)]
pub struct Date {
    pub date: chrono::DateTime<Utc>,
    pub day_of_week: String,
}
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.date.eq(&other.date)
    }
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Date {
    // create new date from string in format dd.mm.yyyy
    pub fn new(date_string: String) -> Date {
        let date_data: Vec<u32> = date_string.split(".").map(|x| x.parse().unwrap()).collect();
        let days_of_week = HashMap::from([
            ("Sun", "Neděle"),
            ("Mon", "Pondělí"),
            ("Tue", "Úterý"),
            ("Wed", "Středa"),
            ("Thu", "Čtvrtek"),
            ("Fri", "Pátek"),
            ("Sat", "Sobota"),
        ]);
        let date = Utc
            .with_ymd_and_hms(date_data[2] as i32, date_data[1], date_data[0], 0, 0, 0)
            .unwrap();
        Date {
            date: date,
            day_of_week: days_of_week
                .get(date.weekday().to_string().as_str())
                .unwrap()
                .to_string(),
        }
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            format!(
                "{} {}",
                self.date.format("%d.%m.%Y").to_string(),
                self.day_of_week.as_str()
            )
            .as_str(),
        )
    }
}
// structure representing user
#[derive(Debug, Deserialize)]
pub struct User<'a> {
    #[serde(rename = "jmeno")]
    pub username: &'a str,
    #[serde(rename = "heslo")]
    pub password: &'a str,
    #[serde(rename = "cislo")]
    pub cantine: &'a str,
    pub lang: &'a str,
    #[serde(rename = "zustatPrihlasen")]
    pub stay_logged: bool,
}
// serialize user to json in format suitable for strava api request body
impl Serialize for User<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 5)?;
        s.serialize_field("heslo", &self.password)?;
        s.serialize_field("jmeno", &self.username)?;
        s.serialize_field("cislo", &self.cantine)?;
        s.serialize_field("lang", &self.lang)?;
        s.serialize_field("zustatPrihlasen", &self.stay_logged.to_string())?;
        s.end()
    }
}
// structure representing information about dish

#[derive(Clone, Debug, Serialize)]
pub struct DishInfo {
    pub id: String,
    pub allergens: Vec<String>,
    pub order_state: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelingSettings {
    pub blacklisted_dishes: Vec<String>,
    pub balacklisted_allergens: Vec<u8>,
    pub strategy: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDBEntry {
    pub id: String,
    pub username: String,
    pub settings: OrdersCancelingSettings,
    pub settings_update_time: SystemTime,
}
#[derive(Serialize, Deserialize,Debug)]
pub struct CantineDBEntry {
    pub cantine_id: String,
    pub name: String,
    pub cantine_history: Vec<ObjectId>,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct DishDBEntry {
    pub name: String,
    pub allergens: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cantine {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CantineData {
    pub v_mesto: Vec<String>,
    pub v_ulice: Vec<String>,
    pub v_nazev: Vec<String>,
    pub zarizeni: Vec<String>,
}
#[derive(Serialize, Deserialize)]
pub struct SettingsRequestBody {
    pub settings: OrdersCancelingSettings,
    pub settings_update_time: SystemTime,
}
#[derive(Serialize, Deserialize)]

pub struct OrderDishRequestBody {
    pub id: String,
    pub status: bool,
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub settings: HashMap<String, String>,
}
#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub username: String,
    pub account: f64,
}