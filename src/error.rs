use thiserror::Error;

/// Result type alias using the crate's Error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when interacting with the Migadu API.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        /// HTTP status code returned by the API.
        status: u16,
        /// Error message from the API response.
        message: String,
    },

    /// Failed to parse the API response.
    #[error("Failed to parse response: {0}")]
    Parse(#[source] serde_json::Error),
}
