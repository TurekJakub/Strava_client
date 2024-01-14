use mongodb::options::Credential;
use mongodb::options::{AuthMechanism, Tls, TlsOptions};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use std::env;
use std::path::PathBuf;
use std::time::SystemTime;
use strava_client::data_struct::{CantineDBEntry, OrdersCancelingSettings, UserDBEntry};
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
        username: &str,
    ) -> Result<Option<OrdersCancelingSettings>, mongodb::error::Error> {
        let user = self.get_user(username).await?;
        match user {
            Some(user) => Ok(Some(user.settings)),
            None => Ok(None),
        }
    }
    pub async fn insert_user(&self, user: UserDBEntry) -> Result<(), mongodb::error::Error> {
        match self.get_user(&user.username).await? {
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
    async fn get_user(&self, username: &str) -> Result<Option<UserDBEntry>, mongodb::error::Error> {
        let collection = self.get_users_collection().await;
        let user = collection
            .find_one(doc! { "username": username }, None)
            .await;
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
        cantine_id: &i32,
    ) -> Result<Option<CantineDBEntry>, mongodb::error::Error> {
        let collection = self.get_cantines_collection().await;
        let cantine = collection.find_one(doc! { "cantine_id": cantine_id }, None).await;
        cantine
    }
    async fn create_cantine(&self, cantine: CantineDBEntry) -> Result<(), mongodb::error::Error> {
        let collection = self.get_cantines_collection().await;
        collection.insert_one(cantine, None).await?;
        Ok(())
    }
    async fn update_cantine(&self, cantine: CantineDBEntry) -> Result<(), mongodb::error::Error> {
        let database = self.client.database("strava");
        let collection: Collection<UserDBEntry> = database.collection("cantines");
        collection
            .update_one(
                doc! { "cantine_id": cantine.cantine_id },
                doc! {
                        "$set": doc! { "dish_history": serde_json::to_string(&cantine.dish_history).unwrap()}
                },
                None,
            )
            .await?;
        Ok(())
    }
    pub async fn insert_cantine(
        &self,
        cantine: CantineDBEntry,
    ) -> Result<(), mongodb::error::Error> {
        match self.get_cantine(&cantine.cantine_id).await? {
            Some(_) => {
                self.update_cantine(cantine).await?;
                Ok(())
            }
            None => {
                self.create_cantine(cantine).await?;
                Ok(())
            }
        }
    }
    async fn get_users_collection(&self) -> Collection<UserDBEntry> {
        let database = self.client.database("strava");
        database.collection("users")
    }
    async fn get_cantines_collection(&self) -> Collection<CantineDBEntry> {
        let database = self.client.database("strava");
        database.collection("cantines")
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
