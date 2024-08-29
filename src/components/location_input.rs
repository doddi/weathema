use anathema::component::{Component, ComponentId, Elements, Emitter, KeyCode, KeyEvent, Value};
use anathema::prelude::*;
use anathema::state::State;
use anathema::widgets::components::events::KeyState;

#[derive(Default)]
struct LocationInputComponent;

#[derive(Default, State)]
struct LocationInputState {
    location: Value<String>,
}

impl Component for LocationInputComponent {
    type State = LocationInputState;
    type Message = ();

    fn on_key(&mut self, key: KeyEvent, state: &mut Self::State, elements: Elements<'_, '_>, context: Context<'_, Self::State>) {
        println!("Key pressed: {:?}", key);
        match key {
            KeyEvent { code: KeyCode::Enter, state: KeyState::Release, ..} => {
                // TODO: Let others know that an enter key has been pressed
                println!("Enter pressed");
            }
            KeyEvent { code: KeyCode::Char(c), state: KeyState::Release, .. } => {
                state.location.to_mut().push(c);
            }
            _ => {}
        }
    }
}

pub fn create_component(
    runtime: &mut anathema::runtime::RuntimeBuilder<TuiBackend>,
) -> ComponentId<()> {
    runtime
        .register_component(
            "locationInput",
            "src/templates/location_input.aml",
            LocationInputComponent::default(),
            LocationInputState::default(),
        )
        .unwrap()
}
