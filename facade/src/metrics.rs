use actix_web::{HttpResponse, Responder, web};
use prometheus::{register_int_counter_vec, register_gauge_vec, Encoder, IntCounterVec, TextEncoder, GaugeVec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref API_REQUESTS_COUNTER: IntCounterVec = register_int_counter_vec!(
        "api_requests",
        "Number of API requests",
        &["result"]
    ).unwrap();

    pub static ref STOCK_UPDATES_GAUGE: GaugeVec = register_gauge_vec!(
        "stock_prices", 
        "Prices of stocks",
        &["stock_name"]
    ).unwrap();
    
}

pub fn init_metrics(cfg: &mut web::ServiceConfig) {
    cfg.route("/metrics", web::get().to(metrics));
}

async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(String::from_utf8(buffer).unwrap())
}