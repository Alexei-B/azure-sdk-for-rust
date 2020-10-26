#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-07-preview")]
mod package_2020_07_preview;
#[cfg(feature = "package-2020-07-preview")]
pub use package_2020_07_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-01")]
mod package_2020_01;
#[cfg(feature = "package-2020-01")]
pub use package_2020_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-04")]
mod package_2017_04;
#[cfg(feature = "package-2017-04")]
pub use package_2017_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-03")]
mod package_2017_03;
#[cfg(feature = "package-2017-03")]
pub use package_2017_03::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-07-preview")]
mod package_2016_07_preview;
#[cfg(feature = "package-2016-07-preview")]
pub use package_2016_07_preview::{models, operations, API_VERSION};
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
