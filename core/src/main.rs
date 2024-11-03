mod models;
// mod db;

use async_std::prelude::StreamExt;
use models::StockData;
use mongodb::{bson::{doc, DateTime as BsonDateTime}, Client as MongoClient, Collection, options::ClientOptions};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct PriceData {
    date: BsonDateTime,  // MongoDB date object
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct StockDocument {
    stock: String,
    prices: Vec<PriceData>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Redis server
    let redis_client = redis::Client::open("redis://redis-service:6379")?;
    let (mut sink, mut stream) = redis_client.get_async_pubsub().await?.split();
    sink.subscribe("stock_updates").await?;
    println!("Subscribed to stock updates");

    // MongoDB connection
    let mongo_uri = "mongodb://user:password@mongodb-service:27017";
    let mongo_client_options = ClientOptions::parse(mongo_uri).await?;
    let mongo_client = MongoClient::with_options(mongo_client_options)?;
    let database = mongo_client.database("DurHack");
    let stocks_collection = database.collection::<StockDocument>("stocks");
    println!("Connected to MongoDB");

    // Main loop to receive updates
    loop {
        let msg = stream.next().await.unwrap();
        let stock_data: StockData = serde_json::from_str(&msg.get_payload::<String>()?)?;
        println!("Received stock update: {:?}", stock_data);

        // Save to MongoDB
        save_stock_data(&stocks_collection, &stock_data).await?;
    }
}

// Save stock data function for MongoDB
async fn save_stock_data(collection: &Collection<StockDocument>, stock_data: &StockData) -> Result<(), Box<dyn std::error::Error>> {
    // Convert current UTC date to MongoDB date object
    let date_time = BsonDateTime::now();

    // Check if stock document exists
    let filter = doc! { "stock": &stock_data.stock_name };
    let update = doc! {
        "$setOnInsert": { "stock": &stock_data.stock_name },
        "$push": { "prices": { "date": date_time, "price": stock_data.price } }
    };
    let options = mongodb::options::UpdateOptions::builder().upsert(true).build();

    collection.update_one(filter, update).with_options(options).await?;
    println!("Saved stock data to MongoDB: {:?}", stock_data);

    Ok(())
}
