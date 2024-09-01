use anathema::component::{Component, ComponentId, Elements, Emitter, State, Value};
use anathema::prelude::{Context, TuiBackend};
use anathema::runtime::RuntimeBuilder;

struct MainHolding;

impl MainHolding {
    fn new() -> Self {
        Self
    }
}

impl Component for MainHolding {
    type State = MainHoldingState;
    type Message = MainHoldingMessage;

    fn message(&mut self, message: Self::Message, state: &mut Self::State, _elements: Elements<'_, '_>, _context: Context<'_, Self::State>) {
        state.is_loading.set(message.is_loading);
        state.value.set(message.value);
    }
}

#[derive(State)]
struct MainHoldingState {
    is_loading: Value<bool>,
    value: Value<String>,
}

impl MainHoldingState {
    fn new() -> Self {
        Self {
            is_loading: Value::new(false),
            value: Value::new("".to_string()),
        }
    }
}

pub struct MainHoldingMessage {
    is_loading: bool,
    value: String,
}

pub fn create_component(
    runtime: &mut RuntimeBuilder<TuiBackend>,
) -> ComponentId<MainHoldingMessage> {
    runtime
        .register_component(
            "mainHolding",
            "src/templates/main_holding.aml",
            MainHolding::new(),
            MainHoldingState::new(),
        )
        .unwrap()
}

pub(crate) fn update_component(emitter: &Emitter, id: ComponentId<MainHoldingMessage>, is_loading: bool, value: String) {
    let _ = emitter.emit(id, MainHoldingMessage { is_loading, value });
}
