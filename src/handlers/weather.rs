use actix_web::{get, HttpResponse, Responder};
use crate::services::weather_service;

#[get("/weather")]
async fn get_weather() -> impl Responder {
    match weather_service::fetch_weather().await {
        Ok(data) => HttpResponse::Ok().json(data), // Return weather data as JSON
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}