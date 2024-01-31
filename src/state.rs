use crate::{
    args::Args,
    geocode::get_lat_lng,
    noaa::{create_noaa_client, get_point_data, get_temperature, NOAATemperatureProperties},
};
use clap::Parser;
use dotenv::dotenv;
use std::time::{Duration, Instant};

pub struct State {
    /// command line arguments
    pub args: Args,
    /// boolean to hold whether or not the program should exit
    pub exited: bool,
    /// current temperature properties based on zip code supplied from arguments
    pub current_temperature: Option<NOAATemperatureProperties>,
    /// amount of temperatures to skip by
    pub skip_by: usize,
    /// elapsed time since starting the program
    time: Instant,
}

impl State {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv()?;
        let args = Args::parse();
        if args.zip.len() != 5 {
            panic!("Zip code must be 5 characters long")
        }
        Ok(Self {
            args,
            time: Instant::now(),
            exited: false,
            skip_by: 0,
            current_temperature: None,
        })
    }

    pub fn elapsed(&self) -> Duration {
        self.time.elapsed()
    }

    pub async fn find_temperature(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let location = get_lat_lng(self.args.zip.clone()).await?;
        let noaa_client = create_noaa_client()?.build()?;
        let grid_point_data = get_point_data(&noaa_client, location).await?;
        Ok(self.current_temperature = Some(get_temperature(&noaa_client, grid_point_data).await?))
    }
}
