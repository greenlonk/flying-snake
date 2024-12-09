pub mod weather;
mod now_playing;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(weather::get_weather).service(now_playing::now_playing);
}