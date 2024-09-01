use std::time::Duration;
use anathema::component::{Component, ComponentId, Elements, Emitter, State, Value};
use anathema::prelude::{Context, TuiBackend};

struct Spinner;

impl Spinner {
    fn new() -> Self {
        Self {
        }
    }
}

impl Component for Spinner {
    type State = SpinnerState;
    type Message = SpinnerMessage;

    fn tick(&mut self, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_, Self::State>, _dt: Duration) {
        if state.animating.to_bool() {
            let status = state.status.copy_value();
            let result = if status == 3 {
                0
            }
            else {
                status +  1
            };
            state.status.set(result);

            state.value.set(match result {
                0 => '|',
                1 => '/',
                2 => '-',
                3 => '\\',
                _ => unreachable!(),
            }.to_string());
        }
        else {
            state.value.set(' '.to_string());
        }
    }

    fn message(&mut self, message: Self::Message, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_, Self::State>) {
        state.animating.set(message.animating);
        state.status.set(0);
    }
}

#[derive(State)]
struct SpinnerState {
    #[state_ignore]
    animating: Value<bool>,
    #[state_ignore]
    status: Value<u8>,
    value: Value<String>,
}

impl SpinnerState {
    fn new() -> Self {
        Self {
            animating: Value::new(false),
            status: Value::new(0),
            value: Value::new(' '.to_string()),
        }
    }
}

pub(crate) struct SpinnerMessage {
    animating: bool,
}

impl SpinnerMessage {
    fn new(animating: bool) -> Self {
        Self {
            animating,
        }
    }
}
pub fn create_component(
    runtime: &mut anathema::runtime::RuntimeBuilder<TuiBackend>,
) -> ComponentId<SpinnerMessage> {
    runtime
        .register_component(
            "spinner",
            "src/templates/spinner.aml",
            Spinner::new(),
            SpinnerState::new(),
        )
        .unwrap()
}

pub fn update_component(
    emitter: &Emitter,
    component_id: ComponentId<SpinnerMessage>,
    animating: bool
) {
    emitter
        .emit(
            component_id,
            SpinnerMessage::new(animating),
        )
        .unwrap();
}
