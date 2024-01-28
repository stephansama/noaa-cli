use super::api::DynamicJSON;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct GoogleResults {
    address_components: Vec<AddressComponents>,
    formatted_address: String,
    place_id: String,
    geometry: GeometryResponse,
}

#[derive(Debug, Deserialize, Serialize)]
struct AddressComponents {
    long_name: String,
    short_name: String,
    types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeometryResponse {
    location: GeometryLocationMap,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeometryLocationMap {
    lat: f32,
    lng: f32,
}

pub async fn get_lat_lng(zip: String) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!(
        "https://maps.googleapis.com/maps/api/geocode/json?components=postal_code:{}&key={}",
        zip,
        env::var("GOOGLE_API_KEY").expect("Must supply google api key")
    ))
    .await?
    .text()
    .await?;

    let json: DynamicJSON = serde_json::from_str(&resp)?;
    if json.dynamic_properties["status"] != "OK" {
        panic!("Response from geocoding was not ok")
    }

    let results: Vec<GoogleResults> =
        serde_json::from_value(json.dynamic_properties["results"].clone())?;

    let lat = results[0].geometry.location.lat;
    let lng = results[0].geometry.location.lng;

    return Ok((lat, lng));
}
