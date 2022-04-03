use aws_sdk_s3::Client;
use aws_types::sdk_config::SdkConfig;

pub fn init(config: &SdkConfig) -> Client {
    let client = Client::new(&config);

    client
}
