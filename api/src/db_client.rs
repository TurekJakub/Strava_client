use mongodb::options::Credential;
use mongodb::options::{AuthMechanism, Tls, TlsOptions};
use mongodb::{
    bson::doc,
    options::ClientOptions,
    Client, Collection,
};
use std::env;
use std::path::PathBuf;
use std::time::SystemTime;
use strava_client::data_struct::{OrdersCancelingSettings, UserDBEntry};
struct StravaClient {
    client: mongodb::Client,
}
impl StravaClient {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            client: self::connect().await?,
        })
    }
    pub async fn get_settings_update_time(
        &self,
        username: &String,
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
        username: &String,
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
    async fn get_user(
        &self,
        username: &String,
    ) -> Result<Option<UserDBEntry>, mongodb::error::Error> {
        let collection = self.get_collection().await;
        let user = collection
            .find_one(doc! { "username": username }, None)
            .await?;
        Ok(user)
    }
    async fn create_user(&self, user: UserDBEntry) -> Result<(), mongodb::error::Error> {
        let collection = self.get_collection().await;
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
    async fn get_collection(&self) -> Collection<UserDBEntry> {
        let database = self.client.database("strava");
        database.collection("users")
    }
}
async fn connect() -> Result<mongodb::Client, mongodb::error::Error> {
    dotenv::dotenv().ok();
    let mut client_options =
            ClientOptions::parse("mongodb+srv://cluster0.ufzbnsx.mongodb.net/?authSource=%24external&authMechanism=MONGODB-X509&retryWrites=true&w=majority").await?;
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
