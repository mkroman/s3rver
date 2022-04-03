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
    #[error("Invalid remote: {0}")]
    #[diagnostic(code(s3rver::invalid_remote_error))]
    InvalidRemoteError(&'static str),
    #[error("AWS S3 SDK error")]
    #[diagnostic(code(s3rver::s3_sdk_error))]
    AwsS3Error(#[from] aws_sdk_s3::Error),
}
