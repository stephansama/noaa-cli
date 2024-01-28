use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Zipcode of city to search for
    #[arg(short, long, default_value_t = String::from(""))]
    pub zip: String,

    // @todo update to accept city as a valid argument
    /// Name of the city to search for
    #[arg(short, long, default_value_t = String::from(""))]
    pub city: String,
}
