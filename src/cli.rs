use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(flatten)]
    pub tracing_opts: TracingOpts,

    /// The s3 remote
    #[clap()]
    pub remote: Url,
}

#[derive(Parser, Debug)]
pub struct TracingOpts {
    /// Sets whether jaeger exporting is enabled
    #[clap(
        long = "jaeger-enabled",
        parse(try_from_str),
        default_value = "true",
        env = "JAEGER_ENABLED"
    )]
    pub enabled: bool,

    /// Sets the jaeger service name
    #[clap(long = "jaeger-service-name", default_value = env!("CARGO_PKG_NAME"), env = "JAEGER_SERVICE_NAME")]
    pub service_name: String,
}
