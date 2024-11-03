use redis::AsyncCommands;
use crate::models::StockData;
use serde_json;

pub async fn publish_stock_update(stock_data: &StockData) -> redis::RedisResult<()> {
    // Connect to Redis server
    let client = redis::Client::open("redis://redis-service:6379")?;
    let mut con = client.get_multiplexed_async_connection().await?;
    
    // Serialize the stock data to JSON format
    let data = serde_json::to_string(&stock_data).expect("Failed to serialize stock data");
    
    // Publish the data on a Redis channel (e.g., "stock_updates")
    con.publish::<&str, String, ()>("stock_updates", data).await?;
    println!("Published stock update: {:?}", stock_data);
    Ok(())
}