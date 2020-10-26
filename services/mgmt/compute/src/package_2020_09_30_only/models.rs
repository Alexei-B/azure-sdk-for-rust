#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gallery {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<GalleryIdentifier>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<gallery_properties::ProvisioningState>,
    #[serde(rename = "sharingProfile", skip_serializing_if = "Option::is_none")]
    pub sharing_profile: Option<SharingProfile>,
}
pub mod gallery_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryIdentifier {
    #[serde(rename = "uniqueName", skip_serializing)]
    pub unique_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<sharing_profile::Permissions>,
    #[serde(skip_serializing)]
    pub groups: Vec<SharingProfileGroup>,
}
pub mod sharing_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        Private,
        Groups,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingProfileGroup {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<sharing_profile_group::Type>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<String>,
}
pub mod sharing_profile_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Subscriptions,
        #[serde(rename = "AADTenants")]
        AadTenants,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplication {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eula: Option<String>,
    #[serde(rename = "privacyStatementUri", skip_serializing_if = "Option::is_none")]
    pub privacy_statement_uri: Option<String>,
    #[serde(rename = "releaseNoteUri", skip_serializing_if = "Option::is_none")]
    pub release_note_uri: Option<String>,
    #[serde(rename = "endOfLifeDate", skip_serializing_if = "Option::is_none")]
    pub end_of_life_date: Option<String>,
    #[serde(rename = "supportedOSType")]
    pub supported_os_type: gallery_application_properties::SupportedOsType,
}
pub mod gallery_application_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SupportedOsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersion {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationVersionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationVersionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionProperties {
    #[serde(rename = "publishingProfile")]
    pub publishing_profile: GalleryApplicationVersionPublishingProfile,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<gallery_application_version_properties::ProvisioningState>,
    #[serde(rename = "replicationStatus", skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatus>,
}
pub mod gallery_application_version_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionPublishingProfile {
    #[serde(flatten)]
    pub gallery_artifact_publishing_profile_base: GalleryArtifactPublishingProfileBase,
    pub source: UserArtifactSource,
    #[serde(rename = "manageActions", skip_serializing_if = "Option::is_none")]
    pub manage_actions: Option<UserArtifactManage>,
    #[serde(rename = "enableHealthCheck", skip_serializing_if = "Option::is_none")]
    pub enable_health_check: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserArtifactSource {
    #[serde(rename = "mediaLink")]
    pub media_link: String,
    #[serde(rename = "defaultConfigurationLink", skip_serializing_if = "Option::is_none")]
    pub default_configuration_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserArtifactManage {
    pub install: String,
    pub remove: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eula: Option<String>,
    #[serde(rename = "privacyStatementUri", skip_serializing_if = "Option::is_none")]
    pub privacy_statement_uri: Option<String>,
    #[serde(rename = "releaseNoteUri", skip_serializing_if = "Option::is_none")]
    pub release_note_uri: Option<String>,
    #[serde(rename = "osType")]
    pub os_type: gallery_image_properties::OsType,
    #[serde(rename = "osState")]
    pub os_state: gallery_image_properties::OsState,
    #[serde(rename = "hyperVGeneration", skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<gallery_image_properties::HyperVGeneration>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GalleryImageFeature>,
    #[serde(rename = "endOfLifeDate", skip_serializing_if = "Option::is_none")]
    pub end_of_life_date: Option<String>,
    pub identifier: GalleryImageIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedMachineConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed: Option<Disallowed>,
    #[serde(rename = "purchasePlan", skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<ImagePurchasePlan>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<gallery_image_properties::ProvisioningState>,
}
pub mod gallery_image_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsState {
        Generalized,
        Specialized,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageFeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageIdentifier {
    pub publisher: String,
    pub offer: String,
    pub sku: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedMachineConfiguration {
    #[serde(rename = "vCPUs", skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<ResourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<ResourceRange>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disallowed {
    #[serde(rename = "diskTypes", skip_serializing_if = "Vec::is_empty")]
    pub disk_types: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImagePurchasePlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersion {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageVersionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageVersionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionProperties {
    #[serde(rename = "publishingProfile", skip_serializing_if = "Option::is_none")]
    pub publishing_profile: Option<GalleryImageVersionPublishingProfile>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<gallery_image_version_properties::ProvisioningState>,
    #[serde(rename = "storageProfile")]
    pub storage_profile: GalleryImageVersionStorageProfile,
    #[serde(rename = "replicationStatus", skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatus>,
}
pub mod gallery_image_version_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryArtifactPublishingProfileBase {
    #[serde(rename = "targetRegions", skip_serializing_if = "Vec::is_empty")]
    pub target_regions: Vec<TargetRegion>,
    #[serde(rename = "replicaCount", skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[serde(rename = "excludeFromLatest", skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[serde(rename = "publishedDate", skip_serializing)]
    pub published_date: Option<String>,
    #[serde(rename = "endOfLifeDate", skip_serializing_if = "Option::is_none")]
    pub end_of_life_date: Option<String>,
    #[serde(rename = "storageAccountType", skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<gallery_artifact_publishing_profile_base::StorageAccountType>,
}
pub mod gallery_artifact_publishing_profile_base {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetRegion {
    pub name: String,
    #[serde(rename = "regionalReplicaCount", skip_serializing_if = "Option::is_none")]
    pub regional_replica_count: Option<i32>,
    #[serde(rename = "storageAccountType", skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<target_region::StorageAccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionImages>,
}
pub mod target_region {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionImages {
    #[serde(rename = "osDiskImage", skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<OsDiskImageEncryption>,
    #[serde(rename = "dataDiskImages", skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<DataDiskImageEncryption>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsDiskImageEncryption {
    #[serde(flatten)]
    pub disk_image_encryption: DiskImageEncryption,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDiskImageEncryption {
    #[serde(flatten)]
    pub disk_image_encryption: DiskImageEncryption,
    pub lun: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskImageEncryption {
    #[serde(rename = "diskEncryptionSetId", skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryArtifactSource {
    #[serde(rename = "managedImage")]
    pub managed_image: ManagedArtifact,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedArtifact {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionPublishingProfile {
    #[serde(flatten)]
    pub gallery_artifact_publishing_profile_base: GalleryArtifactPublishingProfileBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionStorageProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<GalleryArtifactVersionSource>,
    #[serde(rename = "osDiskImage", skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<GalleryOsDiskImage>,
    #[serde(rename = "dataDiskImages", skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<GalleryDataDiskImage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryArtifactVersionSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryOsDiskImage {
    #[serde(flatten)]
    pub gallery_disk_image: GalleryDiskImage,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryDataDiskImage {
    #[serde(flatten)]
    pub gallery_disk_image: GalleryDiskImage,
    pub lun: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryDiskImage {
    #[serde(rename = "sizeInGB", skip_serializing)]
    pub size_in_gb: Option<i32>,
    #[serde(rename = "hostCaching", skip_serializing_if = "Option::is_none")]
    pub host_caching: Option<gallery_disk_image::HostCaching>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<GalleryArtifactVersionSource>,
}
pub mod gallery_disk_image {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostCaching {
        None,
        ReadOnly,
        ReadWrite,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationStatus {
    #[serde(rename = "aggregatedState", skip_serializing)]
    pub aggregated_state: Option<replication_status::AggregatedState>,
    #[serde(skip_serializing)]
    pub summary: Vec<RegionalReplicationStatus>,
}
pub mod replication_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AggregatedState {
        Unknown,
        InProgress,
        Completed,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionalReplicationStatus {
    #[serde(skip_serializing)]
    pub region: Option<String>,
    #[serde(skip_serializing)]
    pub state: Option<regional_replication_status::State>,
    #[serde(skip_serializing)]
    pub details: Option<String>,
    #[serde(skip_serializing)]
    pub progress: Option<i32>,
}
pub mod regional_replication_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Unknown,
        Replicating,
        Completed,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryList {
    pub value: Vec<Gallery>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageList {
    pub value: Vec<GalleryImage>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionList {
    pub value: Vec<GalleryImageVersion>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationList {
    pub value: Vec<GalleryApplication>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionList {
    pub value: Vec<GalleryApplicationVersion>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ApiErrorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptiontype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errordetail: Option<String>,
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
pub struct UpdateResourceDefinition {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingUpdate {
    #[serde(rename = "operationType")]
    pub operation_type: sharing_update::OperationType,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<SharingProfileGroup>,
}
pub mod sharing_update {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationType {
        Add,
        Remove,
        Reset,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PirResource {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PirSharedGalleryResource {
    #[serde(flatten)]
    pub pir_resource: PirResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<SharedGalleryIdentifier>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryIdentifier {
    #[serde(rename = "uniqueId", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryList {
    pub value: Vec<SharedGallery>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGallery {
    #[serde(flatten)]
    pub pir_shared_gallery_resource: PirSharedGalleryResource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageList {
    pub value: Vec<SharedGalleryImage>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImage {
    #[serde(flatten)]
    pub pir_shared_gallery_resource: PirSharedGalleryResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedGalleryImageProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageProperties {
    #[serde(rename = "osType")]
    pub os_type: shared_gallery_image_properties::OsType,
    #[serde(rename = "osState")]
    pub os_state: shared_gallery_image_properties::OsState,
    #[serde(rename = "endOfLifeDate", skip_serializing_if = "Option::is_none")]
    pub end_of_life_date: Option<String>,
    pub identifier: GalleryImageIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedMachineConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed: Option<Disallowed>,
    #[serde(rename = "hyperVGeneration", skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<shared_gallery_image_properties::HyperVGeneration>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GalleryImageFeature>,
    #[serde(rename = "purchasePlan", skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<ImagePurchasePlan>,
}
pub mod shared_gallery_image_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsState {
        Generalized,
        Specialized,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageVersionList {
    pub value: Vec<SharedGalleryImageVersion>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageVersion {
    #[serde(flatten)]
    pub pir_shared_gallery_resource: PirSharedGalleryResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedGalleryImageVersionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageVersionProperties {
    #[serde(rename = "publishedDate", skip_serializing_if = "Option::is_none")]
    pub published_date: Option<String>,
    #[serde(rename = "endOfLifeDate", skip_serializing_if = "Option::is_none")]
    pub end_of_life_date: Option<String>,
}
