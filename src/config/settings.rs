use std::env;
use std::env::VarError;

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