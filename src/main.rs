mod client;

use std::error::Error;
use std::fs::{read_to_string};
use std::sync::mpsc;
use rand::Rng;
use anathema::component::{Component, Elements, State};
use anathema::prelude::*;
use anathema::state::Value;
use rand::distr::{Distribution, Standard};
use crate::client::{WeatherAPI, WeatherInformation};

struct WeatherImage;

impl WeatherImage {
    fn new() -> Self {
        Self
    }
}

struct WeatherImageMessage {
    weather_type: WeatherType,
}

impl WeatherImageMessage {
    fn new(weather_type: WeatherType) -> Self {
        Self {
            weather_type
        }
    }
}

impl Component for WeatherImage {
    type State = WeatherImageState;
    type Message = WeatherImageMessage;

    fn message(&mut self, message: Self::Message, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_>) {
        match message {
            WeatherImageMessage { weather_type } => {
                state.weather_image.set(match weather_type {
                    WeatherType::Sunny => read_to_string("src/images/sunny.txt").unwrap(),
                    WeatherType::PartlyCloudy => read_to_string("src/images/partly-cloudy.txt").unwrap(),
                    WeatherType::Cloudy => read_to_string("src/images/cloudy.txt").unwrap(),
                    WeatherType::Rainy => read_to_string("src/images/rainy.txt").unwrap(),
                    WeatherType::Snowy => read_to_string("src/images/snowy.txt").unwrap(),
                    WeatherType::Stormy => read_to_string("src/images/stormy.txt").unwrap(),
                    _ => read_to_string("src/images/unknown.txt").unwrap(),
                });
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum WeatherType {
    Unknown,
    Sunny,
    PartlyCloudy,
    Cloudy,
    Rainy,
    Snowy,
    Stormy,
}

impl Distribution<WeatherType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WeatherType {
        match rng.gen_range(0..=7) {
            1 => WeatherType::Sunny,
            2 => WeatherType::PartlyCloudy,
            3 => WeatherType::Cloudy,
            4 => WeatherType::Rainy,
            5 => WeatherType::Snowy,
            _ => WeatherType::Unknown,
        }
    }
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
    let template = read_to_string("src/templates/index.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        // .enable_alt_screen()
        // .enable_raw_mode()
        // .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let weather_image_component = runtime.register_component("weatherImage", "src/templates/weather_image.aml",
                                            WeatherImage::new(), WeatherImageState::new()).unwrap();


    let (tx, rx) = mpsc::channel::<WeatherType>();

    tokio::spawn(async move {
        poll_backend_service(tx).await;
    });

    let emitter = runtime.emitter();

    tokio::spawn(async move {
        while let Ok(weather_update) = rx.recv() {
            emitter.emit(weather_image_component, WeatherImageMessage::new(weather_update)).unwrap();
        }
    });

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}

async fn poll_backend_service(mut tx: mpsc::Sender<WeatherType>) {
    loop {
        let weather_api = WeatherAPI::new();

        match weather_api.get_weather("Dyserth").await {
            Ok(information) => {
                println!("Received weather information {:?}", information);

                let weather_update = match information.weather_type {
                    1 => WeatherType::Sunny,
                    2 => WeatherType::PartlyCloudy,
                    3 => WeatherType::Cloudy,
                    4 => WeatherType::Rainy,
                    5 => WeatherType::Snowy,
                    _ => WeatherType::Unknown,
                };

                // Send the weather update to the main thread
                if tx.send(weather_update).is_err() {
                    println!("Receiver dropped");
                    return;
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}