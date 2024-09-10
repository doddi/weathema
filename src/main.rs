mod components;

use anathema::prelude::*;
use clap::Parser;
use std::fs::read_to_string;

use tracing::info;
use tracing_subscriber::prelude::*;

#[derive(Parser)]
struct Args {
    location: Option<String>,
}

#[tokio::main]
async fn main() {
    enable_tracing_support();

    info!("Starting up");

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

    components::graph_component::update_component(&emitter, graph_component_id, (1..11).collect());

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}

fn enable_tracing_support() {
    let fmt_layer = tracing_subscriber::fmt::layer();

    let telemetry_layer =
        create_otlp_tracer().map(|t| tracing_opentelemetry::layer().with_tracer(t));

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();
}

fn create_otlp_tracer() -> Option<opentelemetry_sdk::trace::Tracer> {
    if !std::env::vars().any(|(name, _)| name.starts_with("OTEL_")) {
        return None;
    }
    let tracer = opentelemetry_otlp::new_pipeline().tracing();
    let exporter = opentelemetry_otlp::new_exporter().http();
    let tracer = tracer.with_exporter(exporter);

    Some(
        tracer
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap(),
    )
}
