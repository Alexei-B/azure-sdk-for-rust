#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "isAvailiable", skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "isAvailiable", skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "serviceBusEndpoint", skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "scaleUnit", skip_serializing_if = "Option::is_none")]
    pub scale_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(rename = "namespaceType", skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<namespace_properties::NamespaceType>,
}
pub mod namespace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NamespaceType {
        Messaging,
        NotificationHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceCreateOrUpdateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: NamespaceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleProperties {
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "claimType", skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<String>,
    #[serde(rename = "claimValue", skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<String>,
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "modifiedTime", skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub properties: SharedAccessAuthorizationRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamespaceResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListKeys {
    #[serde(rename = "primaryConnectionString", skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnsCredentialProperties {
    #[serde(rename = "apnsCertificate", skip_serializing_if = "Option::is_none")]
    pub apns_certificate: Option<String>,
    #[serde(rename = "certificateKey", skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnsCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApnsCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WnsCredentialProperties {
    #[serde(rename = "packageSid", skip_serializing_if = "Option::is_none")]
    pub package_sid: Option<String>,
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "windowsLiveEndpoint", skip_serializing_if = "Option::is_none")]
    pub windows_live_endpoint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WnsCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WnsCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcmCredentialProperties {
    #[serde(rename = "gcmEndpoint", skip_serializing_if = "Option::is_none")]
    pub gcm_endpoint: Option<String>,
    #[serde(rename = "googleApiKey", skip_serializing_if = "Option::is_none")]
    pub google_api_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcmCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GcmCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpnsCredentialProperties {
    #[serde(rename = "mpnsCertificate", skip_serializing_if = "Option::is_none")]
    pub mpns_certificate: Option<String>,
    #[serde(rename = "certificateKey", skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpnsCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MpnsCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdmCredentialProperties {
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "authTokenUrl", skip_serializing_if = "Option::is_none")]
    pub auth_token_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdmCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdmCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaiduCredentialProperties {
    #[serde(rename = "baiduApiKey", skip_serializing_if = "Option::is_none")]
    pub baidu_api_key: Option<String>,
    #[serde(rename = "baiduEndPoint", skip_serializing_if = "Option::is_none")]
    pub baidu_end_point: Option<String>,
    #[serde(rename = "baiduSecretKey", skip_serializing_if = "Option::is_none")]
    pub baidu_secret_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaiduCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaiduCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "registrationTtl", skip_serializing_if = "Option::is_none")]
    pub registration_ttl: Option<String>,
    #[serde(rename = "authorizationRules", skip_serializing_if = "Vec::is_empty")]
    pub authorization_rules: Vec<SharedAccessAuthorizationRuleProperties>,
    #[serde(rename = "apnsCredential", skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[serde(rename = "wnsCredential", skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[serde(rename = "gcmCredential", skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[serde(rename = "mpnsCredential", skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[serde(rename = "admCredential", skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[serde(rename = "baiduCredential", skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubCreateOrUpdateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: NotificationHubProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationHubResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
