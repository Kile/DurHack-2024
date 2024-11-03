mod api;
// mod kube_client;
mod redis_queue;
mod metrics;
mod models;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(api::init_routes)
            .configure(metrics::init_metrics)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}