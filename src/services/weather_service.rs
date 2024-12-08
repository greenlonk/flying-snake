use reqwest::Error;
use serde::{Deserialize, Serialize};
use crate::config::settings::{get_api_key, get_city};

#[derive(serde::Serialize, Deserialize)]
pub struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Deserialize, Serialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Serialize)]
struct Main {
    temp: f64,
}

pub async fn fetch_weather() -> Result<WeatherResponse, Error> {
    let api_key = get_api_key();
    let city = get_city();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather=q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url).await?;
    response.json::<WeatherResponse>().await
}