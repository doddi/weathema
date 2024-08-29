use std::sync::mpsc::Sender;
use anathema::component::{Component, ComponentId, Elements, KeyCode, KeyEvent, Value};
use anathema::prelude::*;
use anathema::state::State;
use anathema::widgets::components::events::KeyState;

struct LocationInputComponent {
    tx_input: Sender<String>,
}

impl LocationInputComponent {
    fn new(tx_input: Sender<String>) -> Self {
        Self {
            tx_input,
        }
    }
}

#[derive(State)]
struct LocationInputState {
    location: Value<String>,
    has_focus: Value<String>,
}

impl LocationInputState {
    fn new(location: String) -> Self {
        Self {
            location: Value::new(location),
            has_focus: Value::new("thin".into()),
        }
    }
}

impl Component for LocationInputComponent {
    type State = LocationInputState;
    type Message = ();

    fn on_blur(&mut self, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_, Self::State>) {
        state.has_focus.set("thin".into());
    }

    fn on_focus(&mut self, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_, Self::State>) {
       state.has_focus.set("thick".into());
    }

    fn on_key(&mut self, key: KeyEvent, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_, Self::State>) {
        match key {
            KeyEvent { code: KeyCode::Enter, state: KeyState::Press, ..} => {
                let location = state.location.to_ref().clone();
                let _ = self.tx_input.send(location);
                state.location.set("".into());
            }
            KeyEvent { code: KeyCode::Char(c), state: KeyState::Press, .. } => {
                state.location.to_mut().push(c);
            }
            KeyEvent { code: KeyCode::Backspace, state: KeyState::Press, .. } => {
                state.location.to_mut().pop();
            }
            _ => {}
        }
    }
}

pub fn create_component(
    runtime: &mut anathema::runtime::RuntimeBuilder<TuiBackend>,
    tx_input: Sender<String>,
    location: &Option<String>,
) -> ComponentId<()> {

    let location = location.clone();
    runtime
        .register_component(
            "locationInput",
            "src/templates/location_input.aml",
            LocationInputComponent::new(tx_input),
            LocationInputState::new(location.unwrap_or_else(|| "".into())),
        )
        .unwrap()
}
