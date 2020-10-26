#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-05")]
mod package_2018_05;
#[cfg(feature = "package-2018-05")]
pub use package_2018_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-03-preview")]
mod package_2018_03_preview;
#[cfg(feature = "package-2018-03-preview")]
pub use package_2018_03_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-10")]
mod package_2017_10;
#[cfg(feature = "package-2017-10")]
pub use package_2017_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-09")]
mod package_2017_09;
#[cfg(feature = "package-2017-09")]
pub use package_2017_09::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-04")]
mod package_2016_04;
#[cfg(feature = "package-2016-04")]
pub use package_2016_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-05-preview")]
mod package_2015_05_preview;
#[cfg(feature = "package-2015-05-preview")]
pub use package_2015_05_preview::{models, operations, API_VERSION};
pub struct OperationConfig {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    pub token_credential_resource: String,
}
impl OperationConfig {
    pub fn new(token_credential: Box<dyn azure_core::TokenCredential>) -> Self {
        Self {
            token_credential: Some(token_credential),
            ..Default::default()
        }
    }
}
impl Default for OperationConfig {
    fn default() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            client: reqwest::Client::new(),
            base_path: "https://management.azure.com".to_owned(),
            token_credential: None,
            token_credential_resource: "https://management.azure.com/".to_owned(),
        }
    }
}
