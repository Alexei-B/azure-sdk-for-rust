#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationsDisplayDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDisplayDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationProvider {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: StatusResult,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "trustModel", skip_serializing_if = "Option::is_none")]
    pub trust_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<status_result::Status>,
    #[serde(rename = "attestUri", skip_serializing_if = "Option::is_none")]
    pub attest_uri: Option<String>,
}
pub mod status_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Ready,
        NotReady,
        Error,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationProviderListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AttestationProvider>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationServicePatchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationServiceCreationParams {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: AttestationServiceCreationSpecificParams,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationServiceCreationSpecificParams {
    #[serde(rename = "attestationPolicy", skip_serializing_if = "Option::is_none")]
    pub attestation_policy: Option<String>,
    #[serde(rename = "policySigningCertificates", skip_serializing_if = "Option::is_none")]
    pub policy_signing_certificates: Option<JsonWebKeySet>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonWebKeySet {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<JsonWebKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonWebKey {
    pub alg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<String>,
    pub kid: String,
    pub kty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qi: Option<String>,
    #[serde(rename = "use")]
    pub use_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub x5c: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}
