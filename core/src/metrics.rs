use prometheus::{register_gauge_vec, GaugeVec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STOCK_UPDATES_GAUGE: GaugeVec = register_gauge_vec!(
        "stock_prices", 
        "Prices of stocks",
        &["stock_name"]
    ).unwrap();
}