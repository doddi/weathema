use crate::WeatherImageState;
use anathema::component::{Component, ComponentId, Elements, Emitter};
use anathema::prelude::{Context, TuiBackend};
use std::fs::read_to_string;

struct WeatherImage;

impl WeatherImage {
    fn new() -> Self {
        Self
    }
}

pub(crate) struct WeatherImageMessage {
    weather_type: WeatherType,
}

impl WeatherImageMessage {
    fn new(weather_type: WeatherType) -> Self {
        Self { weather_type }
    }
}

impl Component for WeatherImage {
    type State = WeatherImageState;
    type Message = WeatherImageMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        // TODO: Consider preloading the images and storing them in the state
        let WeatherImageMessage { weather_type } = message;
        {
            state.weather_image.set(match weather_type {
                WeatherType::Sunny => read_to_string("src/images/sunny.txt").unwrap(),
                WeatherType::PartlyCloudy => {
                    read_to_string("src/images/partly-cloudy.txt").unwrap()
                }
                WeatherType::Cloudy => read_to_string("src/images/cloudy.txt").unwrap(),
                WeatherType::Rainy => read_to_string("src/images/rainy.txt").unwrap(),
                WeatherType::Snowy => read_to_string("src/images/snowy.txt").unwrap(),
                WeatherType::Stormy => read_to_string("src/images/stormy.txt").unwrap(),
                _ => read_to_string("src/images/unknown.txt").unwrap(),
            });
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum WeatherType {
    Unknown = -1,
    Sunny = 1,
    PartlyCloudy = 3,
    Cloudy,
    Rainy,
    Snowy,
    Stormy,
}

pub fn create_component(
    runtime: &mut anathema::runtime::RuntimeBuilder<TuiBackend>,
) -> ComponentId<WeatherImageMessage> {
    runtime
        .register_component(
            "weatherImage",
            "src/templates/weather_image.aml",
            WeatherImage::new(),
            WeatherImageState::new(),
        )
        .unwrap()
}

pub fn update_component(
    emitter: &Emitter,
    weather_image_component_id: ComponentId<WeatherImageMessage>,
    weather_type: u8,
) {
    let weather_update = match weather_type {
        0..=1 => WeatherType::Sunny,
        2..=4 => WeatherType::PartlyCloudy,
        5..=8 => WeatherType::Cloudy,
        9..=12 => WeatherType::Rainy,
        20 => WeatherType::Snowy, // TODO
        _ => WeatherType::Unknown,
    };
    emitter
        .emit(
            weather_image_component_id,
            WeatherImageMessage::new(weather_update),
        )
        .unwrap();
}
