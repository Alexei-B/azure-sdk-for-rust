#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ImageTemplate>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSource {
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateIsoSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[serde(rename = "sourceUri")]
    pub source_uri: String,
    #[serde(rename = "sha256Checksum")]
    pub sha256_checksum: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplatePlatformImageSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateManagedImageSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[serde(rename = "imageId")]
    pub image_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSharedImageVersionSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[serde(rename = "imageVersionId")]
    pub image_version_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateCustomizer {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateShellCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[serde(rename = "scriptUri", skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[serde(rename = "sha256Checksum", skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inline: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateRestartCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[serde(rename = "restartCommand", skip_serializing_if = "Option::is_none")]
    pub restart_command: Option<String>,
    #[serde(rename = "restartCheckCommand", skip_serializing_if = "Option::is_none")]
    pub restart_check_command: Option<String>,
    #[serde(rename = "restartTimeout", skip_serializing_if = "Option::is_none")]
    pub restart_timeout: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplatePowerShellCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[serde(rename = "scriptUri", skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[serde(rename = "sha256Checksum", skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inline: Vec<String>,
    #[serde(rename = "runElevated", skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[serde(rename = "validExitCodes", skip_serializing_if = "Vec::is_empty")]
    pub valid_exit_codes: Vec<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateFileCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[serde(rename = "sourceUri", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(rename = "sha256Checksum", skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateDistributor {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "runOutputName")]
    pub run_output_name: String,
    #[serde(rename = "artifactTags", skip_serializing_if = "Option::is_none")]
    pub artifact_tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateManagedImageDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
    #[serde(rename = "imageId")]
    pub image_id: String,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSharedImageDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
    #[serde(rename = "galleryImageId")]
    pub gallery_image_id: String,
    #[serde(rename = "replicationRegions")]
    pub replication_regions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateVhdDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateVmProfile {
    #[serde(rename = "vmSize", skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Updating,
    Succeeded,
    Failed,
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisioningError {
    #[serde(rename = "provisioningErrorCode", skip_serializing_if = "Option::is_none")]
    pub provisioning_error_code: Option<provisioning_error::ProvisioningErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod provisioning_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningErrorCode {
        BadSourceType,
        #[serde(rename = "BadPIRSource")]
        BadPirSource,
        #[serde(rename = "BadISOSource")]
        BadIsoSource,
        BadManagedImageSource,
        BadSharedImageVersionSource,
        BadCustomizerType,
        UnsupportedCustomizerType,
        NoCustomizerScript,
        BadDistributeType,
        BadSharedImageDistribute,
        ServerError,
        Other,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateLastRunStatus {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "runState", skip_serializing_if = "Option::is_none")]
    pub run_state: Option<image_template_last_run_status::RunState>,
    #[serde(rename = "runSubState", skip_serializing_if = "Option::is_none")]
    pub run_sub_state: Option<image_template_last_run_status::RunSubState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod image_template_last_run_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunState {
        Running,
        Succeeded,
        PartiallySucceeded,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunSubState {
        Queued,
        Building,
        Customizing,
        Distributing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateProperties {
    pub source: ImageTemplateSource,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customize: Vec<ImageTemplateCustomizer>,
    pub distribute: Vec<ImageTemplateDistributor>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "provisioningError", skip_serializing_if = "Option::is_none")]
    pub provisioning_error: Option<ProvisioningError>,
    #[serde(rename = "lastRunStatus", skip_serializing_if = "Option::is_none")]
    pub last_run_status: Option<ImageTemplateLastRunStatus>,
    #[serde(rename = "buildTimeoutInMinutes", skip_serializing_if = "Option::is_none")]
    pub build_timeout_in_minutes: Option<i64>,
    #[serde(rename = "vmProfile", skip_serializing_if = "Option::is_none")]
    pub vm_profile: Option<ImageTemplateVmProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateIdentity {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<image_template_identity::Type>,
    #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod image_template_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        UserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunOutputProperties {
    #[serde(rename = "artifactId", skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[serde(rename = "artifactUri", skip_serializing_if = "Option::is_none")]
    pub artifact_uri: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageTemplateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ImageTemplateIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ImageTemplateIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunOutput {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunOutputProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunOutputCollection {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RunOutput>,
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
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
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
pub struct ApiErrorBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(rename = "exceptionType", skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<String>,
    #[serde(rename = "errorDetail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ApiErrorBase>,
    #[serde(rename = "innerError", skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
