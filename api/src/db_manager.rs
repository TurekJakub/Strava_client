use mongodb::{
    bson::{doc, Document},
    options::AuthMechanism,
    options::ClientOptions,
    options::Credential,
    options::ServerApi,
    options::ServerApiVersion,
    options::Tls,
    options::TlsOptions,
    Client, Collection,
};
use std::{path::PathBuf, env};
pub async fn connect() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();
    let client_options = ClientOptions::parse(format!(
        "mongodb+srv://{}:{}@cluster0.ufzbnsx.mongodb.net/?retryWrites=true&w=majority",env::var("USER").unwrap(), env::var("PASSWORD").unwrap()),
    )
    .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll
        .find_one(doc! { "title": "The Perils of Pauline" }, None)
        .await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}

