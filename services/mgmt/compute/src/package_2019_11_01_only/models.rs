#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
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
pub struct Disk {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", skip_serializing)]
    pub managed_by: Option<String>,
    #[serde(rename = "managedByExtended", skip_serializing)]
    pub managed_by_extended: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskUpdateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskList {
    pub value: Vec<Disk>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<disk_sku::Name>,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
}
pub mod disk_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(rename = "UltraSSD_LRS")]
        UltraSsdLrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<snapshot_sku::Name>,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
}
pub mod snapshot_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskProperties {
    #[serde(rename = "timeCreated", skip_serializing)]
    pub time_created: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_properties::OsType>,
    #[serde(rename = "hyperVGeneration", skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<disk_properties::HyperVGeneration>,
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[serde(rename = "diskSizeGB", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "diskSizeBytes", skip_serializing)]
    pub disk_size_bytes: Option<i64>,
    #[serde(rename = "uniqueId", skip_serializing)]
    pub unique_id: Option<String>,
    #[serde(rename = "encryptionSettingsCollection", skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "diskIOPSReadWrite", skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[serde(rename = "diskMBpsReadWrite", skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[serde(rename = "diskIOPSReadOnly", skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_only: Option<i64>,
    #[serde(rename = "diskMBpsReadOnly", skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_only: Option<i64>,
    #[serde(rename = "diskState", skip_serializing)]
    pub disk_state: Option<disk_properties::DiskState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "maxShares", skip_serializing_if = "Option::is_none")]
    pub max_shares: Option<i32>,
    #[serde(rename = "shareInfo", skip_serializing)]
    pub share_info: Vec<ShareInfoElement>,
}
pub mod disk_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DiskState {
        Unattached,
        Attached,
        Reserved,
        #[serde(rename = "ActiveSAS")]
        ActiveSas,
        ReadyToUpload,
        ActiveUpload,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[serde(rename = "timeCreated", skip_serializing)]
    pub time_created: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_properties::OsType>,
    #[serde(rename = "hyperVGeneration", skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<snapshot_properties::HyperVGeneration>,
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[serde(rename = "diskSizeGB", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "diskSizeBytes", skip_serializing)]
    pub disk_size_bytes: Option<i64>,
    #[serde(rename = "uniqueId", skip_serializing)]
    pub unique_id: Option<String>,
    #[serde(rename = "encryptionSettingsCollection", skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}
pub mod snapshot_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareInfoElement {
    #[serde(rename = "vmUri", skip_serializing)]
    pub vm_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSetProperties {
    #[serde(rename = "activeKey", skip_serializing_if = "Option::is_none")]
    pub active_key: Option<KeyVaultAndKeyReference>,
    #[serde(rename = "previousKeys", skip_serializing)]
    pub previous_keys: Vec<KeyVaultAndKeyReference>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsCollection {
    pub enabled: bool,
    #[serde(rename = "encryptionSettings", skip_serializing_if = "Vec::is_empty")]
    pub encryption_settings: Vec<EncryptionSettingsElement>,
    #[serde(rename = "encryptionSettingsVersion", skip_serializing_if = "Option::is_none")]
    pub encryption_settings_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsElement {
    #[serde(rename = "diskEncryptionKey", skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultAndSecretReference>,
    #[serde(rename = "keyEncryptionKey", skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultAndKeyReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndSecretReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndKeyReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceVault {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "diskEncryptionSetId", skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<encryption::Type>,
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        EncryptionAtRestWithPlatformKey,
        EncryptionAtRestWithCustomerKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskUpdateProperties {
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_update_properties::OsType>,
    #[serde(rename = "diskSizeGB", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "diskIOPSReadWrite", skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[serde(rename = "diskMBpsReadWrite", skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[serde(rename = "diskIOPSReadOnly", skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_only: Option<i64>,
    #[serde(rename = "diskMBpsReadOnly", skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_only: Option<i64>,
    #[serde(rename = "maxShares", skip_serializing_if = "Option::is_none")]
    pub max_shares: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}
pub mod disk_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotUpdateProperties {
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_update_properties::OsType>,
    #[serde(rename = "diskSizeGB", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}
pub mod snapshot_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetUpdateProperties {
    #[serde(rename = "activeKey", skip_serializing_if = "Option::is_none")]
    pub active_key: Option<KeyVaultAndKeyReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationData {
    #[serde(rename = "createOption")]
    pub create_option: creation_data::CreateOption,
    #[serde(rename = "storageAccountId", skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "imageReference", skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageDiskReference>,
    #[serde(rename = "galleryImageReference", skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<ImageDiskReference>,
    #[serde(rename = "sourceUri", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(rename = "sourceResourceId", skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[serde(rename = "sourceUniqueId", skip_serializing)]
    pub source_unique_id: Option<String>,
    #[serde(rename = "uploadSizeBytes", skip_serializing_if = "Option::is_none")]
    pub upload_size_bytes: Option<i64>,
}
pub mod creation_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateOption {
        Empty,
        Attach,
        FromImage,
        Import,
        Copy,
        Restore,
        Upload,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDiskReference {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantAccessData {
    pub access: grant_access_data::Access,
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i32,
}
pub mod grant_access_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        None,
        Read,
        Write,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessUri {
    #[serde(rename = "accessSAS", skip_serializing)]
    pub access_sas: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", skip_serializing)]
    pub managed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotUpdateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotList {
    pub value: Vec<Snapshot>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSetIdentity {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<encryption_set_identity::Type>,
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
pub mod encryption_set_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionSetIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EncryptionSetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskEncryptionSetUpdateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetList {
    pub value: Vec<DiskEncryptionSet>,
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
