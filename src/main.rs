use actix_web::{App, HttpServer};

mod api;
mod services;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(api::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
