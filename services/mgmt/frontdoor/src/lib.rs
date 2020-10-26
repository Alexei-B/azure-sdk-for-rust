#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-05")]
mod package_2020_05;
#[cfg(feature = "package-2020-05")]
pub use package_2020_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-04")]
mod package_2020_04;
#[cfg(feature = "package-2020-04")]
pub use package_2020_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-01")]
mod package_2020_01;
#[cfg(feature = "package-2020-01")]
pub use package_2020_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-11")]
mod package_2019_11;
#[cfg(feature = "package-2019-11")]
pub use package_2019_11::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-10")]
mod package_2019_10;
#[cfg(feature = "package-2019-10")]
pub use package_2019_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-05")]
mod package_2019_05;
#[cfg(feature = "package-2019-05")]
pub use package_2019_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-04")]
mod package_2019_04;
#[cfg(feature = "package-2019-04")]
pub use package_2019_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-03-preview")]
mod package_2019_03_preview;
#[cfg(feature = "package-2019-03-preview")]
pub use package_2019_03_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-08-preview")]
mod package_2018_08_preview;
#[cfg(feature = "package-2018-08-preview")]
pub use package_2018_08_preview::{models, operations, API_VERSION};
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
