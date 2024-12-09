use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
struct SpotifyTokens {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>, // Token expiration time
}

const TOKEN_FILE: &str = "spotify_tokens.json"; // File to store tokens

pub async fn get_access_token(client_id: &str, client_secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Load tokens from file
    let mut tokens = load_tokens()?;

    // Check if the token has expired
    if tokens.expires_at < Utc::now() {
        tokens = refresh_access_token(client_id, client_secret, &tokens.refresh_token).await?;
        save_tokens(&tokens)?;
    }

    Ok(tokens.access_token)
}

async fn refresh_access_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<SpotifyTokens, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(client_id, Some(client_secret))
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let access_token = response["access_token"]
        .as_str()
        .ok_or("Failed to parse access_token")?
        .to_string();

    let expires_in = response["expires_in"]
        .as_i64()
        .ok_or("Failed to parse expires_in")?;

    let new_tokens = SpotifyTokens {
        access_token,
        refresh_token: refresh_token.to_string(),
        expires_at: Utc::now() + chrono::Duration::seconds(expires_in),
    };

    Ok(new_tokens)
}

fn load_tokens() -> Result<SpotifyTokens, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(TOKEN_FILE)?;
    let tokens = serde_json::from_str(&data)?;
    Ok(tokens)
}

fn save_tokens(tokens: &SpotifyTokens) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(tokens)?;
    fs::write(TOKEN_FILE, data)?;
    Ok(())
}
