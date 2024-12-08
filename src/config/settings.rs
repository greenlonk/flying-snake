use std::env;

pub fn get_api_key() -> String {
    env::var("OPENWEATHER_API_KEY").expect("API key not set")
}

pub fn get_city() -> String {
    env::var("OPENWEATHER_CITY").expect("City not set")
}