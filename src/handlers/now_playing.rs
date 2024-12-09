use actix_web::{get, HttpResponse, Responder};
use crate::services::spotify::fetch_now_playing;

#[get("/now-playing")]
pub async fn now_playing() -> impl Responder {
    match fetch_now_playing().await {
        Ok(Some(song)) => HttpResponse::Ok().body(song),
        Ok(None) => HttpResponse::Ok().body("No song is currently playing."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}