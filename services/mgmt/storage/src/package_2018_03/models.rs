#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
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
    pub properties: Option<OperationProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "fillGapWithZero", skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "resourceIdDimensionNameOverride", skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: storage_account_check_name_availability_parameters::Type,
}
pub mod storage_account_check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Storage/storageAccounts")]
        Microsoft_StorageStorageAccounts,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapability {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Restriction {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<restriction::ReasonCode>,
}
pub mod restriction {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSkuListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Sku>,
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
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(skip_serializing)]
    pub tier: Option<sku::Tier>,
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub kind: Option<sku::Kind>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(skip_serializing)]
    pub capabilities: Vec<SkuCapability>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    pub name: String,
    #[serde(rename = "useSubDomainName", skip_serializing_if = "Option::is_none")]
    pub use_sub_domain_name: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionService {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastEnabledTime", skip_serializing)]
    pub last_enabled_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionServices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<EncryptionService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<EncryptionService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<EncryptionService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<EncryptionService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyvaulturi: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<EncryptionServices>,
    #[serde(rename = "keySource")]
    pub key_source: encryption::KeySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyvaultproperties: Option<KeyVaultProperties>,
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Storage")]
        Microsoft_Storage,
        #[serde(rename = "Microsoft.Keyvault")]
        Microsoft_Keyvault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<virtual_network_rule::Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<virtual_network_rule::State>,
}
pub mod virtual_network_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "deprovisioning")]
        Deprovisioning,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "networkSourceDeleted")]
        NetworkSourceDeleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ip_rule::Action>,
}
pub mod ip_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass: Option<network_rule_set::Bypass>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[serde(rename = "ipRules", skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[serde(rename = "defaultAction")]
    pub default_action: network_rule_set::DefaultAction,
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Bypass {
        None,
        Logging,
        Metrics,
        AzureServices,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultAction {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPropertiesCreateParameters {
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[serde(rename = "accessTier", skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_create_parameters::AccessTier>,
    #[serde(rename = "supportsHttpsTrafficOnly", skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,
    #[serde(rename = "isHnsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_hns_enabled: Option<bool>,
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    pub sku: Sku,
    pub kind: storage_account_create_parameters::Kind,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
pub mod storage_account_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoints {
    #[serde(skip_serializing)]
    pub blob: Option<String>,
    #[serde(skip_serializing)]
    pub queue: Option<String>,
    #[serde(skip_serializing)]
    pub table: Option<String>,
    #[serde(skip_serializing)]
    pub file: Option<String>,
    #[serde(skip_serializing)]
    pub web: Option<String>,
    #[serde(skip_serializing)]
    pub dfs: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<storage_account_properties::ProvisioningState>,
    #[serde(rename = "primaryEndpoints", skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<Endpoints>,
    #[serde(rename = "primaryLocation", skip_serializing)]
    pub primary_location: Option<String>,
    #[serde(rename = "statusOfPrimary", skip_serializing)]
    pub status_of_primary: Option<storage_account_properties::StatusOfPrimary>,
    #[serde(rename = "lastGeoFailoverTime", skip_serializing)]
    pub last_geo_failover_time: Option<String>,
    #[serde(rename = "secondaryLocation", skip_serializing)]
    pub secondary_location: Option<String>,
    #[serde(rename = "statusOfSecondary", skip_serializing)]
    pub status_of_secondary: Option<storage_account_properties::StatusOfSecondary>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(rename = "secondaryEndpoints", skip_serializing_if = "Option::is_none")]
    pub secondary_endpoints: Option<Endpoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", skip_serializing)]
    pub access_tier: Option<storage_account_properties::AccessTier>,
    #[serde(rename = "supportsHttpsTrafficOnly", skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,
    #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[serde(rename = "isHnsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_hns_enabled: Option<bool>,
}
pub mod storage_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfPrimary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfSecondary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing)]
    pub kind: Option<storage_account::Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
pub mod storage_account {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountKey {
    #[serde(rename = "keyName", skip_serializing)]
    pub key_name: Option<String>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(skip_serializing)]
    pub permissions: Option<storage_account_key::Permissions>,
}
pub mod storage_account_key {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        Read,
        Full,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountListResult {
    #[serde(skip_serializing)]
    pub value: Vec<StorageAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountListKeysResult {
    #[serde(skip_serializing)]
    pub keys: Vec<StorageAccountKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountRegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_update_parameters::AccessTier>,
    #[serde(rename = "supportsHttpsTrafficOnly", skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,
    #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
}
pub mod storage_account_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesUpdateParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<storage_account_update_parameters::Kind>,
}
pub mod storage_account_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<usage::Unit>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i32>,
    #[serde(skip_serializing)]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSasParameters {
    #[serde(rename = "signedServices")]
    pub signed_services: account_sas_parameters::SignedServices,
    #[serde(rename = "signedResourceTypes")]
    pub signed_resource_types: account_sas_parameters::SignedResourceTypes,
    #[serde(rename = "signedPermission")]
    pub signed_permission: account_sas_parameters::SignedPermission,
    #[serde(rename = "signedIp", skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[serde(rename = "signedProtocol", skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<account_sas_parameters::SignedProtocol>,
    #[serde(rename = "signedStart", skip_serializing_if = "Option::is_none")]
    pub signed_start: Option<String>,
    #[serde(rename = "signedExpiry")]
    pub signed_expiry: String,
    #[serde(rename = "keyToSign", skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
}
pub mod account_sas_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedServices {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "q")]
        Q,
        #[serde(rename = "t")]
        T,
        #[serde(rename = "f")]
        F,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedResourceTypes {
        #[serde(rename = "s")]
        S,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "o")]
        O,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAccountSasResponse {
    #[serde(rename = "accountSasToken", skip_serializing)]
    pub account_sas_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSasParameters {
    #[serde(rename = "canonicalizedResource")]
    pub canonicalized_resource: String,
    #[serde(rename = "signedResource", skip_serializing_if = "Option::is_none")]
    pub signed_resource: Option<service_sas_parameters::SignedResource>,
    #[serde(rename = "signedPermission", skip_serializing_if = "Option::is_none")]
    pub signed_permission: Option<service_sas_parameters::SignedPermission>,
    #[serde(rename = "signedIp", skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[serde(rename = "signedProtocol", skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<service_sas_parameters::SignedProtocol>,
    #[serde(rename = "signedStart", skip_serializing_if = "Option::is_none")]
    pub signed_start: Option<String>,
    #[serde(rename = "signedExpiry", skip_serializing_if = "Option::is_none")]
    pub signed_expiry: Option<String>,
    #[serde(rename = "signedIdentifier", skip_serializing_if = "Option::is_none")]
    pub signed_identifier: Option<String>,
    #[serde(rename = "startPk", skip_serializing_if = "Option::is_none")]
    pub start_pk: Option<String>,
    #[serde(rename = "endPk", skip_serializing_if = "Option::is_none")]
    pub end_pk: Option<String>,
    #[serde(rename = "startRk", skip_serializing_if = "Option::is_none")]
    pub start_rk: Option<String>,
    #[serde(rename = "endRk", skip_serializing_if = "Option::is_none")]
    pub end_rk: Option<String>,
    #[serde(rename = "keyToSign", skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsct: Option<String>,
}
pub mod service_sas_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedResource {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "f")]
        F,
        #[serde(rename = "s")]
        S,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListServiceSasResponse {
    #[serde(rename = "serviceSasToken", skip_serializing)]
    pub service_sas_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountManagementPolicies {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountManagementPoliciesRulesProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountManagementPoliciesRulesProperty {
    #[serde(flatten)]
    pub management_policies_rules: ManagementPoliciesRules,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPoliciesRulesSetParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementPoliciesRules>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPoliciesRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    #[serde(rename = "publicAccess", skip_serializing_if = "Option::is_none")]
    pub public_access: Option<container_properties::PublicAccess>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(rename = "leaseStatus", skip_serializing)]
    pub lease_status: Option<container_properties::LeaseStatus>,
    #[serde(rename = "leaseState", skip_serializing)]
    pub lease_state: Option<container_properties::LeaseState>,
    #[serde(rename = "leaseDuration", skip_serializing)]
    pub lease_duration: Option<container_properties::LeaseDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "immutabilityPolicy", skip_serializing_if = "Option::is_none")]
    pub immutability_policy: Option<ImmutabilityPolicyProperties>,
    #[serde(rename = "legalHold", skip_serializing_if = "Option::is_none")]
    pub legal_hold: Option<LegalHoldProperties>,
    #[serde(rename = "hasLegalHold", skip_serializing)]
    pub has_legal_hold: Option<bool>,
    #[serde(rename = "hasImmutabilityPolicy", skip_serializing)]
    pub has_immutability_policy: Option<bool>,
}
pub mod container_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicAccess {
        Container,
        Blob,
        None,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LeaseStatus {
        Locked,
        Unlocked,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LeaseState {
        Available,
        Leased,
        Expired,
        Breaking,
        Broken,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LeaseDuration {
        Infinite,
        Fixed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainer {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImmutabilityPolicyProperty {
    #[serde(rename = "immutabilityPeriodSinceCreationInDays")]
    pub immutability_period_since_creation_in_days: i64,
    #[serde(skip_serializing)]
    pub state: Option<immutability_policy_property::State>,
}
pub mod immutability_policy_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Locked,
        Unlocked,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImmutabilityPolicyProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImmutabilityPolicyProperty>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(rename = "updateHistory", skip_serializing)]
    pub update_history: Vec<UpdateHistoryProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImmutabilityPolicy {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    pub properties: ImmutabilityPolicyProperty,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateHistoryProperty {
    #[serde(skip_serializing)]
    pub update: Option<update_history_property::Update>,
    #[serde(rename = "immutabilityPeriodSinceCreationInDays", skip_serializing)]
    pub immutability_period_since_creation_in_days: Option<i64>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(rename = "objectIdentifier", skip_serializing)]
    pub object_identifier: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing)]
    pub upn: Option<String>,
}
pub mod update_history_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Update {
        #[serde(rename = "put")]
        Put,
        #[serde(rename = "lock")]
        Lock,
        #[serde(rename = "extend")]
        Extend,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegalHoldProperties {
    #[serde(rename = "hasLegalHold", skip_serializing)]
    pub has_legal_hold: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagProperty {
    #[serde(skip_serializing)]
    pub tag: Option<String>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(rename = "objectIdentifier", skip_serializing)]
    pub object_identifier: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing)]
    pub upn: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegalHold {
    #[serde(rename = "hasLegalHold", skip_serializing)]
    pub has_legal_hold: Option<bool>,
    pub tags: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListContainerItem {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListContainerItems {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ListContainerItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaseContainerRequest {
    pub action: lease_container_request::Action,
    #[serde(rename = "leaseId", skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[serde(rename = "breakPeriod", skip_serializing_if = "Option::is_none")]
    pub break_period: Option<i64>,
    #[serde(rename = "leaseDuration", skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<i64>,
    #[serde(rename = "proposedLeaseId", skip_serializing_if = "Option::is_none")]
    pub proposed_lease_id: Option<String>,
}
pub mod lease_container_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Acquire,
        Renew,
        Change,
        Release,
        Break,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaseContainerResponse {
    #[serde(rename = "leaseId", skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[serde(rename = "leaseTimeSeconds", skip_serializing_if = "Option::is_none")]
    pub lease_time_seconds: Option<String>,
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
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
