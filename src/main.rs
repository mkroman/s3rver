use std::env;

use clap::Parser;
use miette::Result;
use opentelemetry::global;
use tracing::{debug, trace};
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

mod cli;
mod error;
mod s3;

pub use error::Error;

fn init_tracing(tracing_opts: cli::TracingOpts) -> Result<()> {
    if tracing_opts.enabled {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        // Install a new OpenTelemetry trace pipeline
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name(tracing_opts.service_name)
            .install_simple()
            .map_err(Error::JaegerPipelineError)?;

        // Create a tracing layer with the configured tracer
        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let fmt_layer = tracing_subscriber::fmt::layer();
        let env_filter = EnvFilter::from_default_env();
        let collector = tracing_subscriber::Registry::default()
            .with(ErrorLayer::default())
            .with(opentelemetry)
            .with(env_filter)
            .with(fmt_layer);

        tracing::subscriber::set_global_default(collector)
            .expect("Unable to set a global collector");
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Override RUST_LOG with a default setting if it's not set by the user
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "s3rver=trace");
    }

    let opts = cli::Opts::parse();
    init_tracing(opts.tracing_opts)?;

    let shared_config = aws_config::load_from_env().await;
    trace!("loaded config");
    let client = s3::init(&shared_config);

    debug!(buckets = ?client.list_buckets().send().await, "reading buckets");

    println!("Hello, world!");

    Ok(())
}
