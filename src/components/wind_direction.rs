use anathema::component::{Component, ComponentId, Elements, Emitter, Value};
use anathema::prelude::*;
use anathema::state::State;

#[derive(Default)]
struct WindDirectionComponent;

#[allow(dead_code)]
enum WindDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(State)]
struct WindDirectionState {
    direction: Value<String>,
}

impl WindDirectionState {
    fn new() -> Self {
        Self {
            direction: Value::new("".into()),
        }
    }
}

pub(crate) struct WindDirectionMessage {
    direction: String,
}

impl WindDirectionMessage {
    fn new(direction: String) -> Self {
        Self { direction }
    }
}
impl Component for WindDirectionComponent {
    type State = WindDirectionState;
    type Message = WindDirectionMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        // let dir = match message.direction.as_str() {
        //     "N" => WindDirection::North,
        //     "NE" => WindDirection::NorthEast,
        //     "E" => WindDirection::East,
        //     "SE" => WindDirection::SouthEast,
        //     "S" => WindDirection::South,
        //     "SW" => WindDirection::SouthWest,
        //     "W" => WindDirection::West,
        //     "NW" => WindDirection::NorthWest,
        //     _ => WindDirection::North,
        // };
        state.direction.set(message.direction);
    }
}

pub fn create_component(
    runtime: &mut anathema::runtime::RuntimeBuilder<TuiBackend>,
) -> ComponentId<WindDirectionMessage> {
    runtime
        .register_component(
            "windDirection",
            "src/templates/wind_direction.aml",
            WindDirectionComponent,
            WindDirectionState::new(),
        )
        .unwrap()
}

pub fn update_component(
    emitter: &Emitter,
    temp_range_component_id: ComponentId<WindDirectionMessage>,
    wind_direction: String,
) {
    emitter
        .emit(
            temp_range_component_id,
            WindDirectionMessage::new(wind_direction),
        )
        .unwrap();
}
