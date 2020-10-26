#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub family: sku::Family,
    pub name: sku::Name,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        A,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "premium")]
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    pub permissions: Permissions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[serde(rename = "vaultUri", skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub sku: Sku,
    #[serde(rename = "accessPolicies")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[serde(rename = "enabledForDeployment", skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[serde(rename = "enabledForDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[serde(rename = "enabledForTemplateDeployment", skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCreateOrUpdateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: VaultProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: VaultProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Resource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
