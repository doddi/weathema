mod client;
mod components;

use clap::Parser;
use std::fs::{read_to_string};
use std::sync::mpsc;
use rand::Rng;
use anathema::component::{Component, State};
use anathema::prelude::*;
use anathema::state::Value;
use crate::client::{WeatherAPI, WeatherInformation};

#[derive(Parser)]
struct Args {
    location: String,
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

    let weather_image_component_id = components::weather_image::create_component(&mut runtime);
    let temperature_range_id = components::temperature_range::create_component(&mut runtime);

    let (tx, rx) = mpsc::channel::<WeatherInformation>();

    tokio::spawn(async move {
        poll_backend_service(tx, location.as_str()).await;
    });

    let emitter = runtime.emitter();

    tokio::spawn(async move {
        while let Ok(weather_update) = rx.recv() {
            components::temperature_range::update_component(&emitter, temperature_range_id, (weather_update.min_temperature, weather_update.max_temperature));
            components::weather_image::update_weather_image_component(&emitter, weather_image_component_id, weather_update.weather_type);
        }
    });

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}

async fn poll_backend_service(tx: mpsc::Sender<WeatherInformation>, location: &str) {

    // TODO: Update this loop to wait for receipt of a message from the main thread
    loop {
        let weather_api = WeatherAPI::new();

        match weather_api.get_weather(location).await {
            Ok(information) => {
                // Send the weather update to the main thread
                if tx.send(information).is_err() {
                    println!("Receiver dropped");
                    return;
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}