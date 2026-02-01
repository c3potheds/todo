use reqwest::Client;
use serde::Serialize;
use todo_model::{NewOptions, Task, TaskId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("API error: {status}: {message}")]
    ApiError { status: reqwest::StatusCode, message: String },
}

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Serialize)]
struct AddDependencyRequest {
    dep_id: TaskId,
}

pub struct TodoApiClient {
    client: Client,
    base_url: String,
}

impl TodoApiClient {
    pub fn new(base_url: String) -> Self {
        TodoApiClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn create_task(&self, new_task: &NewOptions) -> Result<Task> {
        let response = self
            .client
            .post(&format!("{}/tasks", self.base_url))
            .json(new_task)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>> {
        let response = self
            .client
            .get(&format!("{}/tasks", self.base_url))
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    pub async fn get_task(&self, id: TaskId) -> Result<Task> {
        let response = self
            .client
            .get(&format!("{}/tasks/{}", self.base_url, id))
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    pub async fn update_task(&self, id: TaskId, updated_task: &Task) -> Result<Task> {
        let response = self
            .client
            .put(&format!("{}/tasks/{}", self.base_url, id))
            .json(updated_task)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    pub async fn delete_task(&self, id: TaskId) -> Result<()> {
        let response = self
            .client
            .delete(&format!("{}/tasks/{}", self.base_url, id))
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    pub async fn add_dependency(&self, id: TaskId, dep_id: TaskId) -> Result<Task> {
        let request_body = AddDependencyRequest { dep_id };
        let response = self
            .client
            .post(&format!("{}/tasks/{}/deps", self.base_url, id))
            .json(&request_body)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    pub async fn remove_dependency(&self, id: TaskId, dep_id: TaskId) -> Result<Task> {
        let response = self
            .client
            .delete(&format!(
                "{}/tasks/{}/deps/{}",
                self.base_url, id, dep_id
            ))
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::ApiError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }
}
