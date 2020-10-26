#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportImageParameters {
    pub source: ImportSource,
    #[serde(rename = "targetTags", skip_serializing_if = "Vec::is_empty")]
    pub target_tags: Vec<String>,
    #[serde(rename = "untaggedTargetRepositories", skip_serializing_if = "Vec::is_empty")]
    pub untagged_target_repositories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<import_image_parameters::Mode>,
}
pub mod import_image_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        NoForce,
        Force,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportSource {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "registryUri", skip_serializing_if = "Option::is_none")]
    pub registry_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ImportSourceCredentials>,
    #[serde(rename = "sourceImage")]
    pub source_image: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportSourceCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryNameCheckRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: registry_name_check_request::Type,
}
pub mod registry_name_check_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.ContainerRegistry/registries")]
        Microsoft_ContainerRegistryRegistries,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryNameStatus {
    #[serde(rename = "nameAvailable", skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationDefinition>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationPropertiesDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayDefinition {
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
pub struct OperationPropertiesDefinition {
    #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationServiceSpecificationDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationServiceSpecificationDefinition {
    #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetricSpecificationDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetricSpecificationDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "internalMetricName", skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registry {
    #[serde(flatten)]
    pub resource: Resource,
    pub sku: Sku,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(skip_serializing)]
    pub tier: Option<sku::Tier>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Classic,
        Basic,
        Standard,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Classic,
        Basic,
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryProperties {
    #[serde(rename = "loginServer", skip_serializing)]
    pub login_server: Option<String>,
    #[serde(rename = "creationDate", skip_serializing)]
    pub creation_date: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<registry_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "adminUserEnabled", skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountProperties>,
    #[serde(rename = "networkRuleSet", skip_serializing_if = "Option::is_none")]
    pub network_rule_set: Option<NetworkRuleSet>,
}
pub mod registry_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "displayStatus", skip_serializing)]
    pub display_status: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(rename = "defaultAction")]
    pub default_action: network_rule_set::DefaultAction,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[serde(rename = "ipRules", skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultAction {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<virtual_network_rule::Action>,
    pub id: String,
}
pub mod virtual_network_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ip_rule::Action>,
    pub value: String,
}
pub mod ip_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryPropertiesUpdateParameters {
    #[serde(rename = "adminUserEnabled", skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountProperties>,
    #[serde(rename = "networkRuleSet", skip_serializing_if = "Option::is_none")]
    pub network_rule_set: Option<NetworkRuleSet>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Registry>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryListCredentialsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<RegistryPassword>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryPassword {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<registry_password::Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
pub mod registry_password {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "password")]
        Password,
        #[serde(rename = "password2")]
        Password2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateCredentialParameters {
    pub name: regenerate_credential_parameters::Name,
}
pub mod regenerate_credential_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "password")]
        Password,
        #[serde(rename = "password2")]
        Password2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryUsageListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistryUsage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "currentValue", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<registry_usage::Unit>,
}
pub mod registry_usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryPolicies {
    #[serde(rename = "quarantinePolicy", skip_serializing_if = "Option::is_none")]
    pub quarantine_policy: Option<QuarantinePolicy>,
    #[serde(rename = "trustPolicy", skip_serializing_if = "Option::is_none")]
    pub trust_policy: Option<TrustPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuarantinePolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<quarantine_policy::Status>,
}
pub mod quarantine_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustPolicy {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<trust_policy::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<trust_policy::Status>,
}
pub mod trust_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Notary,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Replication {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<replication_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}
pub mod replication_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Replication>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<webhook_properties::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub actions: Vec<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<webhook_properties::ProvisioningState>,
}
pub mod webhook_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookCreateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookPropertiesCreateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookPropertiesCreateParameters {
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[serde(rename = "customHeaders", skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<webhook_properties_create_parameters::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub actions: Vec<String>,
}
pub mod webhook_properties_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookPropertiesUpdateParameters {
    #[serde(rename = "serviceUri", skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(rename = "customHeaders", skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<webhook_properties_update_parameters::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
}
pub mod webhook_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Webhook>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackConfig {
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[serde(rename = "customHeaders", skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Event>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub event_info: EventInfo,
    #[serde(rename = "eventRequestMessage", skip_serializing_if = "Option::is_none")]
    pub event_request_message: Option<EventRequestMessage>,
    #[serde(rename = "eventResponseMessage", skip_serializing_if = "Option::is_none")]
    pub event_response_message: Option<EventResponseMessage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRequestMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<EventContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "requestUri", skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(rename = "reasonPhrase", skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Actor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useragent: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "instanceID", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
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
pub struct ScopeMap {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeMapProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMapProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(rename = "creationDate", skip_serializing)]
    pub creation_date: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<scope_map_properties::ProvisioningState>,
    pub actions: Vec<String>,
}
pub mod scope_map_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMapUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopeMapPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMapPropertiesUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeMapListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScopeMap>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Token {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TokenProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenProperties {
    #[serde(rename = "creationDate", skip_serializing)]
    pub creation_date: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<token_properties::ProvisioningState>,
    #[serde(rename = "scopeMapId", skip_serializing_if = "Option::is_none")]
    pub scope_map_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TokenCredentialsProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<token_properties::Status>,
}
pub mod token_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenCredentialsProperties {
    #[serde(rename = "activeDirectoryObject", skip_serializing_if = "Option::is_none")]
    pub active_directory_object: Option<ActiveDirectoryObject>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<TokenCertificate>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<TokenPassword>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryObject {
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenCertificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<token_certificate::Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[serde(rename = "encodedPemCertificate", skip_serializing_if = "Option::is_none")]
    pub encoded_pem_certificate: Option<String>,
}
pub mod token_certificate {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "certificate1")]
        Certificate1,
        #[serde(rename = "certificate2")]
        Certificate2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenPassword {
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<token_password::Name>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
}
pub mod token_password {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "password1")]
        Password1,
        #[serde(rename = "password2")]
        Password2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TokenUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenUpdateProperties {
    #[serde(rename = "scopeMapId", skip_serializing_if = "Option::is_none")]
    pub scope_map_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<token_update_properties::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TokenCredentialsProperties>,
}
pub mod token_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Token>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateCredentialsParameters {
    #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<generate_credentials_parameters::Name>,
}
pub mod generate_credentials_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "password1")]
        Password1,
        #[serde(rename = "password2")]
        Password2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateCredentialsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<TokenPassword>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
