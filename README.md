# flying-snake

A REST API backend for a Raspberry Pi-powered RGB LED smart panel. This backend fetches weather data from a third-party API (e.g., OpenWeatherMap) and provides it to the frontend program for display (please note this is not implemented yet).

This project is written in Rust and leverages the Actix-web framework for API development.
## Prerequisites

- Rust and Cargo installed
- OpenWeatherMap API key

## Setup

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Create a `.env` file in the root directory and add your OpenWeatherMap API key and city:
    ```env
    OPENWEATHER_API_KEY=your_api_key
    OPENWEATHER_CITY=your_city
    ```

3. Build the project:
    ```sh
    cargo build
    ```

4. Run the project:
    ```sh
    cargo run
    ```

## Usage

Once the server is running, you can fetch the weather data by making a GET request to: http://localhost:8080/weather

## Project Structure

```
src/
├── main.rs              # Entry point
├── api/                 # API endpoints
│   ├── mod.rs           # API module
│   ├── weather.rs       # Weather endpoint logic
├── services/            # External services
│   ├── weather_service.rs # Fetch weather data from OpenWeatherMap
└── config/              # Configuration files
    ├── settings.rs      # API keys, server config, etc.
```

## Dependencies

- actix-web
- reqwest
- serde
- dotenv

## License

This project is licensed under the MIT License.
