use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};

const BASE_URL: &str = "https://api.migadu.com/v1";

/// Client for interacting with the Migadu API.
#[derive(Debug, Clone)]
pub struct MigaduClient {
    http: Client,
    base_url: String,
    email: String,
    api_key: String,
}

impl MigaduClient {
    /// Creates a new Migadu API client.
    ///
    /// # Arguments
    /// * `email` - The email address used for authentication
    /// * `api_key` - The API key for authentication
    pub fn new(email: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
            email: email.into(),
            api_key: api_key.into(),
        }
    }

    /// Creates a new client with a custom base URL (useful for testing).
    pub fn with_base_url(
        email: impl Into<String>,
        api_key: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            email: email.into(),
            api_key: api_key.into(),
        }
    }

    fn auth(&self, req: RequestBuilder) -> RequestBuilder {
        req.basic_auth(&self.email, Some(&self.api_key))
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::Api {
                status: status.as_u16(),
                message,
            });
        }

        let text = response.text().await?;
        serde_json::from_str(&text).map_err(Error::Parse)
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self.auth(self.http.get(self.url(path))).send().await?;
        self.handle_response(response).await
    }

    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .auth(self.http.post(self.url(path)))
            .json(body)
            .send()
            .await?;
        self.handle_response(response).await
    }

    pub(crate) async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .auth(self.http.put(self.url(path)))
            .json(body)
            .send()
            .await?;
        self.handle_response(response).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self.auth(self.http.delete(self.url(path))).send().await?;
        self.handle_response(response).await
    }
}
