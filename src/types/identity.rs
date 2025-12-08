use serde::{Deserialize, Serialize};

/// An identity associated with a mailbox.
#[derive(Debug, Clone, Deserialize)]
pub struct Identity {
    pub local_part: String,
    pub domain_name: String,
    pub address: String,
    pub name: String,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub may_send: bool,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub may_receive: bool,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub may_access_imap: bool,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub may_access_pop3: bool,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub may_access_managesieve: bool,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub footer_active: bool,
    pub footer_plain_body: Option<String>,
    pub footer_html_body: Option<String>,
}

/// Request body for creating a new identity.
#[derive(Debug, Clone, Serialize)]
pub struct CreateIdentity {
    pub local_part: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_send: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_receive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_access_imap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_access_pop3: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_access_managesieve: Option<bool>,
}

impl CreateIdentity {
    /// Creates a new identity creation request with the required fields.
    pub fn new(local_part: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            local_part: local_part.into(),
            name: name.into(),
            password: None,
            may_send: None,
            may_receive: None,
            may_access_imap: None,
            may_access_pop3: None,
            may_access_managesieve: None,
        }
    }
}

/// Request body for updating an existing identity.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_send: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_receive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_access_imap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_access_pop3: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_access_managesieve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_plain_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_html_body: Option<String>,
}
