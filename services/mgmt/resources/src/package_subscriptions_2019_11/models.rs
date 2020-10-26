#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedRegion {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationMetadata {
    #[serde(rename = "regionType", skip_serializing)]
    pub region_type: Option<location_metadata::RegionType>,
    #[serde(rename = "regionCategory", skip_serializing)]
    pub region_category: Option<location_metadata::RegionCategory>,
    #[serde(rename = "geographyGroup", skip_serializing)]
    pub geography_group: Option<String>,
    #[serde(skip_serializing)]
    pub longitude: Option<String>,
    #[serde(skip_serializing)]
    pub latitude: Option<String>,
    #[serde(rename = "physicalLocation", skip_serializing)]
    pub physical_location: Option<String>,
    #[serde(rename = "pairedRegion", skip_serializing_if = "Vec::is_empty")]
    pub paired_region: Vec<PairedRegion>,
}
pub mod location_metadata {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RegionType {
        Physical,
        Logical,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RegionCategory {
        Recommended,
        Other,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "regionalDisplayName", skip_serializing)]
    pub regional_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LocationMetadata>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing)]
    pub state: Option<subscription::State>,
    #[serde(rename = "subscriptionPolicies", skip_serializing_if = "Option::is_none")]
    pub subscription_policies: Option<SubscriptionPolicies>,
    #[serde(rename = "authorizationSource", skip_serializing_if = "Option::is_none")]
    pub authorization_source: Option<String>,
    #[serde(rename = "managedByTenants", skip_serializing_if = "Vec::is_empty")]
    pub managed_by_tenants: Vec<ManagedByTenant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod subscription {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Warned,
        PastDue,
        Disabled,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionPolicies {
    #[serde(rename = "locationPlacementId", skip_serializing)]
    pub location_placement_id: Option<String>,
    #[serde(rename = "quotaId", skip_serializing)]
    pub quota_id: Option<String>,
    #[serde(rename = "spendingLimit", skip_serializing)]
    pub spending_limit: Option<subscription_policies::SpendingLimit>,
}
pub mod subscription_policies {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SpendingLimit {
        On,
        Off,
        CurrentPeriodOff,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedByTenant {
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subscription>,
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantIdDescription {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "tenantCategory", skip_serializing)]
    pub tenant_category: Option<tenant_id_description::TenantCategory>,
    #[serde(skip_serializing)]
    pub country: Option<String>,
    #[serde(rename = "countryCode", skip_serializing)]
    pub country_code: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(skip_serializing)]
    pub domains: Vec<String>,
}
pub mod tenant_id_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TenantCategory {
        Home,
        ProjectedBy,
        ManagedBy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TenantIdDescription>,
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckResourceNameResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<check_resource_name_result::Status>,
}
pub mod check_resource_name_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Allowed,
        Reserved,
    }
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
