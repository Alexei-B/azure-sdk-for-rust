#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExtendedFilter {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourceFilter {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagvalue: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupFilter {
    #[serde(rename = "tagName", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "tagValue", skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateLink {
    pub uri: String,
    #[serde(rename = "contentVersion", skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParametersLink {
    pub uri: String,
    #[serde(rename = "contentVersion", skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(rename = "templateLink", skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "parametersLink", skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    pub mode: deployment_properties::Mode,
    #[serde(rename = "debugSetting", skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
}
pub mod deployment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DebugSetting {
    #[serde(rename = "detailLevel", skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    pub properties: DeploymentProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExportResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceManagementErrorWithDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ResourceManagementErrorWithDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasPathType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "apiVersions", skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<AliasPathType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderResourceType {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<AliasType>,
    #[serde(rename = "apiVersions", skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "registrationState", skip_serializing)]
    pub registration_state: Option<String>,
    #[serde(rename = "resourceTypes", skip_serializing)]
    pub resource_types: Vec<ProviderResourceType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicDependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    #[serde(rename = "dependsOn", skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<BasicDependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentPropertiesExtended {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "correlationId", skip_serializing)]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<Provider>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(rename = "templateLink", skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "parametersLink", skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<deployment_properties_extended::Mode>,
    #[serde(rename = "debugSetting", skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
}
pub mod deployment_properties_extended {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentValidateResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResourceManagementErrorWithDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExtended {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentExtended>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Provider>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "managedBy", skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourceExpanded {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(rename = "createdTime", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(rename = "changedTime", skip_serializing)]
    pub changed_time: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "promotionCode", skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GenericResourceExpanded>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroup {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGroupProperties>,
    pub location: String,
    #[serde(rename = "managedBy", skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceGroup>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesMoveInfo {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(rename = "targetResourceGroup", skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportTemplateRequest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagCount {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagValue {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "tagValue", skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagDetails {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "tagName", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<TagValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagDetails>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOperationProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(rename = "serviceRequestId", skip_serializing)]
    pub service_request_id: Option<String>,
    #[serde(rename = "statusCode", skip_serializing)]
    pub status_code: Option<String>,
    #[serde(rename = "statusMessage", skip_serializing)]
    pub status_message: Option<serde_json::Value>,
    #[serde(rename = "targetResource", skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<TargetResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<HttpMessage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOperation {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "operationId", skip_serializing)]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentOperationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOperationsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentOperation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperationDisplayProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
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
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupExportResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResourceManagementErrorWithDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateHashResult {
    #[serde(rename = "minifiedTemplate", skip_serializing_if = "Option::is_none")]
    pub minified_template: Option<String>,
    #[serde(rename = "templateHash", skip_serializing_if = "Option::is_none")]
    pub template_hash: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}
