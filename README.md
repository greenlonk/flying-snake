# flying-snake

A REST API backend for a Raspberry Pi-powered RGB LED smart panel. This backend fetches weather data from a third-party API (e.g., OpenWeatherMap) and provides it to the frontend program for display (please note this is not implemented yet).
If you provide a `spotify_tokens.json` file with valid Spotify API tokens, the backend can also fetch the currently playing track on Spotify.
This project is written in Rust and leverages the Actix-web framework for API development.

## Prerequisites

- Rust and Cargo installed
- OpenWeatherMap API key
- Spotify API credentials (Client ID and Client Secret)
- Spotify API token in a `spotify_tokens.json` file in the following format:
  ```json
  {
      "access_token": "your_access_token",
      "expires_in": "<utc_timestamp>",
      "refresh_token": "your_refresh_token"
  }
```

## Setup

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Create a `.env` file in the root directory and add your OpenWeatherMap API key, city, zipcode, country code, and Spotify API credentials:
    ```env
   # Weather API
    OPENWEATHER_API_KEY=your_api_key
    OPENWEATHER_CITY=your_city
    OPENWEATHER_ZIPCODE=your_zipcode
    OPENWEATHER_COUNTRYCODE=your_countrycode
   # Spotify API
    SPOTIFY_CLIENT_ID=your_spotify_client_id
    SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
    ```
   Zipcode and country code are optional, but if used, both must be present.

3. Build the project:
    ```sh
    cargo build
    ```

4. Run the project:
    ```sh
    cargo run
    ```

## Usage

Once the server is running, you can fetch current weather data by making a GET request to: http://localhost:8080/weather

To fetch the currently playing track on Spotify, make a GET request to: http://localhost:8080/now-playing

## Project Structure
```
src/ 
   ├── main.rs # Entry point
   ├── handlers/ # API endpoints 
   │ ├── mod.rs # API module 
   │ ├── weather.rs # Weather endpoint logic 
   │ ├── now_playing.rs # Spotify endpoint logic 
   ├── services/ # External services 
   │ ├── mod.rs # Service module
   │ ├── spotify.rs # Fetch currently playing track on Spotify
   │ ├── weather_service.rs # Fetch weather data from OpenWeatherMap 
   │ ├── token_manager.rs # Manages Spotify token retrieval and refresh
   └── utils/ # Configuration files 
     ├── settings.rs # API keys, server config, etc. 
     ├── config.rs # Environment variable handling
```
## Dependencies

- actix-web
- reqwest
- serde
- dotenv
- chrono
- serde_json

## License

This project is licensed under the MIT License.