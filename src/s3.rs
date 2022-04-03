use aws_sdk_s3::Client;
use aws_types::sdk_config::SdkConfig;
use url::Url;

use crate::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Remote {
    pub bucket: String,
    pub prefix: String,
}

impl TryFrom<Url> for Remote {
    type Error = crate::Error;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let bucket = url
            .host_str()
            .map(String::from)
            .ok_or(Error::InvalidRemoteError("missing bucket name"))?;
        let prefix = match url.path() {
            "" => "/".to_string(),
            s => s.to_string(),
        };

        Ok(Remote { bucket, prefix })
    }
}

pub fn init(config: &SdkConfig) -> Client {
    Client::new(config)
}
