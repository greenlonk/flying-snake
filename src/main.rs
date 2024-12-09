use actix_web::{App, HttpServer};
use dotenv::dotenv;
mod handlers;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .configure(handlers::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
