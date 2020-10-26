#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod management_locks {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get_at_resource_group_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        lock_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<ManagementLockObject, get_at_resource_group_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path, subscription_id, resource_group_name, lock_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get_at_resource_group_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get_at_resource_group_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(get_at_resource_group_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_at_resource_group_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(get_at_resource_group_level::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_at_resource_group_level::ResponseBytesError)?;
                get_at_resource_group_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get_at_resource_group_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update_at_resource_group_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        lock_name: &str,
        parameters: &ManagementLockObject,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update_at_resource_group_level::Response, create_or_update_at_resource_group_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path, subscription_id, resource_group_name, lock_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create_or_update_at_resource_group_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder
            .build()
            .context(create_or_update_at_resource_group_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(create_or_update_at_resource_group_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp
                    .bytes()
                    .await
                    .context(create_or_update_at_resource_group_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(create_or_update_at_resource_group_level::DeserializeError { body })?;
                Ok(create_or_update_at_resource_group_level::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp
                    .bytes()
                    .await
                    .context(create_or_update_at_resource_group_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(create_or_update_at_resource_group_level::DeserializeError { body })?;
                Ok(create_or_update_at_resource_group_level::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp
                    .bytes()
                    .await
                    .context(create_or_update_at_resource_group_level::ResponseBytesError)?;
                create_or_update_at_resource_group_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update_at_resource_group_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(ManagementLockObject),
            Created201(ManagementLockObject),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete_at_resource_group_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        lock_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete_at_resource_group_level::Response, delete_at_resource_group_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path, subscription_id, resource_group_name, lock_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(delete_at_resource_group_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete_at_resource_group_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(delete_at_resource_group_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::NO_CONTENT => Ok(delete_at_resource_group_level::Response::NoContent204),
            StatusCode::OK => Ok(delete_at_resource_group_level::Response::Ok200),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete_at_resource_group_level::ResponseBytesError)?;
                delete_at_resource_group_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete_at_resource_group_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            NoContent204,
            Ok200,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update_at_resource_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        resource_provider_namespace: &str,
        parent_resource_path: &str,
        resource_type: &str,
        resource_name: &str,
        lock_name: &str,
        parameters: &ManagementLockObject,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update_at_resource_level::Response, create_or_update_at_resource_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path,
            subscription_id,
            resource_group_name,
            resource_provider_namespace,
            parent_resource_path,
            resource_type,
            resource_name,
            lock_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create_or_update_at_resource_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update_at_resource_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(create_or_update_at_resource_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update_at_resource_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(create_or_update_at_resource_level::DeserializeError { body })?;
                Ok(create_or_update_at_resource_level::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update_at_resource_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(create_or_update_at_resource_level::DeserializeError { body })?;
                Ok(create_or_update_at_resource_level::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update_at_resource_level::ResponseBytesError)?;
                create_or_update_at_resource_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update_at_resource_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(ManagementLockObject),
            Created201(ManagementLockObject),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete_at_resource_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        resource_provider_namespace: &str,
        parent_resource_path: &str,
        resource_type: &str,
        resource_name: &str,
        lock_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete_at_resource_level::Response, delete_at_resource_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path,
            subscription_id,
            resource_group_name,
            resource_provider_namespace,
            parent_resource_path,
            resource_type,
            resource_name,
            lock_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(delete_at_resource_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete_at_resource_level::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete_at_resource_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::NO_CONTENT => Ok(delete_at_resource_level::Response::NoContent204),
            StatusCode::OK => Ok(delete_at_resource_level::Response::Ok200),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete_at_resource_level::ResponseBytesError)?;
                delete_at_resource_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete_at_resource_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            NoContent204,
            Ok200,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        lock_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<ManagementLockObject, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path, subscription_id, lock_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: ManagementLockObject = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update_at_subscription_level(
        operation_config: &crate::OperationConfig,
        lock_name: &str,
        parameters: &ManagementLockObject,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update_at_subscription_level::Response, create_or_update_at_subscription_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path, subscription_id, lock_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create_or_update_at_subscription_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder
            .build()
            .context(create_or_update_at_subscription_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(create_or_update_at_subscription_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp
                    .bytes()
                    .await
                    .context(create_or_update_at_subscription_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(create_or_update_at_subscription_level::DeserializeError { body })?;
                Ok(create_or_update_at_subscription_level::Response::Created201(rsp_value))
            }
            StatusCode::OK => {
                let body: bytes::Bytes = rsp
                    .bytes()
                    .await
                    .context(create_or_update_at_subscription_level::ResponseBytesError)?;
                let rsp_value: ManagementLockObject =
                    serde_json::from_slice(&body).context(create_or_update_at_subscription_level::DeserializeError { body })?;
                Ok(create_or_update_at_subscription_level::Response::Ok200(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp
                    .bytes()
                    .await
                    .context(create_or_update_at_subscription_level::ResponseBytesError)?;
                create_or_update_at_subscription_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update_at_subscription_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(ManagementLockObject),
            Ok200(ManagementLockObject),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete_at_subscription_level(
        operation_config: &crate::OperationConfig,
        lock_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete_at_subscription_level::Response, delete_at_subscription_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/locks/{}",
            &operation_config.base_path, subscription_id, lock_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(delete_at_subscription_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete_at_subscription_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(delete_at_subscription_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::NO_CONTENT => Ok(delete_at_subscription_level::Response::NoContent204),
            StatusCode::OK => Ok(delete_at_subscription_level::Response::Ok200),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete_at_subscription_level::ResponseBytesError)?;
                delete_at_subscription_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete_at_subscription_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            NoContent204,
            Ok200,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_at_resource_group_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        filter: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<ManagementLockListResult, list_at_resource_group_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Authorization/locks",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_at_resource_group_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        let req = req_builder.build().context(list_at_resource_group_level::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(list_at_resource_group_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_at_resource_group_level::ResponseBytesError)?;
                let rsp_value: ManagementLockListResult =
                    serde_json::from_slice(&body).context(list_at_resource_group_level::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_at_resource_group_level::ResponseBytesError)?;
                list_at_resource_group_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_at_resource_group_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_at_resource_level(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        resource_provider_namespace: &str,
        parent_resource_path: &str,
        resource_type: &str,
        resource_name: &str,
        filter: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<ManagementLockListResult, list_at_resource_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/{}/providers/Microsoft.Authorization/locks",
            &operation_config.base_path,
            subscription_id,
            resource_group_name,
            resource_provider_namespace,
            parent_resource_path,
            resource_type,
            resource_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_at_resource_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        let req = req_builder.build().context(list_at_resource_level::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_at_resource_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_at_resource_level::ResponseBytesError)?;
                let rsp_value: ManagementLockListResult =
                    serde_json::from_slice(&body).context(list_at_resource_level::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_at_resource_level::ResponseBytesError)?;
                list_at_resource_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_at_resource_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_at_subscription_level(
        operation_config: &crate::OperationConfig,
        filter: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<ManagementLockListResult, list_at_subscription_level::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/locks",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_at_subscription_level::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        let req = req_builder.build().context(list_at_subscription_level::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_at_subscription_level::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_at_subscription_level::ResponseBytesError)?;
                let rsp_value: ManagementLockListResult =
                    serde_json::from_slice(&body).context(list_at_subscription_level::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_at_subscription_level::ResponseBytesError)?;
                list_at_subscription_level::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_at_subscription_level {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
