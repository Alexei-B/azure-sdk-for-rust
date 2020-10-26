#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub properties: ContainerProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    pub image: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
    #[serde(rename = "environmentVariables", skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[serde(rename = "instanceView", skip_serializing)]
    pub instance_view: Option<container_properties::InstanceView>,
    pub resources: ResourceRequirements,
    #[serde(rename = "volumeMounts", skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
}
pub mod container_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct InstanceView {
        #[serde(rename = "restartCount", skip_serializing)]
        pub restart_count: Option<i64>,
        #[serde(rename = "currentState", skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ContainerState>,
        #[serde(rename = "previousState", skip_serializing_if = "Option::is_none")]
        pub previous_state: Option<ContainerState>,
        #[serde(skip_serializing)]
        pub events: Vec<Event>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "finishTime", skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "detailStatus", skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "firstTimestamp", skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[serde(rename = "lastTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub requests: ResourceRequests,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    pub cpu: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLimits {
    #[serde(rename = "memoryInGB", skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileVolume {
    #[serde(rename = "shareName")]
    pub share_name: String,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[serde(rename = "storageAccountKey", skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmptyDirVolume {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretVolume {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepoVolume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    #[serde(rename = "azureFile", skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolume>,
    #[serde(rename = "emptyDir", skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolume>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolume>,
    #[serde(rename = "gitRepo", skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolume>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    pub server: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    pub ports: Vec<Port>,
    #[serde(rename = "type")]
    pub type_: ip_address::Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "dnsNameLabel", skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
    #[serde(skip_serializing)]
    pub fqdn: Option<String>,
}
pub mod ip_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Public,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<port::Protocol>,
    pub port: i32,
}
pub mod port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<container_port::Protocol>,
    pub port: i32,
}
pub mod container_port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: operation::Display,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        User,
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(skip_serializing)]
    pub name: Option<usage::Name>,
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Name {
        #[serde(skip_serializing)]
        pub value: Option<String>,
        #[serde(rename = "localizedValue", skip_serializing)]
        pub localized_value: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerGroup>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "terminalSize", skip_serializing_if = "Option::is_none")]
    pub terminal_size: Option<container_exec_request::TerminalSize>,
}
pub mod container_exec_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TerminalSize {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cols: Option<i64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecResponse {
    #[serde(rename = "webSocketUri", skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
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
