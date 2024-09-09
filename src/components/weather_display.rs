use anathema::component::{Component, ComponentId, Elements, Emitter, State, Value};
use anathema::prelude::{Context, TuiBackend};
use anathema::runtime::{GlobalEvents, RuntimeBuilder};

struct WeatherDisplay;

impl WeatherDisplay {
    fn new() -> Self {
        Self
    }
}

#[derive(State)]
struct WeatherDisplayState {
    is_loading: Value<bool>,
}

impl WeatherDisplayState {
    fn new() -> Self {
        Self {
            is_loading: Value::new(true),
        }
    }
}

pub(crate) struct WeatherDisplayMessage {
    is_loading: bool,
}

impl Component for WeatherDisplay {
    type State = WeatherDisplayState;
    type Message = WeatherDisplayMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        state.is_loading.set(message.is_loading);
    }
}

pub fn create_component(
    runtime: &mut RuntimeBuilder<TuiBackend, impl GlobalEvents>,
) -> ComponentId<WeatherDisplayMessage> {
    runtime
        .register_component(
            "weatherDisplay",
            "src/templates/weather_display.aml",
            WeatherDisplay::new(),
            WeatherDisplayState::new(),
        )
        .unwrap()
}

pub(crate) fn update_component(
    emitter: &Emitter,
    id: ComponentId<WeatherDisplayMessage>,
    is_loading: bool,
) {
    let _ = emitter.emit(id, WeatherDisplayMessage { is_loading });
}
