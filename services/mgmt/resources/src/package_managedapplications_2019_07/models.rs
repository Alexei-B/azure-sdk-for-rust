#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplicationProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPatchable {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanPatchable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinition {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplicationDefinitionProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename = "managedResourceGroupId", skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[serde(rename = "applicationDefinitionId", skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "billingDetails", skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<ApplicationBillingDetailsDefinition>,
    #[serde(rename = "jitAccessPolicy", skip_serializing_if = "Option::is_none")]
    pub jit_access_policy: Option<ApplicationJitAccessPolicy>,
    #[serde(rename = "publisherTenantId", skip_serializing)]
    pub publisher_tenant_id: Option<String>,
    #[serde(skip_serializing)]
    pub authorizations: Vec<ApplicationAuthorization>,
    #[serde(rename = "managementMode", skip_serializing_if = "Option::is_none")]
    pub management_mode: Option<ApplicationManagementMode>,
    #[serde(rename = "customerSupport", skip_serializing_if = "Option::is_none")]
    pub customer_support: Option<ApplicationPackageContact>,
    #[serde(rename = "supportUrls", skip_serializing_if = "Option::is_none")]
    pub support_urls: Option<ApplicationPackageSupportUrls>,
    #[serde(skip_serializing)]
    pub artifacts: Vec<ApplicationArtifact>,
    #[serde(rename = "createdBy", skip_serializing)]
    pub created_by: Option<ApplicationClientDetails>,
    #[serde(rename = "updatedBy", skip_serializing)]
    pub updated_by: Option<ApplicationClientDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPropertiesPatchable {
    #[serde(rename = "managedResourceGroupId", skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[serde(rename = "applicationDefinitionId", skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionProperties {
    #[serde(rename = "lockLevel")]
    pub lock_level: ApplicationLockLevel,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<ApplicationAuthorization>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationDefinitionArtifact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageFileUri", skip_serializing_if = "Option::is_none")]
    pub package_file_uri: Option<String>,
    #[serde(rename = "mainTemplate", skip_serializing_if = "Option::is_none")]
    pub main_template: Option<serde_json::Value>,
    #[serde(rename = "createUiDefinition", skip_serializing_if = "Option::is_none")]
    pub create_ui_definition: Option<serde_json::Value>,
    #[serde(rename = "notificationPolicy", skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<ApplicationNotificationPolicy>,
    #[serde(rename = "lockingPolicy", skip_serializing_if = "Option::is_none")]
    pub locking_policy: Option<ApplicationPackageLockingPolicyDefinition>,
    #[serde(rename = "deploymentPolicy", skip_serializing_if = "Option::is_none")]
    pub deployment_policy: Option<ApplicationDeploymentPolicy>,
    #[serde(rename = "managementPolicy", skip_serializing_if = "Option::is_none")]
    pub management_policy: Option<ApplicationManagementPolicy>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<ApplicationPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    #[serde(rename = "promotionCode", skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    pub version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanPatchable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "promotionCode", skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
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
    #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedResourceIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
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
pub struct ApplicationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationDefinition>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationLockLevel {
    CanNotDelete,
    ReadOnly,
    None,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactType {
    NotSpecified,
    Template,
    Custom,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationDefinitionArtifactName {
    NotSpecified,
    ApplicationResourceTemplate,
    CreateUiDefinition,
    MainTemplateParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactName {
    NotSpecified,
    ViewDefinition,
    Authorizations,
    CustomRoleDefinition,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationArtifact {
    pub name: ApplicationArtifactName,
    pub uri: String,
    #[serde(rename = "type")]
    pub type_: ApplicationArtifactType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionArtifact {
    pub name: ApplicationDefinitionArtifactName,
    pub uri: String,
    #[serde(rename = "type")]
    pub type_: ApplicationArtifactType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNotificationPolicy {
    #[serde(rename = "notificationEndpoints")]
    pub notification_endpoints: Vec<ApplicationNotificationEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNotificationEndpoint {
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageLockingPolicyDefinition {
    #[serde(rename = "allowedActions", skip_serializing_if = "Vec::is_empty")]
    pub allowed_actions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDeploymentPolicy {
    #[serde(rename = "deploymentMode")]
    pub deployment_mode: DeploymentMode,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationManagementPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ApplicationManagementMode>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyDefinitionId", skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationAuthorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageContact {
    #[serde(rename = "contactName", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    pub email: String,
    pub phone: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageSupportUrls {
    #[serde(rename = "publicAzure", skip_serializing_if = "Option::is_none")]
    pub public_azure: Option<String>,
    #[serde(rename = "governmentCloud", skip_serializing_if = "Option::is_none")]
    pub government_cloud: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationBillingDetailsDefinition {
    #[serde(rename = "resourceUsageId", skip_serializing_if = "Option::is_none")]
    pub resource_usage_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationJitAccessPolicy {
    #[serde(rename = "jitAccessEnabled")]
    pub jit_access_enabled: bool,
    #[serde(rename = "jitApprovalMode", skip_serializing_if = "Option::is_none")]
    pub jit_approval_mode: Option<JitApprovalMode>,
    #[serde(rename = "jitApprovers", skip_serializing_if = "Vec::is_empty")]
    pub jit_approvers: Vec<JitApproverDefinition>,
    #[serde(rename = "maximumJitAccessDuration", skip_serializing_if = "Option::is_none")]
    pub maximum_jit_access_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "httpStatus", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JitRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestPatchable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestDefinitionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JitRequestDefinition>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestProperties {
    #[serde(rename = "applicationResourceId")]
    pub application_resource_id: String,
    #[serde(rename = "publisherTenantId", skip_serializing)]
    pub publisher_tenant_id: Option<String>,
    #[serde(rename = "jitAuthorizationPolicies")]
    pub jit_authorization_policies: Vec<JitAuthorizationPolicies>,
    #[serde(rename = "jitSchedulingPolicy")]
    pub jit_scheduling_policy: JitSchedulingPolicy,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "jitRequestState", skip_serializing)]
    pub jit_request_state: Option<JitRequestState>,
    #[serde(rename = "createdBy", skip_serializing)]
    pub created_by: Option<ApplicationClientDetails>,
    #[serde(rename = "updatedBy", skip_serializing)]
    pub updated_by: Option<ApplicationClientDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitAuthorizationPolicies {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitSchedulingPolicy {
    #[serde(rename = "type", skip_serializing)]
    pub type_: JitSchedulingType,
    pub duration: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitApprovalMode {
    NotSpecified,
    AutoApprove,
    ManualApprove,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitApproverDefinition {
    pub id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<jit_approver_definition::Type>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
pub mod jit_approver_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "group")]
        Group,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationClientDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puid: Option<String>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitRequestState {
    NotSpecified,
    Pending,
    Approved,
    Denied,
    Failed,
    Canceled,
    Expired,
    Timeout,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitSchedulingType {
    NotSpecified,
    Once,
    Recurring,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeploymentMode {
    NotSpecified,
    Incremental,
    Complete,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationManagementMode {
    NotSpecified,
    Unmanaged,
    Managed,
}
