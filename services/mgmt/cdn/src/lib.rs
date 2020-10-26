#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-04")]
mod package_2020_04;
#[cfg(feature = "package-2020-04")]
pub use package_2020_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-12")]
mod package_2019_12;
#[cfg(feature = "package-2019-12")]
pub use package_2019_12::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-06")]
mod package_2019_06;
#[cfg(feature = "package-2019-06")]
pub use package_2019_06::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-06-preview")]
mod package_2019_06_preview;
#[cfg(feature = "package-2019-06-preview")]
pub use package_2019_06_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-04")]
mod package_2019_04;
#[cfg(feature = "package-2019-04")]
pub use package_2019_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-10")]
mod package_2017_10;
#[cfg(feature = "package-2017-10")]
pub use package_2017_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-04")]
mod package_2017_04;
#[cfg(feature = "package-2017-04")]
pub use package_2017_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-10")]
mod package_2016_10;
#[cfg(feature = "package-2016-10")]
pub use package_2016_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-04")]
mod package_2016_04;
#[cfg(feature = "package-2016-04")]
pub use package_2016_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-06")]
mod package_2015_06;
#[cfg(feature = "package-2015-06")]
pub use package_2015_06::{models, operations, API_VERSION};
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
