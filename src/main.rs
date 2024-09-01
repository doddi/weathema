mod client;
mod components;

use crate::client::{WeatherAPI, WeatherForecastResponse};
use anathema::component::State;
use anathema::prelude::*;
use anathema::state::Value;
use clap::Parser;
use std::fs::read_to_string;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};

#[derive(Parser)]
struct Args {
    location: Option<String>,
}

#[derive(State)]
struct WeatherImageState {
    weather_image: Value<String>,
}

impl WeatherImageState {
    fn new() -> Self {
        Self {
            weather_image: Value::new("src/images/unknown.txt".into()),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let location = args.location;

    let template = read_to_string("src/templates/index.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let (tx_input, rx_input) = mpsc::channel::<String>();

    let weather_image_component_id = components::weather_image::create_component(&mut runtime);
    let temperature_range_id = components::temperature_range::create_component(&mut runtime);
    let wind_direction_id = components::wind_direction::create_component(&mut runtime);
    let _location_input_id = components::location_input::create_component(&mut runtime, tx_input, &location);

    let (tx, rx) = mpsc::channel::<WeatherForecastResponse>();

    tokio::spawn(async move {
        poll_backend_service(tx, rx_input, &location).await;
    });

    let emitter = runtime.emitter();

    tokio::spawn(async move {
        while let Ok(weather_update) = rx.recv() {
            components::temperature_range::update_component(
                &emitter,
                temperature_range_id,
                (
                    weather_update.forecasts[0].summary.report.min_temp_c,
                    weather_update.forecasts[0].summary.report.max_temp_c,
                ),
            );
            components::weather_image::update_component(
                &emitter,
                weather_image_component_id,
                weather_update.forecasts[0].summary.report.weather_type,
            );
            components::wind_direction::update_component(
                &emitter,
                wind_direction_id,
                weather_update.forecasts[0].summary.report.wind_direction.clone(),
            );
        }
    });

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}

async fn poll_backend_service(
    tx: Sender<WeatherForecastResponse>,
    rx_input: mpsc::Receiver<String>,
    initial_location: &Option<String>) {

    let weather_api = WeatherAPI::new();

    if let Some(location) = initial_location {
        if !get_weather(&tx, &weather_api, location).await { return; }
    }

    loop {
        match rx_input.try_recv() {
            Ok(entered_location) => {
                if !get_weather(&tx, &weather_api, &entered_location).await { return; }
            }
            Err(err) => {
                match err {
                    TryRecvError::Empty => {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                    TryRecvError::Disconnected => {
                        eprintln!("Disconnected Error: {}", err);
                        return;
                    }
                }
            },
        }
    }
}

async fn get_weather(tx: &Sender<WeatherForecastResponse>, weather_api: &WeatherAPI, entered_location: &str) -> bool {
    match weather_api.get_weather(entered_location).await {
        Ok(information) => {
            // Send the weather update to the main thread
            if tx.send(information).is_err() {
                println!("Receiver dropped");
                return false;
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
    true
}
