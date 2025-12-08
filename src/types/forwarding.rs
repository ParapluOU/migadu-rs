use serde::{Deserialize, Serialize};

/// A forwarding address associated with a mailbox.
#[derive(Debug, Clone, Deserialize)]
pub struct Forwarding {
    pub address: String,
    pub blocked_at: Option<String>,
    pub confirmation_sent_at: Option<String>,
    pub confirmed_at: Option<String>,
    pub expires_on: Option<String>,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub is_active: bool,
    pub remove_upon_expiry: Option<bool>,
}

/// Request body for creating a new forwarding address.
#[derive(Debug, Clone, Serialize)]
pub struct CreateForwarding {
    pub address: String,
}

impl CreateForwarding {
    /// Creates a new forwarding creation request.
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
}

/// Request body for updating an existing forwarding address.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateForwarding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_upon_expiry: Option<bool>,
}
