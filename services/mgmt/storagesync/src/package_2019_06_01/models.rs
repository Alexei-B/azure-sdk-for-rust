#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innererror: Option<StorageSyncApiError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<StorageSyncErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_state::State>,
    #[serde(skip_serializing)]
    pub istransitioning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionStateProperties>,
}
pub mod subscription_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerChangeDetectionParameters {
    #[serde(rename = "directoryPath", skip_serializing_if = "Option::is_none")]
    pub directory_path: Option<String>,
    #[serde(rename = "changeDetectionMode", skip_serializing_if = "Option::is_none")]
    pub change_detection_mode: Option<trigger_change_detection_parameters::ChangeDetectionMode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}
pub mod trigger_change_detection_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeDetectionMode {
        Default,
        Recursive,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecallActionParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "recallPath", skip_serializing_if = "Option::is_none")]
    pub recall_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceCreateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroupCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroupCreateParametersProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpointCreateParametersProperties {
    #[serde(rename = "storageAccountResourceId", skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "azureFileShareName", skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointCreateParametersProperties {
    #[serde(rename = "serverLocalPath", skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "serverResourceId", skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "offlineDataTransfer", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferShareName", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerRolloverRequest {
    #[serde(rename = "serverCertificate", skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServerCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServerCreateParametersProperties {
    #[serde(rename = "serverCertificate", skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "lastHeartBeat", skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "serverRole", skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesMoveInfo {
    #[serde(rename = "targetResourceGroup", skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayResource {
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
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
pub mod check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.StorageSync/storageSyncServices")]
        Microsoft_StorageSyncStorageSyncServices,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostRestoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "failedFileList", skip_serializing_if = "Option::is_none")]
    pub failed_file_list: Option<String>,
    #[serde(rename = "restoreFileSpec", skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreRestoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "backupMetadataPropertyBag", skip_serializing_if = "Option::is_none")]
    pub backup_metadata_property_bag: Option<String>,
    #[serde(rename = "restoreFileSpec", skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
    #[serde(rename = "pauseWaitForSyncDrainTimePeriodInSeconds", skip_serializing_if = "Option::is_none")]
    pub pause_wait_for_sync_drain_time_period_in_seconds: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupRequest {
    #[serde(rename = "azureFileShare", skip_serializing_if = "Option::is_none")]
    pub azure_file_share: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostBackupResponse {
    #[serde(rename = "backupMetadata", skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<PostBackupResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreFileSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isdir: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageSyncService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroupArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpointArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServerArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegisteredServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionStateProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostBackupResponseProperties {
    #[serde(rename = "cloudEndpointName", skip_serializing)]
    pub cloud_endpoint_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceUpdateProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceProperties {
    #[serde(rename = "storageSyncServiceStatus", skip_serializing)]
    pub storage_sync_service_status: Option<i64>,
    #[serde(rename = "storageSyncServiceUid", skip_serializing)]
    pub storage_sync_service_uid: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowProperties {
    #[serde(rename = "lastStepName", skip_serializing_if = "Option::is_none")]
    pub last_step_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    #[serde(rename = "lastOperationId", skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroupProperties {
    #[serde(rename = "uniqueId", skip_serializing)]
    pub unique_id: Option<String>,
    #[serde(rename = "syncGroupStatus", skip_serializing)]
    pub sync_group_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServerProperties {
    #[serde(rename = "serverCertificate", skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "serverManagementErrorCode", skip_serializing_if = "Option::is_none")]
    pub server_management_error_code: Option<i64>,
    #[serde(rename = "lastHeartBeat", skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "serverRole", skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "storageSyncServiceUid", skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
    #[serde(rename = "lastWorkflowId", skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[serde(rename = "discoveryEndpointUri", skip_serializing_if = "Option::is_none")]
    pub discovery_endpoint_uri: Option<String>,
    #[serde(rename = "resourceLocation", skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "serviceLocation", skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "managementEndpointUri", skip_serializing_if = "Option::is_none")]
    pub management_endpoint_uri: Option<String>,
    #[serde(rename = "monitoringConfiguration", skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpointProperties {
    #[serde(rename = "storageAccountResourceId", skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "azureFileShareName", skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[serde(rename = "partnershipId", skip_serializing_if = "Option::is_none")]
    pub partnership_id: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "backupEnabled", skip_serializing)]
    pub backup_enabled: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointUpdateProperties {
    #[serde(rename = "cloudTiering", skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "offlineDataTransfer", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferShareName", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointProperties {
    #[serde(rename = "serverLocalPath", skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "serverResourceId", skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", skip_serializing)]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", skip_serializing)]
    pub last_operation_name: Option<String>,
    #[serde(rename = "syncStatus", skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<ServerEndpointSyncStatus>,
    #[serde(rename = "offlineDataTransfer", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferStorageAccountResourceId", skip_serializing)]
    pub offline_data_transfer_storage_account_resource_id: Option<String>,
    #[serde(rename = "offlineDataTransferStorageAccountTenantId", skip_serializing)]
    pub offline_data_transfer_storage_account_tenant_id: Option<String>,
    #[serde(rename = "offlineDataTransferShareName", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
    #[serde(rename = "cloudTieringStatus", skip_serializing_if = "Option::is_none")]
    pub cloud_tiering_status: Option<ServerEndpointCloudTieringStatus>,
    #[serde(rename = "recallStatus", skip_serializing_if = "Option::is_none")]
    pub recall_status: Option<ServerEndpointRecallStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointSyncStatus {
    #[serde(rename = "downloadHealth", skip_serializing_if = "Option::is_none")]
    pub download_health: Option<ServerEndpointSyncHealthState>,
    #[serde(rename = "uploadHealth", skip_serializing_if = "Option::is_none")]
    pub upload_health: Option<ServerEndpointSyncHealthState>,
    #[serde(rename = "combinedHealth", skip_serializing_if = "Option::is_none")]
    pub combined_health: Option<ServerEndpointSyncHealthState>,
    #[serde(rename = "syncActivity", skip_serializing_if = "Option::is_none")]
    pub sync_activity: Option<ServerEndpointSyncActivityState>,
    #[serde(rename = "totalPersistentFilesNotSyncingCount", skip_serializing)]
    pub total_persistent_files_not_syncing_count: Option<i64>,
    #[serde(rename = "lastUpdatedTimestamp", skip_serializing)]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "uploadStatus", skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<ServerEndpointSyncSessionStatus>,
    #[serde(rename = "downloadStatus", skip_serializing_if = "Option::is_none")]
    pub download_status: Option<ServerEndpointSyncSessionStatus>,
    #[serde(rename = "uploadActivity", skip_serializing_if = "Option::is_none")]
    pub upload_activity: Option<ServerEndpointSyncActivityStatus>,
    #[serde(rename = "downloadActivity", skip_serializing_if = "Option::is_none")]
    pub download_activity: Option<ServerEndpointSyncActivityStatus>,
    #[serde(rename = "offlineDataTransferStatus", skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_status: Option<ServerEndpointOfflineDataTransferState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointSyncSessionStatus {
    #[serde(rename = "lastSyncResult", skip_serializing)]
    pub last_sync_result: Option<i32>,
    #[serde(rename = "lastSyncTimestamp", skip_serializing)]
    pub last_sync_timestamp: Option<String>,
    #[serde(rename = "lastSyncSuccessTimestamp", skip_serializing)]
    pub last_sync_success_timestamp: Option<String>,
    #[serde(rename = "lastSyncPerItemErrorCount", skip_serializing)]
    pub last_sync_per_item_error_count: Option<i64>,
    #[serde(rename = "persistentFilesNotSyncingCount", skip_serializing)]
    pub persistent_files_not_syncing_count: Option<i64>,
    #[serde(rename = "transientFilesNotSyncingCount", skip_serializing)]
    pub transient_files_not_syncing_count: Option<i64>,
    #[serde(rename = "filesNotSyncingErrors", skip_serializing)]
    pub files_not_syncing_errors: Vec<ServerEndpointFilesNotSyncingError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointSyncActivityStatus {
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(rename = "perItemErrorCount", skip_serializing)]
    pub per_item_error_count: Option<i64>,
    #[serde(rename = "appliedItemCount", skip_serializing)]
    pub applied_item_count: Option<i64>,
    #[serde(rename = "totalItemCount", skip_serializing)]
    pub total_item_count: Option<i64>,
    #[serde(rename = "appliedBytes", skip_serializing)]
    pub applied_bytes: Option<i64>,
    #[serde(rename = "totalBytes", skip_serializing)]
    pub total_bytes: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointFilesNotSyncingError {
    #[serde(rename = "errorCode", skip_serializing)]
    pub error_code: Option<i32>,
    #[serde(rename = "persistentCount", skip_serializing)]
    pub persistent_count: Option<i64>,
    #[serde(rename = "transientCount", skip_serializing)]
    pub transient_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalPath {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsObject {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointSyncHealthState {
    Healthy,
    Error,
    SyncBlockedForRestore,
    SyncBlockedForChangeDetectionPostRestore,
    NoActivity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointSyncActivityState {
    Upload,
    Download,
    UploadAndDownload,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointOfflineDataTransferState {
    InProgress,
    Stopping,
    NotRunning,
    Complete,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "failed")]
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationDirection {
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "cancel")]
    Cancel,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgressType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "recall")]
    Recall,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing)]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointCloudTieringStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<ServerEndpointCloudTieringHealthState>,
    #[serde(rename = "lastUpdatedTimestamp", skip_serializing)]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "lastCloudTieringResult", skip_serializing)]
    pub last_cloud_tiering_result: Option<i32>,
    #[serde(rename = "lastSuccessTimestamp", skip_serializing)]
    pub last_success_timestamp: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointCloudTieringHealthState {
    Healthy,
    Error,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointRecallStatus {
    #[serde(rename = "lastUpdatedTimestamp", skip_serializing)]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "totalRecallErrorsCount", skip_serializing)]
    pub total_recall_errors_count: Option<i64>,
    #[serde(rename = "recallErrors", skip_serializing)]
    pub recall_errors: Vec<ServerEndpointRecallError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointRecallError {
    #[serde(rename = "errorCode", skip_serializing)]
    pub error_code: Option<i32>,
    #[serde(skip_serializing)]
    pub count: Option<i64>,
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
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
