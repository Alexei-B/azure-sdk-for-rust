#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trial {
    #[serde(skip_serializing)]
    pub status: Option<trial::Status>,
    #[serde(rename = "availableHosts", skip_serializing)]
    pub available_hosts: Option<i64>,
}
pub mod trial {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        TrialAvailable,
        TrialUsed,
        TrialDisabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    #[serde(rename = "hostsRemaining", skip_serializing)]
    pub hosts_remaining: Option<serde_json::Value>,
    #[serde(rename = "quotaEnabled", skip_serializing_if = "Option::is_none")]
    pub quota_enabled: Option<quota::QuotaEnabled>,
}
pub mod quota {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QuotaEnabled {
        Enabled,
        Disabled,
    }
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
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiErrorBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
        #[serde(skip_serializing)]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressRouteAuthorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circuit {
    #[serde(rename = "primarySubnet", skip_serializing)]
    pub primary_subnet: Option<String>,
    #[serde(rename = "secondarySubnet", skip_serializing)]
    pub secondary_subnet: Option<String>,
    #[serde(rename = "expressRouteID", skip_serializing)]
    pub express_route_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<ExpressRouteAuthorization>,
    #[serde(rename = "expressRoutePrivatePeeringID", skip_serializing)]
    pub express_route_private_peering_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoints {
    #[serde(rename = "nsxtManager", skip_serializing)]
    pub nsxt_manager: Option<String>,
    #[serde(skip_serializing)]
    pub vcsa: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitySource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "baseUserDN", skip_serializing_if = "Option::is_none")]
    pub base_user_dn: Option<String>,
    #[serde(rename = "baseGroupDN", skip_serializing_if = "Option::is_none")]
    pub base_group_dn: Option<String>,
    #[serde(rename = "primaryServer", skip_serializing_if = "Option::is_none")]
    pub primary_server: Option<String>,
    #[serde(rename = "secondaryServer", skip_serializing_if = "Option::is_none")]
    pub secondary_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<identity_source::Ssl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
pub mod identity_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Ssl {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloud {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<private_cloud_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit: Option<Circuit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<DefaultClusterProperties>,
    #[serde(skip_serializing)]
    pub clusters: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Endpoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet: Option<private_cloud_properties::Internet>,
    #[serde(rename = "identitySources", skip_serializing_if = "Vec::is_empty")]
    pub identity_sources: Vec<IdentitySource>,
    #[serde(rename = "networkBlock", skip_serializing_if = "Option::is_none")]
    pub network_block: Option<String>,
    #[serde(rename = "managementNetwork", skip_serializing)]
    pub management_network: Option<String>,
    #[serde(rename = "provisioningNetwork", skip_serializing)]
    pub provisioning_network: Option<String>,
    #[serde(rename = "vmotionNetwork", skip_serializing)]
    pub vmotion_network: Option<String>,
    #[serde(rename = "vcenterPassword", skip_serializing_if = "Option::is_none")]
    pub vcenter_password: Option<String>,
    #[serde(rename = "nsxtPassword", skip_serializing_if = "Option::is_none")]
    pub nsxt_password: Option<String>,
    #[serde(rename = "vcenterCertificateThumbprint", skip_serializing)]
    pub vcenter_certificate_thumbprint: Option<String>,
    #[serde(rename = "nsxtCertificateThumbprint", skip_serializing)]
    pub nsxt_certificate_thumbprint: Option<String>,
}
pub mod private_cloud_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Pending,
        Building,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Internet {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultClusterProperties {
    #[serde(rename = "clusterId", skip_serializing)]
    pub cluster_id: Option<i64>,
    #[serde(rename = "clusterSize", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i64>,
    #[serde(skip_serializing)]
    pub hosts: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(flatten)]
    pub default_cluster_properties: DefaultClusterProperties,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateCloudList {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateCloud>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterList {
    #[serde(skip_serializing)]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminCredentials {
    #[serde(rename = "nsxtUsername", skip_serializing)]
    pub nsxt_username: Option<String>,
    #[serde(rename = "nsxtPassword", skip_serializing)]
    pub nsxt_password: Option<String>,
    #[serde(rename = "vcenterUsername", skip_serializing)]
    pub vcenter_username: Option<String>,
    #[serde(rename = "vcenterPassword", skip_serializing)]
    pub vcenter_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
}
