use std::thread;

use actix_web::{web, HttpResponse, Responder};
use crate::metrics::{API_REQUESTS_COUNTER, STOCK_UPDATES_GAUGE};
use crate::models::StockData;
use crate::redis_queue::publish_stock_update;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/save", web::post().to(save_stock_data));
}

async fn save_stock_data(data: web::Json<StockData>) -> impl Responder {
    // Use the shared Publisher instance
    let stock_data = data.into_inner();
    if let Err(e) = publish_stock_update(&stock_data).await {
        eprintln!("Failed to publish stock update: {}", e);
        // Spawn a thread tho save in prometheus that the stock update failed
        thread::spawn(|| {
            API_REQUESTS_COUNTER.with_label_values(&["failed"]).inc();
        });
        return HttpResponse::InternalServerError().json("Failed to publish stock update");
    }
    // Spawn a thread to save in prometheus that the stock update was successful
    thread::spawn(move || {
        API_REQUESTS_COUNTER.with_label_values(&["success"]).inc();
        STOCK_UPDATES_GAUGE.with_label_values(&[&stock_data.stock_name]).set(stock_data.price);
    });
    HttpResponse::Ok().json("Stock data received")
}