#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollectionListResult {
    #[serde(skip_serializing)]
    pub value: Vec<JobCollectionDefinition>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobDefinition>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobHistoryListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobHistoryDefinition>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollectionDefinition {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobCollectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollectionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<job_collection_properties::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<JobCollectionQuota>,
}
pub mod job_collection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        Free,
        P10Premium,
        P20Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollectionQuota {
    #[serde(rename = "maxJobCount", skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i64>,
    #[serde(rename = "maxJobOccurrence", skip_serializing_if = "Option::is_none")]
    pub max_job_occurrence: Option<i64>,
    #[serde(rename = "maxRecurrence", skip_serializing_if = "Option::is_none")]
    pub max_recurrence: Option<JobMaxRecurrence>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinition {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<JobAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<JobRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<JobState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobHistoryDefinition {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobHistoryDefinitionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobHistoryDefinitionProperties {
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing)]
    pub end_time: Option<String>,
    #[serde(rename = "expectedExecutionTime", skip_serializing)]
    pub expected_execution_time: Option<String>,
    #[serde(rename = "actionName", skip_serializing)]
    pub action_name: Option<job_history_definition_properties::ActionName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobExecutionStatus>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "retryCount", skip_serializing)]
    pub retry_count: Option<i64>,
    #[serde(rename = "repeatCount", skip_serializing)]
    pub repeat_count: Option<i64>,
}
pub mod job_history_definition_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionName {
        MainAction,
        ErrorAction,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobAction {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_action::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpRequest>,
    #[serde(rename = "queueMessage", skip_serializing_if = "Option::is_none")]
    pub queue_message: Option<StorageQueueMessage>,
    #[serde(rename = "serviceBusQueueMessage", skip_serializing_if = "Option::is_none")]
    pub service_bus_queue_message: Option<ServiceBusQueueMessage>,
    #[serde(rename = "serviceBusTopicMessage", skip_serializing_if = "Option::is_none")]
    pub service_bus_topic_message: Option<ServiceBusTopicMessage>,
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "errorAction", skip_serializing_if = "Option::is_none")]
    pub error_action: Option<JobErrorAction>,
}
pub mod job_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Http,
        Https,
        StorageQueue,
        ServiceBusQueue,
        ServiceBusTopic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobErrorAction {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_error_action::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpRequest>,
    #[serde(rename = "queueMessage", skip_serializing_if = "Option::is_none")]
    pub queue_message: Option<StorageQueueMessage>,
    #[serde(rename = "serviceBusQueueMessage", skip_serializing_if = "Option::is_none")]
    pub service_bus_queue_message: Option<ServiceBusQueueMessage>,
    #[serde(rename = "serviceBusTopicMessage", skip_serializing_if = "Option::is_none")]
    pub service_bus_topic_message: Option<ServiceBusTopicMessage>,
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
}
pub mod job_error_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Http,
        Https,
        StorageQueue,
        ServiceBusQueue,
        ServiceBusTopic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<HttpAuthentication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientCertAuthentication {
    #[serde(flatten)]
    pub http_authentication: HttpAuthentication,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfx: Option<String>,
    #[serde(rename = "certificateThumbprint", skip_serializing_if = "Option::is_none")]
    pub certificate_thumbprint: Option<String>,
    #[serde(rename = "certificateExpirationDate", skip_serializing_if = "Option::is_none")]
    pub certificate_expiration_date: Option<String>,
    #[serde(rename = "certificateSubjectName", skip_serializing_if = "Option::is_none")]
    pub certificate_subject_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicAuthentication {
    #[serde(flatten)]
    pub http_authentication: HttpAuthentication,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthAuthentication {
    #[serde(flatten)]
    pub http_authentication: HttpAuthentication,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpAuthentication {
    #[serde(rename = "type")]
    pub type_: http_authentication::Type,
}
pub mod http_authentication {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        ClientCertificate,
        ActiveDirectoryOAuth,
        Basic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageQueueMessage {
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[serde(rename = "queueName", skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[serde(rename = "sasToken", skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueMessage {
    #[serde(flatten)]
    pub service_bus_message: ServiceBusMessage,
    #[serde(rename = "queueName", skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicMessage {
    #[serde(flatten)]
    pub service_bus_message: ServiceBusMessage,
    #[serde(rename = "topicPath", skip_serializing_if = "Option::is_none")]
    pub topic_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<ServiceBusAuthentication>,
    #[serde(rename = "brokeredMessageProperties", skip_serializing_if = "Option::is_none")]
    pub brokered_message_properties: Option<ServiceBusBrokeredMessageProperties>,
    #[serde(rename = "customMessageProperties", skip_serializing_if = "Option::is_none")]
    pub custom_message_properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "transportType", skip_serializing_if = "Option::is_none")]
    pub transport_type: Option<service_bus_message::TransportType>,
}
pub mod service_bus_message {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransportType {
        NotSpecified,
        NetMessaging,
        #[serde(rename = "AMQP")]
        Amqp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusAuthentication {
    #[serde(rename = "sasKey", skip_serializing_if = "Option::is_none")]
    pub sas_key: Option<String>,
    #[serde(rename = "sasKeyName", skip_serializing_if = "Option::is_none")]
    pub sas_key_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<service_bus_authentication::Type>,
}
pub mod service_bus_authentication {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        SharedAccessKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusBrokeredMessageProperties {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "forcePersistence", skip_serializing_if = "Option::is_none")]
    pub force_persistence: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(rename = "replyToSessionId", skip_serializing_if = "Option::is_none")]
    pub reply_to_session_id: Option<String>,
    #[serde(rename = "scheduledEnqueueTimeUtc", skip_serializing_if = "Option::is_none")]
    pub scheduled_enqueue_time_utc: Option<String>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "timeToLive", skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "viaPartitionKey", skip_serializing_if = "Option::is_none")]
    pub via_partition_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetryPolicy {
    #[serde(rename = "retryType", skip_serializing_if = "Option::is_none")]
    pub retry_type: Option<retry_policy::RetryType>,
    #[serde(rename = "retryInterval", skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<String>,
    #[serde(rename = "retryCount", skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
}
pub mod retry_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RetryType {
        None,
        Fixed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobMaxRecurrence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<job_max_recurrence::Frequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
}
pub mod job_max_recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        Minute,
        Hour,
        Day,
        Week,
        Month,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobRecurrence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<job_recurrence::Frequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JobRecurrenceSchedule>,
}
pub mod job_recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        Minute,
        Hour,
        Day,
        Week,
        Month,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecurrenceFrequency {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobRecurrenceSchedule {
    #[serde(rename = "weekDays", skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hours: Vec<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub minutes: Vec<i64>,
    #[serde(rename = "monthDays", skip_serializing_if = "Vec::is_empty")]
    pub month_days: Vec<i64>,
    #[serde(rename = "monthlyOccurrences", skip_serializing_if = "Vec::is_empty")]
    pub monthly_occurrences: Vec<JobRecurrenceScheduleMonthlyOccurrence>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobRecurrenceScheduleMonthlyOccurrence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<job_recurrence_schedule_monthly_occurrence::Day>,
    #[serde(rename = "Occurrence", skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<i64>,
}
pub mod job_recurrence_schedule_monthly_occurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStateFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<JobState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobState {
    Enabled,
    Disabled,
    Faulted,
    Completed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobHistoryFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobExecutionStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobExecutionStatus {
    Completed,
    Failed,
    Postponed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatus {
    #[serde(rename = "executionCount", skip_serializing)]
    pub execution_count: Option<i64>,
    #[serde(rename = "failureCount", skip_serializing)]
    pub failure_count: Option<i64>,
    #[serde(rename = "faultedCount", skip_serializing)]
    pub faulted_count: Option<i64>,
    #[serde(rename = "lastExecutionTime", skip_serializing)]
    pub last_execution_time: Option<String>,
    #[serde(rename = "nextExecutionTime", skip_serializing)]
    pub next_execution_time: Option<String>,
}
