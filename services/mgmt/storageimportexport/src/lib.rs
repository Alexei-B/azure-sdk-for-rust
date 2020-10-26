#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2016-11")]
mod package_2016_11;
#[cfg(feature = "package-2016-11")]
pub use package_2016_11::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-08")]
mod package_2020_08;
#[cfg(feature = "package-2020-08")]
pub use package_2020_08::{models, operations, API_VERSION};
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
