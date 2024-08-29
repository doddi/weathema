use anathema::component::{Component, ComponentId, Elements, Emitter, Value};
use anathema::prelude::*;
use anathema::state::State;

struct TemperatureRange;

impl TemperatureRange {
    fn new() -> Self {
        Self {}
    }
}

#[derive(State)]
struct TemperatureRangeState {
    min_temperature: Value<f64>,
    max_temperature: Value<f64>,
}

impl TemperatureRangeState {
    fn new() -> Self {
        Self {
            min_temperature: Value::new(0.0),
            max_temperature: Value::new(0.0),
        }
    }
}

pub(crate) struct TemperatureRangeMessage {
    min_temperature: f64,
    max_temperature: f64,
}

impl TemperatureRangeMessage {
    fn new(temperature_range: (f64, f64)) -> Self {
        Self {
            min_temperature: temperature_range.0,
            max_temperature: temperature_range.1,
        }
    }
}
impl Component for TemperatureRange {
    type State = TemperatureRangeState;
    type Message = TemperatureRangeMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        state.min_temperature.set(message.min_temperature);
        state.max_temperature.set(message.max_temperature);
    }
}

pub fn create_component(
    runtime: &mut anathema::runtime::RuntimeBuilder<TuiBackend>,
) -> ComponentId<TemperatureRangeMessage> {
    runtime
        .register_component(
            "temperatureRange",
            "src/templates/temperature_range.aml",
            TemperatureRange::new(),
            TemperatureRangeState::new(),
        )
        .unwrap()
}

pub fn update_component(
    emitter: &Emitter,
    temp_range_component_id: ComponentId<TemperatureRangeMessage>,
    temperature_range: (f64, f64),
) {
    emitter
        .emit(
            temp_range_component_id,
            TemperatureRangeMessage::new(temperature_range),
        )
        .unwrap();
}
