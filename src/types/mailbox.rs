use serde::{Deserialize, Serialize};

/// A mailbox in the Migadu system.
#[derive(Debug, Clone, Deserialize)]
pub struct Mailbox {
    pub local_part: String,
    pub domain_name: String,
    pub address: String,
    pub name: String,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub is_internal: bool,
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
    pub password_recovery_email: Option<String>,
    pub spam_action: Option<String>,
    pub spam_aggressiveness: Option<String>,
    #[serde(default)]
    pub sender_denylist: Vec<String>,
    #[serde(default)]
    pub sender_allowlist: Vec<String>,
    #[serde(default)]
    pub recipient_denylist: Vec<String>,
    pub autorespond_active: Option<bool>,
    pub autorespond_subject: Option<String>,
    pub autorespond_body: Option<String>,
    pub autorespond_expires_on: Option<String>,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub footer_active: bool,
    pub footer_plain_body: Option<String>,
    pub footer_html_body: Option<String>,
    #[serde(default)]
    pub delegations: Vec<String>,
    #[serde(default)]
    pub identities: Vec<String>,
}

/// Request body for creating a new mailbox.
#[derive(Debug, Clone, Serialize)]
pub struct CreateMailbox {
    pub local_part: String,
    pub name: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_recovery_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}

impl CreateMailbox {
    /// Creates a new mailbox creation request with the required fields.
    pub fn new(
        local_part: impl Into<String>,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            local_part: local_part.into(),
            name: name.into(),
            password: password.into(),
            password_recovery_email: None,
            is_internal: None,
        }
    }
}

/// Request body for updating an existing mailbox.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMailbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_recovery_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
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
    pub spam_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_aggressiveness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_denylist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_allowlist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_denylist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorespond_expires_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_plain_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_html_body: Option<String>,
}
