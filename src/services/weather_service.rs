use reqwest::Error;
use serde::{Deserialize, Serialize};
use crate::config::settings::{get_api_key, get_city, get_countrycode, get_zipcode};

#[derive(serde::Serialize, Deserialize)]
pub struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(serde::Serialize, Deserialize)]
#[derive(Clone)]
struct LocationResponse {
    name: String,
    lat: f64,
    lon: f64,
    country: String,
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
    let location = fetch_location().await.unwrap();
    let lat = location.lat;
    let lon = location.lon;
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
        lat, lon, api_key
    );
    println!("{}", &url);

    let response = reqwest::get(&url).await?;
    response.json::<WeatherResponse>().await
}

async fn fetch_location() -> Result<LocationResponse, Error> {
    if get_zipcode().is_ok() {
        fetch_location_by_zipcode_and_countrycode().await
    } else { 
        fetch_location_by_city().await
    }
}

async fn fetch_location_by_zipcode_and_countrycode() -> Result<LocationResponse, Error> {
    let api_key = get_api_key();
    let zipcode = get_zipcode().unwrap();
    let countrycode = get_countrycode().unwrap();

    let zipcode_countrycode = format!("{},{}", zipcode, countrycode);
    let url = format!(
        "https://api.openweathermap.org/geo/1.0/zip?zip={}&appid={}",
        zipcode_countrycode, api_key
    );
    println!("{}", &url);

    let response = reqwest::get(&url).await?;
    response.json::<LocationResponse>().await
}

async fn fetch_location_by_city() -> Result<LocationResponse, Error> {
    let api_key = get_api_key();
    let city = get_city().unwrap();
    let url = format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
        city, api_key
    );
    println!("{}", &url);

    let response = reqwest::get(&url).await?;
    let location_vec = response.json::<Vec<LocationResponse>>().await.expect("Err");
    let location = location_vec.first().cloned().expect("Result empty");
    Ok(location)
}