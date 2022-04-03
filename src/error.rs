use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(s3rver::io_error))]
    IoError(#[from] std::io::Error),
    #[error("Could not create Jaeger OpenTelemetry pipeline")]
    #[diagnostic(code(s3rver::jaeger_init_error))]
    JaegerPipelineError(#[source] opentelemetry::trace::TraceError),
}
