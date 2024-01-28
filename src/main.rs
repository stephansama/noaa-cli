// use std::collections::HashMap;
use args::Args;
use clap::Parser;
use dotenv::dotenv;
use google::get_lat_lng;
use noaa::{create_noaa_client, get_point_data, get_temperature};
use std::time::Instant;

mod api;
mod args;
mod google;
mod noaa;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Instant::now();
    let args = Args::parse();
    println!("{:?}", args);
    dotenv()?;
    if args.zip.len() != 5 {
        panic!("Zip code must be 5 characters long")
    }

    let location = get_lat_lng(args.zip).await?;

    let noaa_client = create_noaa_client()?.build()?;

    let grid_point_data = get_point_data(&noaa_client, location).await?;

    let temperature_date = get_temperature(&noaa_client, grid_point_data).await?;

    // @todo update to use ratatui interface
    for period in temperature_date.periods {
        println!(
            "{}: {}{}",
            period.start_time, period.temperature, period.temperature_unit
        );
    }

    let seconds = timer.elapsed();
    println!("took {:#?} to complete", seconds);

    return Ok(());
}
