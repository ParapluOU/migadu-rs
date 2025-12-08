use serde::{Deserialize, Serialize};

/// An email alias that forwards to one or more destinations.
#[derive(Debug, Clone, Deserialize)]
pub struct Alias {
    pub local_part: String,
    pub domain_name: String,
    pub address: String,
    #[serde(default, deserialize_with = "crate::types::nullable_bool")]
    pub is_internal: bool,
    #[serde(default)]
    pub destinations: Vec<String>,
}

/// Request body for creating a new alias.
#[derive(Debug, Clone, Serialize)]
pub struct CreateAlias {
    pub local_part: String,
    /// Destinations as a comma-separated string (e.g., "one@domain.tld,two@domain.tld").
    pub destinations: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}

impl CreateAlias {
    /// Creates a new alias creation request.
    pub fn new(local_part: impl Into<String>, destinations: impl Into<String>) -> Self {
        Self {
            local_part: local_part.into(),
            destinations: destinations.into(),
            is_internal: None,
        }
    }

    /// Creates a new alias creation request from a list of destination addresses.
    pub fn from_destinations(
        local_part: impl Into<String>,
        destinations: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        let destinations: Vec<_> = destinations.into_iter().map(|s| s.as_ref().to_string()).collect();
        Self {
            local_part: local_part.into(),
            destinations: destinations.join(","),
            is_internal: None,
        }
    }
}

/// Request body for updating an existing alias.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAlias {
    /// Destinations as a comma-separated string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}

impl UpdateAlias {
    /// Creates an update request with new destinations.
    pub fn with_destinations(destinations: impl Into<String>) -> Self {
        Self {
            destinations: Some(destinations.into()),
            is_internal: None,
        }
    }
}
