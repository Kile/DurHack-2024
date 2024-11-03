use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct StockData {
    pub stock_name: String,
    pub price: f64,
}