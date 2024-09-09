mod components;

use anathema::component::State;
use anathema::prelude::*;
use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    location: Option<String>,
}

#[tokio::main]
async fn main() {
    let template = read_to_string("src/templates/index.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let graph_component_id = components::graph_component::create_component(&mut runtime);

    let emitter = runtime.emitter();

    components::graph_component::update_component(
        &emitter,
        graph_component_id,
        (1..11).collect(),
    );

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
