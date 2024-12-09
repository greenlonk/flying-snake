use reqwest::Client;
use serde::Deserialize;
use crate::utils::config::get_spotify_access_token;

#[derive(Deserialize)]
struct NowPlayingResponse {
    item: Option<Track>,
}

#[derive(Deserialize)]
struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(Deserialize)]
struct Artist {
    name: String,
}

pub async fn fetch_now_playing() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.spotify.com/v1/me/player/currently-playing";
    let access_token = get_spotify_access_token().await;
    
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<NowPlayingResponse>()
        .await?;
    
    if let Some(track) = response.item {
        let artists = track.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ");        
        Ok(Some(format!("{} by {}", track.name, artists)))
    } else {
        Ok(None)
    }
}