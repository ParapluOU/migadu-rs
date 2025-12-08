use serde::{Deserialize, Serialize};

/// A rewrite rule that matches patterns and forwards to destinations.
#[derive(Debug, Clone, Deserialize)]
pub struct Rewrite {
    pub name: String,
    #[serde(default)]
    pub domain_name: Option<String>,
    pub local_part_rule: String,
    pub order_num: Option<i32>,
    #[serde(default)]
    pub destinations: Vec<String>,
}

/// Request body for creating a new rewrite rule.
#[derive(Debug, Clone, Serialize)]
pub struct CreateRewrite {
    /// A slug identifier for the rewrite rule.
    pub name: String,
    /// Pattern to match (e.g., "support-*" or "sales-*").
    pub local_part_rule: String,
    /// Destinations as a comma-separated string.
    pub destinations: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_num: Option<i32>,
}

impl CreateRewrite {
    /// Creates a new rewrite rule creation request.
    pub fn new(
        name: impl Into<String>,
        local_part_rule: impl Into<String>,
        destinations: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            local_part_rule: local_part_rule.into(),
            destinations: destinations.into(),
            order_num: None,
        }
    }

    /// Creates a new rewrite rule from a list of destination addresses.
    pub fn from_destinations(
        name: impl Into<String>,
        local_part_rule: impl Into<String>,
        destinations: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        let destinations: Vec<_> = destinations.into_iter().map(|s| s.as_ref().to_string()).collect();
        Self {
            name: name.into(),
            local_part_rule: local_part_rule.into(),
            destinations: destinations.join(","),
            order_num: None,
        }
    }
}

/// Request body for updating an existing rewrite rule.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRewrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_part_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_num: Option<i32>,
}
