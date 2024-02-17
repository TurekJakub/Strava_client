use bson::oid::ObjectId;
use bson::{Document, Regex};
use futures_util::io::Cursor;
use futures_util::stream::StreamExt;

use mongodb::options::{AuthMechanism, Tls, TlsOptions};
use mongodb::options::{Credential, UpdateOptions};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};

use std::env;
use std::path::PathBuf;
use std::time::SystemTime;
use strava_client::data_struct::{
    CantineDBEntry, DBHistoryQuery, DishDBEntry, OrdersCancelingSettings, UserDBEntry,
};

pub struct DbClient {
    client: mongodb::Client,
}
impl DbClient {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            client: self::connect().await?,
        })
    }
    pub async fn get_settings_update_time(
        &self,
        username: &str,
    ) -> Result<Option<SystemTime>, mongodb::error::Error> {
        let user = self.get_user(username).await?;
        match user {
            Some(user) => {
                let settings_update_time = user.settings_update_time;
                Ok(Some(settings_update_time))
            }
            None => Ok(None),
        }
    }
    pub async fn get_settings(
        &self,
        id: &str,
    ) -> Result<Option<OrdersCancelingSettings>, mongodb::error::Error> {
        let user = self.get_user(id).await?;
        match user {
            Some(user) => Ok(Some(user.settings)),
            None => Ok(None),
        }
    }
    pub async fn insert_user(&self, user: UserDBEntry) -> Result<(), mongodb::error::Error> {
        match self.get_user(&user.id).await? {
            Some(_) => {
                self.update_user(user).await?;
                Ok(())
            }
            None => {
                self.create_user(user).await?;
                Ok(())
            }
        }
    }
    async fn get_user(&self, id: &str) -> Result<Option<UserDBEntry>, mongodb::error::Error> {
        let collection = self.get_users_collection().await;
        let user = collection.find_one(doc! { "id": id }, None).await;
        user
    }
    async fn create_user(&self, user: UserDBEntry) -> Result<(), mongodb::error::Error> {
        let collection = self.get_users_collection().await;
        collection.insert_one(user, None).await?;
        Ok(())
    }
    async fn update_user(&self, user: UserDBEntry) -> Result<(), mongodb::error::Error> {
        let database = self.client.database("strava");
        let collection: Collection<UserDBEntry> = database.collection("users");
        collection
            .update_one(
                doc! { "username": user.username },
                doc! {
                        "$set": doc! { "update_time": serde_json::to_string(&user.settings_update_time).unwrap(),
                                      "settings": serde_json::to_string(&user.settings).unwrap(), }
                },
                None,
            )
            .await?;
        Ok(())
    }
    pub async fn get_cantine(
        &self,
        cantine_id: &String,
    ) -> Result<Option<CantineDBEntry>, mongodb::error::Error> {
        let collection = self.get_cantines_collection().await;
        let cantine = collection
            .find_one(doc! { "cantine_id": cantine_id }, None)
            .await;
        cantine
    }
    async fn create_cantine(&self, cantine: CantineDBEntry) -> Result<(), mongodb::error::Error> {
        let collection = self.get_cantines_collection().await;
        collection.insert_one(cantine, None).await?;
        Ok(())
    }
    pub async fn update_cantine(
        &self,
        cantine_id: &str,
        cantine_history: Vec<ObjectId>,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.get_cantines_collection().await;
        let mut result = collection
            .aggregate(
                [
                    doc! {
                       "$match": doc!{
                           "cantine_id": cantine_id
                        }
                    },
                    doc! {
                        "$project":  doc!{
                            "cantine_id": "$cantine_id",
                            "name": "$name",
                            "cantine_history": doc!{
                                "$setUnion": [
                                    "$cantine_history",
                                       cantine_history
                                        ]
                                    }
                        }
                    },
                ],
                None,
            )
            .await?;
        match result.next().await {
            Some(doc) => {
                let doc: CantineDBEntry = bson::from_document(doc?)?;
                collection
                    .update_one(
                        doc! { "cantine_id": cantine_id },
                        doc! {
                                "$set": doc! { "cantine_history": doc.cantine_history }
                        },
                        None,
                    )
                    .await?;
                Ok(())
            }
            None => Ok(()),
        }
    }
    pub async fn insert_cantine(
        &self,
        cantine: CantineDBEntry,
    ) -> Result<(), mongodb::error::Error> {
        match self.get_cantine(&cantine.cantine_id).await? {
            Some(_) => {
                self.update_cantine(cantine.cantine_id.as_str(), cantine.cantine_history)
                    .await?;
                Ok(())
            }
            None => {
                self.create_cantine(cantine).await?;
                Ok(())
            }
        }
    }
    pub async fn insert_dish(
        &self,
        dish: &DishDBEntry,
    ) -> Result<Option<ObjectId>, mongodb::error::Error> {
        let collection = self.get_dishes_collection().await;
        let options = UpdateOptions::builder().upsert(true).build();
        let res = collection
            .update_one(
                doc! {"name": dish.name.clone(), "allergens":dish.allergens.clone()},
                doc! { "$setOnInsert": doc!{"name":dish.name.clone(), "allergens":dish.allergens.clone()}},
                options,
            )
            .await?;
        match res.upserted_id {
            Some(id) => Ok(Some(id.as_object_id().unwrap())),
            None => Ok(None),
        }
    }
    pub async fn insert_dishes(
        &self,
        dishes: Vec<DishDBEntry>,
    ) -> Result<Vec<ObjectId>, mongodb::error::Error> {
        let mut updated = Vec::new();
        for dish in dishes {
            match self.insert_dish(&dish).await? {
                Some(id) => updated.push(id),
                None => match self.get_dish_id(&dish.name, &dish.allergens).await {
                    Ok(Some(id)) => updated.push(id),
                    Ok(None) => continue,
                    Err(_) => continue,
                },
            }
        }
        Ok(updated)
    }
    pub async fn get_dish_id(
        &self,
        name: &String,
        allergens: &Vec<String>,
    ) -> Result<Option<ObjectId>, mongodb::error::Error> {
        let collection: Collection<Document> = self.client.database("strava").collection("dishes");
        let dish = collection
            .find_one(doc! {"name": name, "allergens": allergens}, None)
            .await?;
        match dish {
            Some(dish) => Ok(Some(dish.get_object_id("_id").unwrap().clone())),
            None => Ok(None),
        }
    }
    pub async fn get_cantine_history(
        &self,
        cantine_id: &str,
    ) -> Result<Vec<DishDBEntry>, mongodb::error::Error> {
        // TODO write aggregation
        Ok(Vec::new())
    }
    async fn get_users_collection(&self) -> Collection<UserDBEntry> {
        let database = self.client.database("strava");
        database.collection("users")
    }
    async fn get_cantines_collection(&self) -> Collection<CantineDBEntry> {
        let database = self.client.database("strava");
        database.collection("cantines")
    }
    async fn get_dishes_collection(&self) -> Collection<DishDBEntry> {
        let database = self.client.database("strava");
        database.collection("dishes")
    }
    /*
        Get cantine histrory db query
        [
        doc! {
            "$match": doc! {
                "cantine_id": cantine_id
            }
        },
        doc! {
            "$unwind": doc! {
                "path": "$cantine_history"
            }
        },
        doc! {
            "$lookup": doc! {
                "from": "dishes",
                "localField": "cantine_history",
                "foreignField": "_id",
                "as": "dish"
            }
        },
        doc! {
            "$unwind": doc! {
                "path": "$dish"
            }
        },
        doc! {
            "$group": doc! {
                "_id": "$_id",
                "dishes": doc! {
                    "$push": "$dish"
                },
                "cantine_history": doc! {
                    "$push": "$cantine_history"
                }
            }
        }
    ]
     */
    pub async fn query_cantine_history(
        &self,
        cantine_id: &str,
        query: &str,
    ) -> Result<Vec<DishDBEntry>, String> {
        let reslut_stream = self
            .get_cantines_collection()
            .await
            .aggregate(
                [
                    doc! {
                        "$match": doc! {
                            "cantine_id": cantine_id
                        }
                    },
                    doc! {
                        "$unwind": doc! {
                            "path": "$cantine_history",
                            "preserveNullAndEmptyArrays": false
                        }
                    },
                    doc! {
                        "$lookup": doc! {
                            "from": "dishes",
                            "localField": "cantine_history",
                            "foreignField": "_id",
                            "as": "dish"
                        }
                    },
                    doc! {
                        "$unwind": doc! {
                            "path": "$dish",
                            "preserveNullAndEmptyArrays": false
                        }
                    },
                    doc! {
                        "$match": doc! {
                            "dish.name": doc! {
                                "$regex": Regex { pattern: input_to_regex_string(query), options: "i".to_string() }
                        }
                    }
                    },
                    doc! {
                        "$group": doc! {
                            "_id": "id",
                            "dishes": doc! {
                                "$push": "$dish"
                            }
                        }
                    },
                ],
                None,
            )
            .await;
        match reslut_stream {
            Ok(mut stream) => {
                let result_option = stream.next().await;
                match result_option {
                    Some(result) => match result {
                        Ok(doc) => match bson::from_document::<DBHistoryQuery>(doc) {
                            Ok(results) => Ok(results.dishes),
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        },
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    },
                    None => {
                        return Ok(Vec::new());
                    }
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

async fn connect() -> Result<mongodb::Client, mongodb::error::Error> {
    dotenv::dotenv().ok();
    let mut client_options = ClientOptions::parse(env::var("CONNECTION_STRING").unwrap()).await?;
    client_options.credential = Some(
        Credential::builder()
            .mechanism(AuthMechanism::MongoDbX509)
            .build(),
    );
    let tls_options = TlsOptions::builder()
        .cert_key_file_path(PathBuf::from(env::var("CERT_PATH").unwrap()))
        .build();
    client_options.tls = Some(Tls::Enabled(tls_options));
    let client = Client::with_options(client_options)?;
    Ok(client)
}
fn input_to_regex_string(input: &str) -> String {
    let str = replace_multiple(
        &input.to_lowercase(),
        vec![
            ("a", "[a,á]"),
            ("e", "[e,é,ě]"),
            ("y", "[y,ý]"),
            ("n", "[n,ň]"),
            ("c", "[c,č]"),
            ("r", "[r,ř]"),
            ("z", "[z,ž]"),
            ("s", "[s,š]"),
            ("t", "[t,ť]"),
            ("d", "[d,ď]"),
            ("u", "[u,ů,ú]"),
            ("i", "[i,í]"),
            ("o", "[o,ó]"),
        ],
    );
   // format!("^{}", str.trim())
   str.trim().to_string()
}
fn replace_multiple(input: &str, replacements: Vec<(&str, &str)>) -> String {
    let mut result = input.to_string();
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}

