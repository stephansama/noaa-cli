use crate::api::DynamicJSON;
use chrono::{DateTime, Utc};
use reqwest::{header, Client, ClientBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecipationInformation {
    pub unit_code: String,
    pub value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NOAAPointProperties {
    /// id of station to use from noaa
    grid_id: String,
    /// y location on united states grid
    grid_x: u8,
    /// y location on united states grid
    grid_y: u8,
    /// radar station received from point
    radar_station: String,
    /// url for hourly forecast
    forecast_hourly: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NOAATemperaturePeriod {
    pub is_daytime: bool,
    pub temperature: u8,
    pub number: u8,
    pub icon: String,
    pub name: String,
    pub end_time: DateTime<Utc>,
    pub wind_speed: String,
    pub start_time: DateTime<Utc>,
    pub short_forecast: String,
    pub wind_direction: String,
    pub temperature_unit: String,
    pub dewpoint: PrecipationInformation,
    pub relative_humidity: PrecipationInformation,
    pub probability_of_precipitation: PrecipationInformation,
    // temperature_trend: null,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NOAATemperatureProperties {
    /// time temperature was generated
    pub generated_at: String,
    /// hourly periods for temperature
    pub periods: Vec<NOAATemperaturePeriod>,
}

pub fn create_noaa_client() -> Result<ClientBuilder, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        std::env::var("NOAA_USER_AGENT")
            .expect("Must Have User agent")
            .parse()
            .unwrap(),
    );
    Ok(ClientBuilder::new().default_headers(headers))
}

pub async fn get_point_data(
    client: &Client,
    (lat, lng): (f32, f32),
) -> Result<NOAAPointProperties, Box<dyn std::error::Error>> {
    // https://api.weather.gov/points/{lat},{lon}
    let noaa_result = client
        .get(format!("https://api.weather.gov/points/{}%2C{}", lat, lng))
        .send()
        .await?
        .text()
        .await?;

    let json: DynamicJSON = serde_json::from_str(&noaa_result)?;

    let results: NOAAPointProperties =
        serde_json::from_value(json.dynamic_properties["properties"].clone())?;

    Ok(results)
}

pub async fn get_temperature(
    client: &Client,
    grid_data: NOAAPointProperties,
) -> Result<NOAATemperatureProperties, Box<dyn std::error::Error>> {
    let noaa_result = client
        .get(format!("{}", grid_data.forecast_hourly))
        .send()
        .await?
        .text()
        .await?;

    let json: DynamicJSON = serde_json::from_str(&noaa_result)?;

    let properties: NOAATemperatureProperties =
        serde_json::from_value(json.dynamic_properties["properties"].clone())?;

    Ok(properties)
}
