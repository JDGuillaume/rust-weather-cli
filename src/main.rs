use clap::Parser;
use dotenv;
use reqwest;
use serde::Deserialize;

// Latitude and Longitude for Lakewood, OH.
const LAT: f32 = 41.478130;
const LON: f32 = -81.804850;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in the terminal!", long_about = None)]

struct Args {
    /// Number of days for the forecast.
    #[arg(short, default_value_t = 0)]
    days: u8,
}

#[derive(Debug, Deserialize)]
struct Coordinates {
    lat: f32,
    lon: f32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    coord: Coordinates,
    weather: Vec<Weather>,
    main: Main,
    base: String,
}

fn main() -> Result<(), reqwest::Error> {
    // Access the API Key from .env.
    dotenv::dotenv().ok();

    let mut api_key: Option<String> = None;

    for (key, value) in std::env::vars() {
        if key != "API_KEY" {
            continue;
        };

        api_key = Some(value);
    }

    if api_key.is_none() {
        panic!("Please set the API_KEY in your .env file.")
    }

    let api_key = api_key.unwrap();

    // Set Arguments
    let args = Args::parse();

    // Choose which API is hit.
    let method = match args.days {
        0 => "weather",
        _ => "forecast",
    };

    let count = args.days * 8;

    let url: String = format!("https://api.openweathermap.org/data/2.5/{method}?lat={LAT}&lon={LON}&appid={api_key}&units=imperial&cnt={count}");

    let weather: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    println!("{} {:?}", weather.main.temp, weather.weather[0].description);

    Ok(())
}
