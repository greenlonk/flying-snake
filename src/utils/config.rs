use std::env;
use std::env::VarError;
use token_manager::get_access_token;
use crate::services::token_manager;

pub fn get_api_key() -> String {
    env::var("OPENWEATHER_API_KEY").expect("API key not set")
}

pub fn get_city() -> Result<String, VarError> {
    env::var("OPENWEATHER_CITY")
}

pub fn get_zipcode() -> Result<String, VarError> {
    env::var("OPENWEATHER_ZIPCODE")
}

pub fn get_countrycode() -> Result<String, VarError> {
    env::var("OPENWEATHER_COUNTRYCODE")
}

pub async fn get_spotify_access_token() -> String {
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("Client ID missing!");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("Client secret missing!");
    get_access_token(&client_id, &client_secret).await.unwrap()
}