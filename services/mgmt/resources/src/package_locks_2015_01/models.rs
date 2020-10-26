#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<management_lock_properties::Level>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
pub mod management_lock_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        NotSpecified,
        CanNotDelete,
        ReadOnly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementLockProperties>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementLockListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementLockObject>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
