use serde::{Deserialize, Serialize};
use std::env;

pub type Locations = Vec<Location>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    place_id: i64,
    licence: String,
    boundingbox: Vec<String>,
    lat: String,
    lon: String,
    display_name: String,
    class: String,
    #[serde(rename = "type")]
    location_type: String,
    importance: f64,
    osm_type: Option<String>,
    osm_id: Option<i64>,
}

pub async fn get_lat_lng(zip: String) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let api_key = env::var("GEOCODING_API_KEY").expect("Must supply geocoding api key");
    let resp = reqwest::get(format!(
        "https://geocode.maps.co/search?q={},US&api_key={}",
        zip, api_key
    ))
    .await?
    .text()
    .await?;

    let json: Locations = serde_json::from_str(&resp)?;

    Ok((json[0].lat.clone().parse()?, json[0].lon.clone().parse()?))
}
